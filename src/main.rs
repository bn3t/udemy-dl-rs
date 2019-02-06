#![allow(dead_code)]

use std::thread;
use std::time::Duration;

use regex::Regex;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, USER_AGENT};
use reqwest::Client;

use serde_json::Value;

use clap::{App, AppSettings, Arg, SubCommand};

use failure::{format_err, Error};

mod model;
mod parser;
mod test_data;

use crate::model::*;
use crate::parser::*;

const DEFAULT_UA: &str = "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.21 (KHTML, like Gecko) Mwendo/1.1.5 Safari/537.21";

const PORTAL_NAME: &str = "www";
const COURSE_SEARCH: &str = "https://{portal_name}.udemy.com/api-2.0/users/me/subscribed-courses?fields[course]=id,url,published_title&page=1&page_size=1000&ordering=-access_time&search={course_name}";

struct UdemyHttpClient {
    access_token: String,
    client_id: String,
    client: Client,
}

trait HttpClient {
    fn get(&self, url: &str) -> Result<Value, Error>;
}

impl HttpClient for UdemyHttpClient {
    fn get(&self, url: &str) -> Result<Value, Error> {
        let mut resp = self
            .client
            .get(url)
            .headers(self.construct_headers())
            .send()?;
        if resp.status().is_success() {
            Ok(resp.json()?)
        } else {
            Err(format_err!("Error while getting from url <{}>", url))
        }
    }
}

impl UdemyHttpClient {
    pub fn new(access_token: &str, client_id: &str) -> UdemyHttpClient {
        let client = Client::new();

        UdemyHttpClient {
            client: client,
            access_token: String::from(access_token),
            client_id: String::from(client_id),
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
        headers
    }
}

struct UdemyDownloader<'a> {
    course_name: String,
    portal_name: String,
    parser: UdemyParser,
    client: &'a HttpClient,
}

type CourseId = u64;

