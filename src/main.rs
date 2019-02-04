#![allow(dead_code, unused_imports, unused_variables)]

use regex::Regex;

use reqwest::header::{
    HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, COOKIE, HOST, REFERER, USER_AGENT,
};
use reqwest::{Client, Response, StatusCode};

use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use clap::{App, AppSettings, Arg, SubCommand};

const DEFAULT_UA: &str = "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.21 (KHTML, like Gecko) Mwendo/1.1.5 Safari/537.21";

const PORTAL_NAME: &str = "www";
const COURSE_SEARCH: &str = "https://{portal_name}.udemy.com/api-2.0/users/me/subscribed-courses?fields[course]=id,url,published_title&page=1&page_size=1000&ordering=-access_time&search={course_name}";

/*
{
          "_class": "course",
          "id": 1561458,
          "url": "/css-the-complete-guide-incl-flexbox-grid-sass/learn/v4/",
          "published_title": "css-the-complete-guide-incl-flexbox-grid-sass"
        }
        */

#[derive(Serialize, Deserialize, Debug)]
struct Course {
    id: u64,
    url: String,
    published_title: String,
}

/*
    def _set_auth_headers(self, access_token='', client_id=''):
        self._headers['Authorization'] = "Bearer {}".format(access_token)
        self._headers['X-Udemy-Authorization'] = "Bearer {}".format(access_token)

*/
struct UdemyHttpClient {
    access_token: String,
    client_id: String,
    _host: String,
    _referer: String,
    client: Client,
}

use failure::{format_err, Error, Fail};

impl UdemyHttpClient {
    pub fn new(access_token: &str, client_id: &str, host: &str, referer: &str) -> UdemyHttpClient {
        let client = Client::new();

        UdemyHttpClient {
            client: client,
            access_token: String::from(access_token),
            client_id: String::from(client_id),
            _host: String::from(host),
            _referer: String::from(referer),
        }
    }

    fn construct_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let bearer = format!("Bearer {}", self.access_token);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(bearer.as_str()).unwrap(),
        );
        headers.insert(
            HeaderName::from_lowercase(b"x-udemy-authorization").unwrap(),
            HeaderValue::from_str(bearer.as_str()).unwrap(),
        );
        headers.insert(USER_AGENT, HeaderValue::from_str(DEFAULT_UA).unwrap());
        // headers.insert(
        //     REFERER,
        //     HeaderValue::from_str(self.referer.as_str()).unwrap(),
        // );
        // headers.insert(HOST, HeaderValue::from_str(self.host.as_str()).unwrap());
        headers
    }

    pub fn get(&self, url: &str) -> Result<Value, Error> {
        //Err(format_err!("Error doing {}", "test"))
        //self.client.get("url")
        println!("get url={}", url);
        let mut resp = self
            .client
            .get(url)
            .headers(self.construct_headers())
            .send()?;
        if resp.status().is_success() {
            println!("Success {}", resp.status().as_str());
            Ok(resp.json()?)
        } else {
            println!("Error {}", resp.status().as_str());
            Err(format_err!("Error while getting from url <{}>", url))
        }
    }
}

struct UdemyDownloader {
    course_name: String,
    portal_name: String,

    client: UdemyHttpClient,
}

type CourseId = u64;

impl UdemyDownloader {
    pub fn new(url: &str, access_token: &str, client_id: &str) -> Result<UdemyDownloader, Error> {
        let re = Regex::new(
            r"(?i)(?://(?P<portal_name>.+?).udemy.com/(?P<course_name>[a-zA-Z0-9_-]+))",
        )?;
        let captures = re
            .captures(url)
            .ok_or(format_err!("Could not parse provide url <{}>", url))?;
        let course_name = String::from(
            captures
                .name("course_name")
                .ok_or(format_err!(
                    "Could not compute course name out of url <{}>",
                    url
                ))?
                .as_str(),
        );
        let portal_name = String::from(
            captures
                .name("portal_name")
                .ok_or(format_err!(
                    "Could not compute portal name out of url <{}>",
                    url
                ))?
                .as_str(),
        );
        let host = format!("{portal_name}.udemy.com", portal_name = portal_name);
        let referer = format!(
            "https://{portal_name}.udemy.com/home/my-courses/search/?q={course_name}",
            portal_name = portal_name,
            course_name = course_name
        );
        Ok(UdemyDownloader {
            course_name,
            portal_name,
            client: UdemyHttpClient::new(access_token, client_id, host.as_str(), referer.as_str()),
        })
    }

