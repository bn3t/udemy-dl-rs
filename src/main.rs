#![allow(dead_code, unused_imports, unused_variables)]

use std::thread;
use std::time::Duration;

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

#[derive(Serialize, Deserialize, Debug)]
struct Course {
    id: u64,
    url: String,
    published_title: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DownloadUrl {
    r#type: Option<String>,
    file: String,
    label: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Asset {
    filename: String,
    asset_type: String,
    time_estimation: u64,
    download_urls: Option<Vec<DownloadUrl>>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Lecture {
    object_index: u64,
    title: String,
    asset: Asset,
    supplementary_assets: Vec<Asset>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Chapter {
    object_index: u64,
    title: String,
    lectures: Vec<Lecture>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CourseContent {
    chapters: Vec<Chapter>,
}

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
        headers
    }

    pub fn get(&self, url: &str) -> Result<Value, Error> {
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

    fn parse_subscribed_courses(&self, subscribed_courses: &Value) -> Result<Vec<Course>, Error> {
        let results = subscribed_courses
            .get("results")
            .ok_or_else(|| format_err!("Error parsing json"))?
            .as_array()
            .ok_or_else(|| format_err!("Error parsing json"))?;
        let courses: Vec<Course> = results
            .into_iter()
            .map(|result| serde_json::from_value(result.clone()))
            .filter(|course| course.is_ok())
            .map(|course| course.unwrap())
            .collect();
        Ok(courses)
    }

    fn parse_assets(&self, value: &Value) -> Result<Vec<Asset>, Error> {
        let assets = value
            .as_array()
            .ok_or_else(|| format_err!("Error parsing json"))?;

        let assets: Vec<Asset> = assets
            .into_iter()
            .map(|asset| self.parse_asset(asset))
            .filter(|asset| asset.is_ok())
            .map(|asset| asset.unwrap())
            .collect();
        Ok(assets)
    }

    fn parse_asset(&self, asset: &Value) -> Result<Asset, Error> {
        let filename: String = asset
            .get("filename")
            .ok_or_else(|| format_err!("Error parsing json"))?
            .as_str()
            .ok_or_else(|| format_err!("Error parsing json"))?
            .into();
        let asset_type: String = asset
            .get("asset_type")
            .ok_or_else(|| format_err!("Error parsing json"))?
            .as_str()
            .ok_or_else(|| format_err!("Error parsing json"))?
            .into();
        let time_estimation: u64 = asset
            .get("time_estimation")
            .ok_or_else(|| format_err!("Error parsing json"))?
            .as_u64()
            .ok_or_else(|| format_err!("Error parsing json"))?;
        let download_urls = asset
            .get("download_urls")
            .ok_or_else(|| format_err!("Error parsing json"))?;
        let download_urls = if let Some(video) = download_urls.get("Video") {
            Some(video)
        } else if let Some(filee) = download_urls.get("File") {
            Some(filee)
        } else {
            // println!("Unkonwn filetype {:?}", asset);
            None
        };

        let download_urls: Option<Vec<DownloadUrl>> = if let Some(dl_urls) = download_urls {
            Some(serde_json::from_value::<Vec<DownloadUrl>>(dl_urls.clone()).unwrap())
        } else {
            None
        };
        Ok(Asset {
            filename,
            asset_type,
            time_estimation,
            download_urls,
        })
    }

    fn parse_course_content(&self, full_course: &Value) -> Result<CourseContent, Error> {
        let results = full_course
            .get("results")
            .ok_or_else(|| format_err!("Error parsing json"))?
            .as_array()
            .ok_or_else(|| format_err!("Error parsing json"))?;

        let mut chapters: Vec<Chapter> = Vec::new();
        let mut lectures: Vec<Lecture> = Vec::new();
        let mut current_chapter: Option<Chapter> = None;

        for item in results.into_iter() {
            if item.get("_class").unwrap() == "chapter" {
                if current_chapter.is_some() {
                    let mut this_chapter = current_chapter.unwrap();
                    this_chapter.lectures = lectures;
                    chapters.push(this_chapter);
                }
                current_chapter = Some(Chapter {
                    object_index: item
                        .get("object_index")
                        .ok_or_else(|| format_err!("Error parsing json"))?
                        .as_u64()
                        .ok_or_else(|| format_err!("Error parsing json"))?,
                    title: String::from(
                        item.get("title")
                            .ok_or_else(|| format_err!("Error parsing json"))?
                            .as_str()
                            .ok_or_else(|| format_err!("Error parsing json"))?,
                    ),
                    lectures: Vec::new(),
                });
                lectures = Vec::new();
            }
            if item.get("_class").unwrap() == "lecture" {
                let asset = self.parse_asset(
                    item.get("asset")
                        .ok_or_else(|| format_err!("Error parsing json"))?,
                )?;
                let supplementary_assets = self.parse_assets(
                    item.get("supplementary_assets")
                        .ok_or_else(|| format_err!("Error parsing json"))?,
                )?;
                lectures.push(Lecture {
                    object_index: item
                        .get("object_index")
                        .ok_or_else(|| format_err!("Error parsing json"))?
                        .as_u64()
                        .ok_or_else(|| format_err!("Error parsing json"))?,
                    title: String::from(
                        item.get("title")
                            .ok_or_else(|| format_err!("Error parsing json"))?
                            .as_str()
                            .ok_or_else(|| format_err!("Error parsing json"))?,
                    ),
                    asset,
                    supplementary_assets,
                });
            }
        }
        if current_chapter.is_some() {
            let mut this_chapter = current_chapter.unwrap();
            this_chapter.lectures.append(&mut lectures);
            chapters.push(this_chapter);
        }
        Ok(CourseContent { chapters: chapters })
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
        let detail = value.get("detail");
        let course = self
            .parse_subscribed_courses(&value)?
            .into_iter()
            .find(|course| course.published_title == self.course_name)
            .ok_or_else(|| {
                format_err!("{} was not found in subscribed courses", self.course_name)
            })?;

        let url = format!("https://{portal_name}.udemy.com/api-2.0/courses/{course_id}/cached-subscriber-curriculum-items?fields[asset]=results,external_url,time_estimation,download_urls,slide_urls,filename,asset_type,captions,stream_urls,body&fields[chapter]=object_index,title,sort_order&fields[lecture]=id,title,object_index,asset,supplementary_assets,view_html&page_size=10000",
        portal_name = self.portal_name, course_id=course.id);

        let value = self.client.get(url.as_str())?;
        let course_content = self.parse_course_content(&value)?;
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

    let udemy_downloader = UdemyDownloader::new(url, access_token, client_id).unwrap();

    let result: Result<(), Error> = match matches.subcommand() {
        ("info", Some(sub_m)) => {
            println!(
                "Request information from {}",
                matches.value_of("url").unwrap()
            );
            udemy_downloader.info().map(|r| ())
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

mod test_data;

#[cfg(test)]
mod test_udemy_downloader {

    use super::UdemyDownloader;
    use crate::test_data::*;
    use serde_json::{Result as JsonResult, Value};

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

        let actual = dl.parse_subscribed_courses(&subscribed_courses);

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

    #[test]
    fn parse_course_content() {
        let full_course: Value = serde_json::from_str(TEST_FULL_COURSE).unwrap();

        let dl = UdemyDownloader::new(
            "https://www.udemy.com/css-the-complete-guide-incl-flexbox-grid-sass",
            "acctok",
            "clid",
        )
        .unwrap();

        let actual = dl.parse_course_content(&full_course);

        assert_eq!(actual.is_ok(), true);
        let course_content = actual.unwrap();
        assert_eq!(course_content.chapters.len(), 2);

        assert_eq!(course_content.chapters[0].object_index, 1);
        assert_eq!(course_content.chapters[0].title, "Getting Started");
        assert_eq!(
            course_content.chapters[1].title,
            "Diving Into the Basics of CSS"
        );

        assert_eq!(course_content.chapters[0].lectures.len(), 2);
        assert_eq!(course_content.chapters[0].lectures[0].object_index, 1);
        assert_eq!(course_content.chapters[0].lectures[0].title, "Introduction");
        assert_eq!(course_content.chapters[0].lectures[1].title, "What is CSS?");
        assert_eq!(
            course_content.chapters[0].lectures[1]
                .supplementary_assets
                .len(),
            1
        );
        assert_eq!(
            course_content.chapters[0].lectures[1].supplementary_assets[0].asset_type,
            "File"
        );
    }

    #[test]
    fn parse_asset() {
        let asset: Value = serde_json::from_str(TEST_ASSET).unwrap();

        let dl = UdemyDownloader::new(
            "https://www.udemy.com/css-the-complete-guide-incl-flexbox-grid-sass",
            "acctok",
            "clid",
        )
        .unwrap();

        let actual = dl.parse_asset(&asset);

        assert_eq!(actual.is_ok(), true);
        let asset = actual.unwrap();
        assert_eq!(asset.filename, "getting-started-01-welcome.mp4");
        assert_eq!(asset.asset_type, "Video");
        assert_eq!(asset.time_estimation, 99);
        assert_eq!(asset.download_urls.is_some(), true);
        assert_eq!(asset.download_urls.as_ref().unwrap().len(), 4);
        assert_eq!(
            asset.download_urls.as_ref().unwrap()[0].r#type.is_some(),
            true
        );
        assert_eq!(
            asset.download_urls.as_ref().unwrap()[0]
                .r#type
                .as_ref()
                .unwrap(),
            "video/mp4"
        );
        assert_eq!(asset.download_urls.as_ref().unwrap()[0].file, "https://udemy-assets-on-demand2.udemy.com/2018-03-16_18-03-45-cb7a7f9f7ce092310d2ba43b50b0d2b8/WebHD_720p.mp4?nva=20190204223948&filename=getting-started-01-welcome.mp4&download=True&token=068ae457bbe97231de938");
        assert_eq!(asset.download_urls.as_ref().unwrap()[0].label, "720");
        assert_eq!(asset.download_urls.as_ref().unwrap()[1].label, "480");
        assert_eq!(asset.download_urls.as_ref().unwrap()[2].label, "360");
        assert_eq!(asset.download_urls.as_ref().unwrap()[3].label, "144");
    }
    #[test]
    fn parse_assets() {
        let assets: Value = serde_json::from_str(TEST_SUP_ASSETS).unwrap();

        let dl = UdemyDownloader::new(
            "https://www.udemy.com/css-the-complete-guide-incl-flexbox-grid-sass",
            "acctok",
            "clid",
        )
        .unwrap();

        let actual = dl.parse_assets(&assets);

        assert_eq!(actual.is_ok(), true);
    }
}
