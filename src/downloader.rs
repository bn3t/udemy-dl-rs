#![allow(clippy::too_many_arguments)]
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

use failure::{format_err, Error};
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;

use crate::http_client::HttpClient;
use crate::model::*;
use crate::parser::*;
use crate::udemy_helper::*;
use crate::utils::*;

const PORTAL_NAME: &str = "www";
const COURSE_SEARCH: &str = "https://{portal_name}.udemy.com/api-2.0/users/me/subscribed-courses?fields[course]=id,url,published_title&page=1&page_size=1000&ordering=-access_time&search={course_name}";
const LOGIN_URL: &str =
    "https://www.udemy.com/api-2.0/auth/udemy-auth/login/?fields[user]=access_token";

pub struct UdemyDownloader<'a> {
    course_name: String,
    portal_name: String,
    auth: Auth,
    parser: &'a Parser,
    client: &'a HttpClient,
    udemy_helper: &'a UdemyHelper<'a>,
}

type CourseId = u64;

impl<'a> UdemyDownloader<'a> {
    pub fn new(
        url: &str,
        client: &'a HttpClient,
        parser: &'a Parser,
        udemy_helper: &'a UdemyHelper,
        auth: Auth,
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
            udemy_helper,
            auth,
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

    fn get_subscribed_course(&self, verbose: bool) -> Result<Course, Error> {
        if verbose {
            println!("Requesting subscribed courses");
        }
        let url = format!(
            "https://{portal_name}.udemy.com/api-2.0/users/me/subscribed-courses?fields[course]=id,url,published_title&page=1&page_size=1000&ordering=-access_time&search={course_name}",
            portal_name = self.portal_name,
            course_name = self.course_name
        );
        let value = self.client.get_as_json(url.as_str(), &self.auth)?;
        self.parser
            .parse_subscribed_courses(&value)?
            .into_iter()
            .find(|course| course.published_title == self.course_name)
            .ok_or_else(|| format_err!("{} was not found in subscribed courses", self.course_name))
    }

    fn get_info(&self, course: &Course, verbose: bool) -> Result<String, Error> {
        let url = format!("https://{portal_name}.udemy.com/api-2.0/courses/{course_id}/cached-subscriber-curriculum-items?fields[asset]=results,external_url,time_estimation,download_urls,slide_urls,filename,asset_type,captions,stream_urls,body&fields[chapter]=object_index,title,sort_order&fields[lecture]=id,title,object_index,asset,supplementary_assets,view_html&page_size=10000",
        portal_name = self.portal_name, course_id=course.id);

        if verbose {
            println!("Requesting info for course");
        }
        self.client.get_as_text(url.as_str(), &self.auth)
    }

    fn parse_info(&self, info: &str) -> Result<CourseContent, Error> {
        let value = serde_json::from_str(info)?;
        let course_content = self.parser.parse_course_content(&value)?;
        Ok(course_content)
    }

    fn download_url(
        &self,
        lecture_title: &str,
        url: &str,
        target_filename: &str,
    ) -> Result<(), Error> {
        let content_length = self.client.get_content_length(url)?;
        let start = Instant::now();

        let pb = ProgressBar::new(content_length);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta}) ({msg})")
                .progress_chars("#>-"),
        );
        pb.set_message(lecture_title);
        let buf = self.client.get_as_data(url, &mut |size| {
            pb.set_position(size);
        })?;
        let mut file = File::create(target_filename)?;
        let _size = file.write(&buf)?;
        let elapsed = Instant::now().duration_since(start);
        let elapsed = elapsed.as_secs() * 1000u64 + u64::from(elapsed.subsec_millis());
        pb.finish_with_message(
            format!(
                "{:1.2} MB/s",
                calculate_download_speed(content_length, elapsed)
            )
            .as_str(),
        );
        Ok(())
    }

    fn determine_quality(
        &self,
        download_urls: &[DownloadUrl],
        wanted_quality: Option<u64>,
    ) -> Result<String, Error> {
        let quality = match wanted_quality {
            Some(quality) => download_urls
                .iter()
                .filter(|url| url.r#type == Some("video/mp4".into()))
                .map(|url| &url.label)
                .filter_map(|label| label.parse::<u64>().ok())
                .filter(|label| *label >= quality)
                .min(),
            None => download_urls
                .iter()
                .filter(|url| url.r#type == Some("video/mp4".into()))
                .map(|url| &url.label)
                .filter_map(|label| label.parse::<u64>().ok())
                .max(),
        };
        let quality = quality
            .map(|q| q.to_string())
            .ok_or_else(|| format_err!("No best quality could be found"))?;
        Ok(quality)
    }

    fn complete_chapter(
        &self,
        course_id: u64,
        chapter: &Chapter,
        wanted_lecture: Option<u64>,
        verbose: bool,
    ) -> Result<(), Error> {
        if verbose {
            println!(
                "Completing chapter {} - {}",
                chapter.object_index, chapter.title
            );
        }
        chapter
            .lectures
            .iter()
            .filter(|lecture| {
                wanted_lecture.is_none() || wanted_lecture.unwrap() == lecture.object_index
            })
            .for_each(move |lecture| {
                match self.complete_lecture(course_id, &lecture, verbose) {
                    Ok(_) => {
                        // if verbose {
                        //     println!("Lecture downloaded");
                        // }
                    }
                    Err(e) => {
                        eprintln!("Error while completing {}: {}", lecture.title, e);
                    }
                };
            });
        Ok(())
    }

    fn complete_lecture(
        &self,
        course_id: u64,
        lecture: &Lecture,
        verbose: bool,
    ) -> Result<(), Error> {
        if verbose {
            println!("Completing lecture {}", lecture.title);
        }
        let url = format!(
            "https://{portal_name}.udemy.com/api-2.0/users/me/subscribed-courses/{course_id}/completed-lectures/",
            portal_name = self.portal_name,
            course_id = course_id
        );
        let complete_request = CompleteRequest {
            lecture_id: lecture.id,
            downloaded: false,
        };
        let value = serde_json::to_value(complete_request)?;
        self.client.post_json(url.as_str(), &value, &self.auth)?;
        Ok(())
    }

    fn download_chapter(
        &self,
        chapter: &Chapter,
        wanted_lecture: Option<u64>,
        wanted_quality: Option<u64>,
        output: &str,
        dry_run: bool,
        verbose: bool,
    ) -> Result<(), Error> {
        if verbose {
            println!(
                "Downloading chapter {} - {}",
                chapter.object_index, chapter.title
            );
        }
        let chapter_path = self
            .udemy_helper
            .calculate_target_dir(output, &chapter, self.course_name.as_str())
            .unwrap();
        if self
            .udemy_helper
            .create_target_dir(chapter_path.as_str())
            .is_ok()
        {
            chapter
                .lectures
                .iter()
                .filter(|lecture| lecture.asset.asset_type == "Video")
                .filter(|lecture| {
                    wanted_lecture.is_none() || wanted_lecture.unwrap() == lecture.object_index
                })
                .for_each(move |lecture| {
                    match self.download_lecture(
                        &lecture,
                        wanted_quality,
                        chapter_path.as_str(),
                        dry_run,
                        verbose,
                    ) {
                        Ok(_) => {
                            // if verbose {
                            //     println!("Lecture downloaded");
                            // }
                        }
                        Err(e) => {
                            eprintln!("Error while saving {}: {}", lecture.title, e);
                        }
                    };
                });
        }
        Ok(())
    }

    fn download_lecture(
        &self,
        lecture: &Lecture,
        wanted_quality: Option<u64>,
        path: &str,
        dry_run: bool,
        verbose: bool,
    ) -> Result<(), Error> {
        let target_filename = self
            .udemy_helper
            .calculate_target_filename(path, &lecture)
            .unwrap();
        if let Some(download_urls) = &lecture.asset.download_urls {
            let best_quality = self.determine_quality(&download_urls, wanted_quality)?;
            for url in download_urls {
                if let Some(video_type) = &url.r#type {
                    if url.label == best_quality && video_type == "video/mp4" {
                        if verbose {
                            println!("\tGetting ({}) {}", url.label, url.file);
                            println!("\t\t-> {}", target_filename);
                        }
                        if !dry_run {
                            self.download_url(
                                lecture.title.as_str(),
                                url.file.as_str(),
                                target_filename.as_str(),
                            )?
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn authenticate(&mut self) -> Result<(), Error> {
        if self.auth.access_token.is_none() {
            let access_token = self.client.post_login_form(LOGIN_URL, &self.auth)?;
            self.auth.access_token = Some(access_token);
        }
        Ok(())
    }

    pub fn info(&self, verbose: bool, wanted_save: Option<&str>) -> Result<(), Error> {
        let course = self.get_subscribed_course(verbose)?;
        let info = self.get_info(&course, verbose)?;
        if let Some(filename) = wanted_save {
            save_to_file(filename, info.as_str())?;
            println!("Course info saved to <{}>", filename);
        } else {
            let course_content = self.parse_info(info.as_str())?;
            self.print_course_content(&course_content);
        }

        Ok(())
    }

    /// Download files to a specified location. It is possible to specify
    /// which chapter / lecture to download.
    pub fn download(
        &self,
        wanted_chapter: Option<u64>,
        wanted_lecture: Option<u64>,
        wanted_quality: Option<u64>,
        wanted_info: Option<&str>,
        output: &str,
        dry_run: bool,
        verbose: bool,
    ) -> Result<(), Error> {
        if verbose {
            println!(
                "Download request chapter: {:?}, lecture: {:?}, quality: {:?}, dry_run: {}",
                wanted_chapter, wanted_lecture, wanted_quality, dry_run
            );
        }
        let info = match wanted_info {
            Some(filename) => load_from_file(filename)?,
            None => self
                .get_subscribed_course(verbose)
                .and_then(|course| self.get_info(&course, verbose))?,
        };
        let course_content = self.parse_info(info.as_str())?;

        for chapter in course_content.chapters.iter() {
            if wanted_chapter.is_none() || wanted_chapter.unwrap() == chapter.object_index {
                self.download_chapter(
                    &chapter,
                    wanted_lecture,
                    wanted_quality,
                    output,
                    dry_run,
                    verbose,
                )?;
            }
        }

        Ok(())
    }

    /// Complete chapters and lectures.
    pub fn complete(
        &self,
        wanted_chapter: u64,
        wanted_lecture: Option<u64>,
        verbose: bool,
    ) -> Result<(), Error> {
        if verbose {
            println!(
                "Complete chapter: {}, lecture: {:?}",
                wanted_chapter, wanted_lecture
            );
        }
        let course = self.get_subscribed_course(verbose)?;
        let info = self.get_info(&course, verbose)?;
        let course_content = self.parse_info(info.as_str())?;

        for chapter in course_content.chapters.iter() {
            if wanted_chapter == chapter.object_index {
                self.complete_chapter(course.id, &chapter, wanted_lecture, verbose)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test_udemy_downloader {
    use failure::Error;
    use serde_json::Value;

    use super::UdemyDownloader;
    use crate::fs_helper::FsHelper;
    use crate::http_client::HttpClient;
    use crate::model::*;
    use crate::parser::Parser;
    use crate::udemy_helper::UdemyHelper;

    static mut GETS_AS_JSON: Option<Vec<String>> = None;
    static mut GETS_CONTENT_LENGTH: Option<Vec<String>> = None;
    static mut GETS_AS_DATA: Option<Vec<String>> = None;
    static mut PARSE: Option<Vec<String>> = None;

    struct MockHttpClient {}

    impl HttpClient for MockHttpClient {
        fn get_as_text(&self, url: &str, _auth: &Auth) -> Result<String, Error> {
            println!("get_as_text url={}", url);
            unsafe {
                match GETS_AS_JSON {
                    Some(ref mut gaj) => {
                        gaj.push(String::from(url));
                    }
                    None => panic!(),
                }
            };
            Ok(format!(r#"{{ "url": "{}" }}"#, url))
        }
        fn get_content_length(&self, url: &str) -> Result<u64, Error> {
            println!("get_content_length url={}", url);
            unsafe {
                match GETS_CONTENT_LENGTH {
                    Some(ref mut gcl) => {
                        gcl.push(String::from(url));
                    }
                    None => panic!(),
                }
            };
            Ok(321)
        }
        fn get_as_data(&self, url: &str, _f: &mut FnMut(u64)) -> Result<Vec<u8>, Error> {
            println!("get_as_data url={}", url);
            unsafe {
                match GETS_AS_DATA {
                    Some(ref mut gad) => {
                        gad.push(String::from(url));
                    }
                    None => panic!(),
                }
            };
            Ok(vec![])
        }
        fn post_login_form(&self, _url: &str, _auth: &Auth) -> Result<String, Error> {
            Ok("blah".into())
        }
        fn post_json(&self, _url: &str, _json: &Value, _auth: &Auth) -> Result<(), Error> {
            Ok(())
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
            Ok(CourseContent {
                chapters: vec![Chapter {
                    object_index: 1,
                    title: "The Chapter".into(),
                    lectures: vec![Lecture {
                        id: 4321,
                        object_index: 1,
                        title: "The Lecture".into(),
                        asset: Asset {
                            filename: "the-filename.mp4".into(),
                            asset_type: "Video".into(),
                            time_estimation: 321,
                            download_urls: Some(vec![DownloadUrl {
                                r#type: Some("video/mp4".into()),
                                file: "http://host-name/the-filename.mp4".into(),
                                label: "720".into(),
                            }]),
                        },
                        supplementary_assets: vec![],
                    }],
                }],
            })
        }
    }

    struct MockFsHelper {}

    impl FsHelper for MockFsHelper {
        fn create_dir_recursive(&self, _path: &str) -> Result<(), Error> {
            Ok(())
        }
    }

    #[test]
    fn parse_url() {
        unsafe {
            PARSE = Some(vec![]);
            GETS_AS_JSON = Some(vec![]);
            GETS_CONTENT_LENGTH = Some(vec![]);
            GETS_AS_DATA = Some(vec![]);
        }

        let fs_helper = MockFsHelper {};

        let mock_http_client = MockHttpClient {};
        let mock_parser = MockParser::new();
        let udemy_helper = UdemyHelper::new(&fs_helper);
        let auth = Auth::with_token("blah");
        let dl = UdemyDownloader::new(
            "https://www.udemy.com/css-the-complete-guide-incl-flexbox-grid-sass",
            &mock_http_client,
            &mock_parser,
            &udemy_helper,
            auth,
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
            GETS_CONTENT_LENGTH = Some(vec![]);
            GETS_AS_DATA = Some(vec![]);
        }

        let fs_helper = MockFsHelper {};

        let mock_http_client = MockHttpClient {};
        let mock_parser = MockParser::new();
        let udemy_helper = UdemyHelper::new(&fs_helper);
        let auth = Auth::with_token("blah");
        let dl = UdemyDownloader::new(
            "https://www.udemy.com/css-the-complete-guide-incl-flexbox-grid-sass",
            &mock_http_client,
            &mock_parser,
            &udemy_helper,
            auth,
        )
        .unwrap();

        let result = dl.info(true, None);

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
    }

    #[test]
    fn download() {
        unsafe {
            PARSE = Some(vec![]);
            GETS_AS_JSON = Some(vec![]);
            GETS_CONTENT_LENGTH = Some(vec![]);
            GETS_AS_DATA = Some(vec![]);
        }

        let fs_helper = MockFsHelper {};

        let mock_http_client = MockHttpClient {};
        let mock_parser = MockParser::new();
        let udemy_helper = UdemyHelper::new(&fs_helper);
        let auth = Auth::with_token("blah");
        let dl = UdemyDownloader::new(
            "https://www.udemy.com/css-the-complete-guide-incl-flexbox-grid-sass",
            &mock_http_client,
            &mock_parser,
            &udemy_helper,
            auth,
        )
        .unwrap();

        let result = dl.download(Some(1), Some(1), None, None, "~/Downloads", false, false);

        assert!(result.is_ok());

        unsafe {
            if let Some(ref gaj) = GETS_AS_JSON {
                assert_eq!(gaj.len(), 2);
                assert_eq!(gaj[0], "https://www.udemy.com/api-2.0/users/me/subscribed-courses?fields[course]=id,url,published_title&page=1&page_size=1000&ordering=-access_time&search=css-the-complete-guide-incl-flexbox-grid-sass");
                assert_eq!(gaj[1], "https://www.udemy.com/api-2.0/courses/54321/cached-subscriber-curriculum-items?fields[asset]=results,external_url,time_estimation,download_urls,slide_urls,filename,asset_type,captions,stream_urls,body&fields[chapter]=object_index,title,sort_order&fields[lecture]=id,title,object_index,asset,supplementary_assets,view_html&page_size=10000");
            }
            if let Some(ref gcl) = GETS_CONTENT_LENGTH {
                assert_eq!(gcl.len(), 1);
                assert_eq!(gcl[0], "http://host-name/the-filename.mp4");
            }
            if let Some(ref gad) = GETS_AS_DATA {
                assert_eq!(gad.len(), 1);
                assert_eq!(gad[0], "http://host-name/the-filename.mp4");
            }
            if let Some(ref psc) = PARSE {
                assert_eq!(psc.len(), 2);
                assert_eq!(psc[0], "Object({\"url\": String(\"https://www.udemy.com/api-2.0/users/me/subscribed-courses?fields[course]=id,url,published_title&page=1&page_size=1000&ordering=-access_time&search=css-the-complete-guide-incl-flexbox-grid-sass\")})");
                assert_eq!(psc[1], "Object({\"url\": String(\"https://www.udemy.com/api-2.0/courses/54321/cached-subscriber-curriculum-items?fields[asset]=results,external_url,time_estimation,download_urls,slide_urls,filename,asset_type,captions,stream_urls,body&fields[chapter]=object_index,title,sort_order&fields[lecture]=id,title,object_index,asset,supplementary_assets,view_html&page_size=10000\")})");
            }
        }
    }

    #[test]
    fn determine_quality_for_best() {
        let fs_helper = MockFsHelper {};

        let mock_http_client = MockHttpClient {};
        let mock_parser = MockParser::new();
        let udemy_helper = UdemyHelper::new(&fs_helper);
        let auth = Auth::with_token("blah");
        let dl = UdemyDownloader::new(
            "https://www.udemy.com/css-the-complete-guide-incl-flexbox-grid-sass",
            &mock_http_client,
            &mock_parser,
            &udemy_helper,
            auth,
        )
        .unwrap();

        let download_urls = vec![
            DownloadUrl {
                label: "480".into(),
                file: "the-file-video-480".into(),
                r#type: Some("video/mp4".into()),
            },
            DownloadUrl {
                label: "720".into(),
                file: "the-file-video-720".into(),
                r#type: Some("video/mp4".into()),
            },
            DownloadUrl {
                label: "1720".into(),
                file: "the-file-720".into(),
                r#type: Some("other/mp4".into()),
            },
        ];
        let wanted_quality = None;

        let actual = dl.determine_quality(&download_urls, wanted_quality);

        assert_eq!(actual.is_ok(), true);
        assert_eq!(actual.unwrap(), "720");
    }

    #[test]
    fn determine_quality_for_wanted_480() {
        let fs_helper = MockFsHelper {};

        let mock_http_client = MockHttpClient {};
        let mock_parser = MockParser::new();
        let udemy_helper = UdemyHelper::new(&fs_helper);
        let auth = Auth::with_token("blah");
        let dl = UdemyDownloader::new(
            "https://www.udemy.com/css-the-complete-guide-incl-flexbox-grid-sass",
            &mock_http_client,
            &mock_parser,
            &udemy_helper,
            auth,
        )
        .unwrap();

        let download_urls = vec![
            DownloadUrl {
                label: "480".into(),
                file: "the-file-video-480".into(),
                r#type: Some("video/mp4".into()),
            },
            DownloadUrl {
                label: "360".into(),
                file: "the-file-video-360".into(),
                r#type: Some("video/mp4".into()),
            },
            DownloadUrl {
                label: "720".into(),
                file: "the-file-video-720".into(),
                r#type: Some("video/mp4".into()),
            },
            DownloadUrl {
                label: "1720".into(),
                file: "the-file-720".into(),
                r#type: Some("other/mp4".into()),
            },
        ];
        let wanted_quality = Some(480u64);

        let actual = dl.determine_quality(&download_urls, wanted_quality);

        assert_eq!(actual.is_ok(), true);
        assert_eq!(actual.unwrap(), "480");
    }

    #[test]
    fn determine_quality_for_wanted_470() {
        let fs_helper = MockFsHelper {};

        let mock_http_client = MockHttpClient {};
        let mock_parser = MockParser::new();
        let udemy_helper = UdemyHelper::new(&fs_helper);
        let auth = Auth::with_token("blah");
        let dl = UdemyDownloader::new(
            "https://www.udemy.com/css-the-complete-guide-incl-flexbox-grid-sass",
            &mock_http_client,
            &mock_parser,
            &udemy_helper,
            auth,
        )
        .unwrap();

        let download_urls = vec![
            DownloadUrl {
                label: "480".into(),
                file: "the-file-video-480".into(),
                r#type: Some("video/mp4".into()),
            },
            DownloadUrl {
                label: "360".into(),
                file: "the-file-video-360".into(),
                r#type: Some("video/mp4".into()),
            },
            DownloadUrl {
                label: "720".into(),
                file: "the-file-video-720".into(),
                r#type: Some("video/mp4".into()),
            },
            DownloadUrl {
                label: "1720".into(),
                file: "the-file-720".into(),
                r#type: Some("other/mp4".into()),
            },
        ];
        let wanted_quality = Some(470u64);

        let actual = dl.determine_quality(&download_urls, wanted_quality);

        assert_eq!(actual.is_ok(), true);
        assert_eq!(actual.unwrap(), "480");
    }
}
