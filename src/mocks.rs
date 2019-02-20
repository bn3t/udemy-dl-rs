use failure::Error;
use serde_json::Value;

use crate::fs_helper::FsHelper;
use crate::http_client::HttpClient;
use crate::model::*;
use crate::parser::Parser;

pub static mut GETS_AS_JSON: Option<Vec<String>> = None;
pub static mut GETS_CONTENT_LENGTH: Option<Vec<String>> = None;
pub static mut GETS_AS_DATA: Option<Vec<String>> = None;
pub static mut PARSE: Option<Vec<String>> = None;

pub struct MockHttpClient {}

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

pub struct MockParser {}

impl MockParser {
    pub fn new() -> MockParser {
        MockParser {}
    }
}

impl Parser for MockParser {
    fn parse_subscribed_courses(&self, subscribed_courses: &Value) -> Result<Vec<Course>, Error> {
        unsafe {
            match PARSE {
                Some(ref mut psc) => psc.push(format!("{:?}", subscribed_courses)),
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
                Some(ref mut psc) => psc.push(format!("{:?}", full_course)),
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

pub struct MockFsHelper {}

impl FsHelper for MockFsHelper {
    fn create_dir_recursive(&self, _path: &str) -> Result<(), Error> {
        Ok(())
    }
}
