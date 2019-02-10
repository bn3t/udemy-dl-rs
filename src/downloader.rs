use std::fs::File;
use std::io::prelude::*;

use failure::{format_err, Error};
use regex::Regex;

use crate::http_client::HttpClient;
use crate::model::*;
use crate::parser::*;
use crate::udemy_helper::*;

const PORTAL_NAME: &str = "www";
const COURSE_SEARCH: &str = "https://{portal_name}.udemy.com/api-2.0/users/me/subscribed-courses?fields[course]=id,url,published_title&page=1&page_size=1000&ordering=-access_time&search={course_name}";

pub struct UdemyDownloader<'a> {
    course_name: String,
    portal_name: String,
    parser: &'a Parser,
    client: &'a HttpClient,
}

type CourseId = u64;

impl<'a> UdemyDownloader<'a> {
    pub fn new(
        url: &str,
        client: &'a HttpClient,
        parser: &'a Parser,
    ) -> Result<UdemyDownloader<'a>, Error> {
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
            parser,
        })
    }

    fn print_course_content(&self, course_content: &CourseContent) {
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
        let value = self.client.get_as_json(url.as_str())?;
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

        let value = self.client.get_as_json(url.as_str())?;
        let course_content = self.parser.parse_course_content(&value)?;
        Ok(course_content)
    }

    fn download_url(&self, url: &str, target_filename: &str) -> Result<(), Error> {
        if let Ok(content_length) = self.client.get_content_length(url) {
            println!("Length: {}", content_length);

            let buf = self.client.get_as_data(url)?;
            let mut file = File::create(target_filename)?;
            let size = file.write(&buf)?;
            println!("{} bytes written", size);
        }
        Ok(())
    }

    fn download_lecture(&self, lecture: &Lecture, path: &str, dry_run: bool) -> Result<(), Error> {
        let target_filename = UdemyHelper::calculate_target_filename(path, &lecture).unwrap();
        if let Some(download_urls) = &lecture.asset.download_urls {
            for url in download_urls {
                if let Some(video_type) = &url.r#type {
                    if url.label == "720" && video_type == "video/mp4" {
                        println!("\tGetting {}", url.file);
                        println!("\t\t-> {}", target_filename);
                        if !dry_run {
                            self.download_url(url.file.as_str(), target_filename.as_str())?
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn info(&self) -> Result<(), Error> {
        let course_content = self.extract()?;
        self.print_course_content(&course_content);
        Ok(())
    }

    /// Download files to a specified location. It is possible to specify
    /// which chapter / lecture to download.
    pub fn download(
        &self,
        wanted_chapter: Option<u64>,
        wanted_lecture: Option<u64>,
        output: &str,
        dry_run: bool,
    ) -> Result<(), Error> {
        println!(
            "Download request chapter: {:?}, lecture: {:?}, dry_run: {}",
            wanted_chapter, wanted_lecture, dry_run
        );
        let course_content = self.extract()?;

        course_content
            .chapters
            .into_iter()
            .for_each(move |chapter| {
                if wanted_chapter.is_none() || wanted_chapter.unwrap() == chapter.object_index {
                    println!(
                        "Downloading chapter {} - {}",
                        chapter.object_index, chapter.title
                    );
                    let chapter_path = UdemyHelper::calculate_target_dir(
                        output,
                        &chapter,
                        self.course_name.as_str(),
                    )
                    .unwrap();
                    if let Ok(_) = UdemyHelper::create_target_dir(chapter_path.as_str()) {
                        chapter
                            .lectures
                            .into_iter()
                            .filter(|lecture| lecture.asset.asset_type == "Video")
                            .filter(|lecture| {
                                wanted_lecture.is_none()
                                    || wanted_lecture.unwrap() == lecture.object_index
                            })
                            .for_each(move |lecture| {
                                match self.download_lecture(
                                    &lecture,
                                    chapter_path.as_str(),
                                    dry_run,
                                ) {
                                    Ok(()) => {
                                        println!("Lecture downloaded");
                                    }
                                    Err(e) => {
                                        println!("Error while saving {}: {}", lecture.title, e);
                                    }
                                };
                            });
                    }
                }
            });
        Ok(())
    }
}

#[cfg(test)]
mod test_udemy_downloader {
    use failure::Error;
    use serde_json::{json, Value};

    use super::UdemyDownloader;
    use crate::http_client::HttpClient;
    use crate::model::*;
    use crate::parser::Parser;

    static mut GETS_AS_JSON: Option<Vec<String>> = None;
    static mut PARSE: Option<Vec<String>> = None;

    struct MockHttpClient {}

    impl HttpClient for MockHttpClient {
        fn get_as_json(&self, url: &str) -> Result<Value, Error> {
            unsafe {
                match GETS_AS_JSON {
                    Some(ref mut gaj) => {
                        gaj.push(String::from(url));
                    }
                    None => panic!(),
                }
            };
            Ok(json!({ "url": url }))
        }
        fn get_content_length(&self, _url: &str) -> Result<u64, Error> {
            Ok(0)
        }
        fn get_as_data(&self, _url: &str) -> Result<Vec<u8>, Error> {
            Ok(vec![])
        }
    }

    struct MockParser {}

    impl MockParser {
        pub fn new() -> MockParser {
            MockParser {}
        }
    }

    impl Parser for MockParser {
        fn parse_subscribed_courses(
            &self,
            subscribed_courses: &Value,
        ) -> Result<Vec<Course>, Error> {
            unsafe {
                match PARSE {
                    Some(ref mut psc) => {
                        psc.push(String::from(format!("{:?}", subscribed_courses)))
                    }
                    None => {
                        panic!();
                    }
                };
            };
            Ok(vec![Course {
                id: 54321,
                url: "the-url".into(),
                published_title: "css-the-complete-guide-incl-flexbox-grid-sass".into(),
            }])
        }
        fn parse_course_content(&self, full_course: &Value) -> Result<CourseContent, Error> {
            unsafe {
                match PARSE {
                    Some(ref mut psc) => psc.push(String::from(format!("{:?}", full_course))),
                    None => {
                        panic!();
                    }
                };
            };
            Ok(CourseContent { chapters: vec![] })
        }
    }

    #[test]
    fn parse_url() {
        let mock_http_client = MockHttpClient {};
        let mock_parser = MockParser::new();
        let dl = UdemyDownloader::new(
            "https://www.udemy.com/css-the-complete-guide-incl-flexbox-grid-sass",
            &mock_http_client,
            &mock_parser,
        )
        .unwrap();

        assert_eq!(
            dl.course_name,
            "css-the-complete-guide-incl-flexbox-grid-sass"
        );
        assert_eq!(dl.portal_name, "www");
    }

    #[test]
    fn info() {
        unsafe {
            PARSE = Some(vec![]);
            GETS_AS_JSON = Some(vec![]);
        }
        let mock_http_client = MockHttpClient {};
        let mock_parser = MockParser::new();
        let dl = UdemyDownloader::new(
            "https://www.udemy.com/css-the-complete-guide-incl-flexbox-grid-sass",
            &mock_http_client,
            &mock_parser,
        )
        .unwrap();

        let result = dl.info();

        assert!(result.is_ok());

        unsafe {
            if let Some(ref gaj) = GETS_AS_JSON {
                assert_eq!(gaj.len(), 2);
                assert_eq!(gaj[0], "https://www.udemy.com/api-2.0/users/me/subscribed-courses?fields[course]=id,url,published_title&page=1&page_size=1000&ordering=-access_time&search=css-the-complete-guide-incl-flexbox-grid-sass");
                assert_eq!(gaj[1], "https://www.udemy.com/api-2.0/courses/54321/cached-subscriber-curriculum-items?fields[asset]=results,external_url,time_estimation,download_urls,slide_urls,filename,asset_type,captions,stream_urls,body&fields[chapter]=object_index,title,sort_order&fields[lecture]=id,title,object_index,asset,supplementary_assets,view_html&page_size=10000");
            }
            if let Some(ref psc) = PARSE {
                assert_eq!(psc.len(), 2);
                assert_eq!(psc[0], "Object({\"url\": String(\"https://www.udemy.com/api-2.0/users/me/subscribed-courses?fields[course]=id,url,published_title&page=1&page_size=1000&ordering=-access_time&search=css-the-complete-guide-incl-flexbox-grid-sass\")})");
                assert_eq!(psc[1], "Object({\"url\": String(\"https://www.udemy.com/api-2.0/courses/54321/cached-subscriber-curriculum-items?fields[asset]=results,external_url,time_estimation,download_urls,slide_urls,filename,asset_type,captions,stream_urls,body&fields[chapter]=object_index,title,sort_order&fields[lecture]=id,title,object_index,asset,supplementary_assets,view_html&page_size=10000\")})");
            }
        }

        //assert!(result.is_ok());
    }
}