impl<'a> UdemyDownloader<'a> {
    pub fn new(url: &str, client: &'a HttpClient) -> Result<UdemyDownloader<'a>, Error> {
        let re = Regex::new(
            r"(?i)(?://(?P<portal_name>.+?).udemy.com/(?P<course_name>[a-zA-Z0-9_-]+))",
        )?;
        let captures = re
            .captures(url)
            .ok_or_else(|| format_err!("Could not parse provide url <{}>", url))?;
        let course_name = String::from(
            captures
                .name("course_name")
                .ok_or_else(|| format_err!("Could not compute course name out of url <{}>", url))?
                .as_str(),
        );
        let portal_name = String::from(
            captures
                .name("portal_name")
                .ok_or_else(|| format_err!("Could not compute portal name out of url <{}>", url))?
                .as_str(),
        );
        Ok(UdemyDownloader {
            course_name,
            portal_name,
            client,
            parser: UdemyParser::new(),
        })
    }

    fn print_course_content(&self, course_content: &CourseContent) -> () {
        for chapter in course_content.chapters.iter() {
            println!("{:03} Chapter {}", chapter.object_index, chapter.title);
            for lecture in chapter.lectures.iter() {
                println!("\t{:03} Lecture {}", lecture.object_index, lecture.title);
                println!("\t\tFilename {}", lecture.asset.filename);
                println!("\t\tAsset Type {}", lecture.asset.asset_type);
                println!("\t\tTime estimation {}", lecture.asset.time_estimation);
                if let Some(download_urls) = lecture.asset.download_urls.as_ref() {
                    for url in download_urls.iter() {
                        println!("\t\t\tUrl {}", url.file);
                        println!("\t\t\tType {:?}", url.r#type);
                        println!("\t\t\tLabel {}", url.label);
                    }
                }
                for asset in lecture.supplementary_assets.iter() {
                    println!("\t\tSuppl Filename {}", asset.filename);
                    println!("\t\tSuppl Asset Type {}", asset.asset_type);
                    println!("\t\tSuppl Time estimation {}", asset.time_estimation);
                    if let Some(download_urls) = asset.download_urls.as_ref() {
                        for url in download_urls.iter() {
                            println!("\t\t\tUrl {}", url.file);
                            println!("\t\t\tType {:?}", url.r#type);
                            println!("\t\t\tLabel {}", url.label);
                        }
                    }
                }
            }
        }
    }

    fn extract(&self) -> Result<CourseContent, Error> {
        println!("Requesting info");
        let url = format!(
            "https://{portal_name}.udemy.com/api-2.0/users/me/subscribed-courses?fields[course]=id,url,published_title&page=1&page_size=1000&ordering=-access_time&search={course_name}",
            portal_name = self.portal_name,
            course_name = self.course_name
        );
        let value = self.client.get(url.as_str())?;
        let course = self
            .parser
            .parse_subscribed_courses(&value)?
            .into_iter()
            .find(|course| course.published_title == self.course_name)
            .ok_or_else(|| {
                format_err!("{} was not found in subscribed courses", self.course_name)
            })?;

        let url = format!("https://{portal_name}.udemy.com/api-2.0/courses/{course_id}/cached-subscriber-curriculum-items?fields[asset]=results,external_url,time_estimation,download_urls,slide_urls,filename,asset_type,captions,stream_urls,body&fields[chapter]=object_index,title,sort_order&fields[lecture]=id,title,object_index,asset,supplementary_assets,view_html&page_size=10000",
        portal_name = self.portal_name, course_id=course.id);

        let value = self.client.get(url.as_str())?;
        let course_content = self.parser.parse_course_content(&value)?;
        Ok(course_content)
    }

    pub fn info(&self) -> Result<(), Error> {
        let course_content = self.extract()?;
        self.print_course_content(&course_content);
        Ok(())
    }

    pub fn download(
        &self,
        wanted_chapter: Option<u64>,
        wanted_lecture: Option<u64>,
        dry_run: bool,
    ) -> Result<(), Error> {
        println!(
            "Downloadi request chapter: {:?}, lecture: {:?}, dry_run: {}",
            wanted_chapter, wanted_lecture, dry_run
        );
        let course_content = self.extract()?;

        for chapter in course_content.chapters {
            if wanted_chapter.is_none() || wanted_chapter.unwrap() == chapter.object_index {
                println!(
                    "Downloading chapter {} - {}",
                    chapter.object_index, chapter.title
                );
                for lecture in chapter.lectures {
                    if wanted_lecture.is_none() || wanted_lecture.unwrap() == lecture.object_index {
                        println!(
                            "Downloading lecture {} - {}",
                            lecture.object_index, lecture.title
                        );
                        if lecture.asset.asset_type == "Video" {
                            if let Some(download_urls) = lecture.asset.download_urls {
                                for url in download_urls {
                                    if url.label == "720" {
                                        println!("\tGetting {}", url.file);
                                        if !dry_run {
                                            thread::sleep(Duration::from_millis(3000));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

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
        .subcommand(
            SubCommand::with_name("download")
                .about("Download course content")
                .arg(
                    Arg::with_name("dry-run")
                        .short("d")
                        .long("dry-run")
                        .takes_value(false)
                        .help("Dry run, show what's would be done but don't download anything"),
                )
                .arg(
                    Arg::with_name("chapter")
                        .short("c")
                        .long("chapter")
                        .takes_value(true)
                        .value_name("CHAPTER")
                        .help("Restrict downloads to a specific chapter"),
                )
                .arg(
                    Arg::with_name("lecture")
                        .short("l")
                        .long("lecture")
                        .value_name("LECTURE")
                        .takes_value(true)
                        .help("Restrict download to a specific lecture"),
                ),
        )
        .get_matches();

    let url = matches.value_of("url").unwrap();
    let access_token = matches.value_of("access_token").unwrap();
    let client_id = matches.value_of("client_id").unwrap();

    let client = UdemyHttpClient::new(access_token, client_id);
    let udemy_downloader = UdemyDownloader::new(url, &client).unwrap();

    let result: Result<(), Error> = match matches.subcommand() {
        ("info", Some(_sub_m)) => {
            println!(
                "Request information from {}",
                matches.value_of("url").unwrap()
            );
            udemy_downloader.info().map(|_r| ())
        }
        ("download", Some(sub_m)) => {
            println!("Downloading from {}", matches.value_of("url").unwrap());
            let wanted_chapter = sub_m
                .value_of("chapter")
                .map(|v| v.parse::<u64>().ok().unwrap_or(0));
            let wanted_lecture = sub_m
                .value_of("lecture")
                .map(|v| v.parse::<u64>().ok().unwrap_or(0));
            let dry_run = sub_m.is_present("dry-run");

            // Ok(())
            udemy_downloader.download(wanted_chapter, wanted_lecture, dry_run)
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
    use crate::HttpClient;
    use failure::Error;
    use serde_json::{json, Value};

    struct MockHttpClient {}

    impl HttpClient for MockHttpClient {
        fn get(&self, _url: &str) -> Result<Value, Error> {
            Ok(json!({ "an": "object" }))
        }
    }

    #[test]
    fn parse_url() {
        let mock_http_client = MockHttpClient {};
        let dl = UdemyDownloader::new(
            "https://www.udemy.com/css-the-complete-guide-incl-flexbox-grid-sass",
            &mock_http_client,
        )
        .unwrap();

        assert_eq!(
            dl.course_name,
            "css-the-complete-guide-incl-flexbox-grid-sass"
        );
        assert_eq!(dl.portal_name, "www");
    }

}