    fn parse_subscribed_courses(&self, subscribed_courses: Value) -> Result<Vec<Course>, Error> {
        let results = subscribed_courses
            .get("results")
            .ok_or(format_err!("Error parsing json"))?
            .as_array()
            .ok_or(format_err!("Error parsing json"))?;
        println!("results={:?}", results);
        let courses: Vec<Course> = results
            .into_iter()
            .map(|result| serde_json::from_value(result.clone()))
            .filter(|course| course.is_ok())
            .map(|course| course.unwrap())
            .collect();
        Ok(courses)
    }

    pub fn info(&self) -> Result<String, Error> {
        println!("Requesting info");
        let url = format!(
            "https://{portal_name}.udemy.com/api-2.0/users/me/subscribed-courses?fields[course]=id,url,published_title&page=1&page_size=1000&ordering=-access_time&search={course_name}",
            portal_name = self.portal_name,
            course_name = self.course_name
        );
        let value = self.client.get(url.as_str())?;
        let course = self
            .parse_subscribed_courses(value)?
            .into_iter()
            .find(|course| course.published_title == self.course_name)
            .ok_or(format_err!(
                "{} was not found in subscribed courses",
                self.course_name
            ))?;

        Ok(String::from("Info"))
    }
}

trait HttpClient {}

fn main() {
    let matches = App::new("Udemy Downloader")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Bernard Niset")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("url")
                .short("u")
                .long("url")
                .value_name("URL")
                .help("URL of the course to download")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("access_token")
                .short("t")
                .long("access-token")
                .value_name("TOKEN")
                .help("Access token to authenticate to udemy")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("client_id")
                .short("c")
                .long("client-id")
                .value_name("CLIENT_ID")
                .help("Client id to authenticate to udemy")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(SubCommand::with_name("info").about("Query course information"))
        .get_matches();

    let url = matches.value_of("url").unwrap();
    let access_token = matches.value_of("access_token").unwrap();
    let client_id = matches.value_of("client_id").unwrap();

    let udemy_downloader = UdemyDownloader::new(url, access_token, client_id).unwrap();

    let result: Result<(), Error> = match matches.subcommand() {
        ("info", Some(sub_m)) => {
            println!(
                "Request information from {}",
                matches.value_of("url").unwrap()
            );
            udemy_downloader.info().map(|r| ())
        }
        _ => Ok(()),
    };

    if let Err(err) = result {
        eprintln!("An error Occured: {}", err);
    }
}

#[cfg(test)]
mod test_udemy_downloader {
    use super::UdemyDownloader;
    use serde_json::{Result as JsonResult, Value};

    const TEST_SUBSCRIBED_COURSES: &str = r#"{
      "count": 13,
      "next": null,
      "previous": null,
      "results": [
        {
          "_class": "course",
          "id": 1561458,
          "url": "/css-the-complete-guide-incl-flexbox-grid-sass/learn/v4/",
          "published_title": "css-the-complete-guide-incl-flexbox-grid-sass"
        },
        {
          "_class": "course",
          "id": 995016,
          "url": "/vuejs-2-the-complete-guide/learn/v4/",
          "published_title": "vuejs-2-the-complete-guide"
        },
        {
          "_class": "course",
          "id": 1362070,
          "url": "/react-the-complete-guide-incl-redux/learn/v4/",
          "published_title": "react-the-complete-guide-incl-redux"
        }
      ],
      "aggregations": null
    }"#;

    #[test]
    fn parse_url() {
        let dl = UdemyDownloader::new(
            "https://www.udemy.com/css-the-complete-guide-incl-flexbox-grid-sass",
            "acctok",
            "clid",
        )
        .unwrap();

        assert_eq!(
            dl.course_name,
            "css-the-complete-guide-incl-flexbox-grid-sass"
        );
        assert_eq!(dl.portal_name, "www");
    }

    #[test]
    fn parse_subscribed_courses() {
        let subscribed_courses: Value = serde_json::from_str(TEST_SUBSCRIBED_COURSES).unwrap();

        let dl = UdemyDownloader::new(
            "https://www.udemy.com/css-the-complete-guide-incl-flexbox-grid-sass",
            "acctok",
            "clid",
        )
        .unwrap();

        let actual = dl.parse_subscribed_courses(subscribed_courses);

        assert_eq!(actual.is_ok(), true);
        assert_eq!(
            actual
                .unwrap()
                .into_iter()
                .map(|course| course.id)
                .collect::<Vec<u64>>(),
            vec!(1561458, 995016, 1362070)
        )
    }
}
