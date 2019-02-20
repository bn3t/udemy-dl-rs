#![allow(clippy::too_many_arguments)]

use failure::{format_err, Error};

use crate::command::*;
use crate::model::*;

const PORTAL_NAME: &str = "www";
const COURSE_SEARCH: &str = "https://{portal_name}.udemy.com/api-2.0/users/me/subscribed-courses?fields[course]=id,url,published_title&page=1&page_size=1000&ordering=-access_time&search={course_name}";
const LOGIN_URL: &str =
    "https://www.udemy.com/api-2.0/auth/udemy-auth/login/?fields[user]=access_token";

pub struct UdemyDownloader<'a> {
    command_context: &'a mut CommandContext<'a>,
}

type CourseId = u64;

impl<'a> UdemyDownloader<'a> {
    pub fn new(context: &'a mut CommandContext<'a>) -> UdemyDownloader<'a> {
        UdemyDownloader {
            command_context: context,
        }
    }

    fn get_subscribed_course(&self, verbose: bool) -> Result<Course, Error> {
        if verbose {
            println!("Requesting subscribed courses");
        }
        let url = format!(
            "https://{portal_name}.udemy.com/api-2.0/users/me/subscribed-courses?fields[course]=id,url,published_title&page=1&page_size=1000&ordering=-access_time&search={course_name}",
            portal_name = self.command_context.portal_name,
            course_name = self.command_context.course_name
        );
        let value = self
            .command_context
            .client
            .get_as_json(url.as_str(), &self.command_context.auth)?;
        self.command_context
            .parser
            .parse_subscribed_courses(&value)?
            .into_iter()
            .find(|course| course.published_title == self.command_context.course_name)
            .ok_or_else(|| {
                format_err!(
                    "{} was not found in subscribed courses",
                    self.command_context.course_name
                )
            })
    }

    fn get_info(&self, course: &Course, verbose: bool) -> Result<String, Error> {
        let url = format!("https://{portal_name}.udemy.com/api-2.0/courses/{course_id}/cached-subscriber-curriculum-items?fields[asset]=results,external_url,time_estimation,download_urls,slide_urls,filename,asset_type,captions,stream_urls,body&fields[chapter]=object_index,title,sort_order&fields[lecture]=id,title,object_index,asset,supplementary_assets,view_html&page_size=10000",
        portal_name = self.command_context.portal_name, course_id=course.id);

        if verbose {
            println!("Requesting info for course");
        }
        self.command_context
            .client
            .get_as_text(url.as_str(), &self.command_context.auth)
    }

    fn parse_info(&self, info: &str) -> Result<CourseContent, Error> {
        let value = serde_json::from_str(info)?;
        let course_content = self.command_context.parser.parse_course_content(&value)?;
        Ok(course_content)
    }

    pub fn authenticate(&mut self) -> Result<(), Error> {
        if self.command_context.auth.access_token.is_none() {
            let access_token = self
                .command_context
                .client
                .post_login_form(LOGIN_URL, &self.command_context.auth)?;
            self.command_context.auth.access_token = Some(access_token);
        }
        Ok(())
    }

    pub fn execute(&self, command: &Command) -> Result<(), Error> {
        command.execute(&self.command_context)
    }

    pub fn prepare_course_info(&mut self, verbose: bool) -> Result<(), Error> {
        let course = self.get_subscribed_course(verbose)?;
        let info = self.get_info(&course, verbose)?;
        let course_content = self.parse_info(info.as_str())?;

        self.command_context.course_content = Some(course_content);
        self.command_context.course = Some(course);

        Ok(())
    }
}

// #[cfg(test)]
// mod test_udemy_downloader {
//     use failure::Error;
//     use serde_json::Value;

//     use super::UdemyDownloader;
//     use crate::command::*;
//     use crate::fs_helper::FsHelper;
//     use crate::http_client::HttpClient;
//     use crate::info::*;
//     use crate::model::*;
//     use crate::parser::Parser;
//     use crate::udemy_helper::UdemyHelper;

//     static mut GETS_AS_JSON: Option<Vec<String>> = None;
//     static mut GETS_CONTENT_LENGTH: Option<Vec<String>> = None;
//     static mut GETS_AS_DATA: Option<Vec<String>> = None;
//     static mut PARSE: Option<Vec<String>> = None;

//     struct MockHttpClient {}

//     impl HttpClient for MockHttpClient {
//         fn get_as_text(&self, url: &str, _auth: &Auth) -> Result<String, Error> {
//             println!("get_as_text url={}", url);
//             unsafe {
//                 match GETS_AS_JSON {
//                     Some(ref mut gaj) => {
//                         gaj.push(String::from(url));
//                     }
//                     None => panic!(),
//                 }
//             };
//             Ok(format!(r#"{{ "url": "{}" }}"#, url))
//         }
//         fn get_content_length(&self, url: &str) -> Result<u64, Error> {
//             println!("get_content_length url={}", url);
//             unsafe {
//                 match GETS_CONTENT_LENGTH {
//                     Some(ref mut gcl) => {
//                         gcl.push(String::from(url));
//                     }
//                     None => panic!(),
//                 }
//             };
//             Ok(321)
//         }
//         fn get_as_data(&self, url: &str, _f: &mut FnMut(u64)) -> Result<Vec<u8>, Error> {
//             println!("get_as_data url={}", url);
//             unsafe {
//                 match GETS_AS_DATA {
//                     Some(ref mut gad) => {
//                         gad.push(String::from(url));
//                     }
//                     None => panic!(),
//                 }
//             };
//             Ok(vec![])
//         }
//         fn post_login_form(&self, _url: &str, _auth: &Auth) -> Result<String, Error> {
//             Ok("blah".into())
//         }
//         fn post_json(&self, _url: &str, _json: &Value, _auth: &Auth) -> Result<(), Error> {
//             Ok(())
//         }
//     }

//     struct MockParser {}

//     impl MockParser {
//         pub fn new() -> MockParser {
//             MockParser {}
//         }
//     }

//     impl Parser for MockParser {
//         fn parse_subscribed_courses(
//             &self,
//             subscribed_courses: &Value,
//         ) -> Result<Vec<Course>, Error> {
//             unsafe {
//                 match PARSE {
//                     Some(ref mut psc) => {
//                         psc.push(String::from(format!("{:?}", subscribed_courses)))
//                     }
//                     None => {
//                         panic!();
//                     }
//                 };
//             };
//             Ok(vec![Course {
//                 id: 54321,
//                 url: "the-url".into(),
//                 published_title: "css-the-complete-guide-incl-flexbox-grid-sass".into(),
//             }])
//         }
//         fn parse_course_content(&self, full_course: &Value) -> Result<CourseContent, Error> {
//             unsafe {
//                 match PARSE {
//                     Some(ref mut psc) => psc.push(String::from(format!("{:?}", full_course))),
//                     None => {
//                         panic!();
//                     }
//                 };
//             };
//             Ok(CourseContent {
//                 chapters: vec![Chapter {
//                     object_index: 1,
//                     title: "The Chapter".into(),
//                     lectures: vec![Lecture {
//                         id: 4321,
//                         object_index: 1,
//                         title: "The Lecture".into(),
//                         asset: Asset {
//                             filename: "the-filename.mp4".into(),
//                             asset_type: "Video".into(),
//                             time_estimation: 321,
//                             download_urls: Some(vec![DownloadUrl {
//                                 r#type: Some("video/mp4".into()),
//                                 file: "http://host-name/the-filename.mp4".into(),
//                                 label: "720".into(),
//                             }]),
//                         },
//                         supplementary_assets: vec![],
//                     }],
//                 }],
//             })
//         }
//     }

//     struct MockFsHelper {}

//     impl FsHelper for MockFsHelper {
//         fn create_dir_recursive(&self, _path: &str) -> Result<(), Error> {
//             Ok(())
//         }
//     }

//     #[test]
//     fn parse_url() {
//         unsafe {
//             PARSE = Some(vec![]);
//             GETS_AS_JSON = Some(vec![]);
//             GETS_CONTENT_LENGTH = Some(vec![]);
//             GETS_AS_DATA = Some(vec![]);
//         }

//         let fs_helper = MockFsHelper {};

//         let mock_http_client = MockHttpClient {};
//         let mock_parser = MockParser::new();
//         let udemy_helper = UdemyHelper::new(&fs_helper);
//         let auth = Auth::with_token("blah");
//         let dl = UdemyDownloader::new(
//             "https://www.udemy.com/css-the-complete-guide-incl-flexbox-grid-sass",
//             &mock_http_client,
//             &mock_parser,
//             &udemy_helper,
//             auth,
//         )
//         .unwrap();

//         assert_eq!(
//             dl.command_context.course_name,
//             "css-the-complete-guide-incl-flexbox-grid-sass"
//         );
//         assert_eq!(dl.command_context.portal_name, "www");
//     }

//     #[test]
//     fn info() {
//         unsafe {
//             PARSE = Some(vec![]);
//             GETS_AS_JSON = Some(vec![]);
//             GETS_CONTENT_LENGTH = Some(vec![]);
//             GETS_AS_DATA = Some(vec![]);
//         }

//         let fs_helper = MockFsHelper {};

//         let mock_http_client = MockHttpClient {};
//         let mock_parser = MockParser::new();
//         let udemy_helper = UdemyHelper::new(&fs_helper);
//         let auth = Auth::with_token("blah");
//         let dl = UdemyDownloader::new(
//             "https://www.udemy.com/css-the-complete-guide-incl-flexbox-grid-sass",
//             &mock_http_client,
//             &mock_parser,
//             &udemy_helper,
//             auth,
//         )
//         .unwrap();

//         let info = Info::new();
//         info.set_params(&InfoParams { verbose: true });

//         let result = dl.execute(&info);

//         assert!(result.is_ok());

//         unsafe {
//             if let Some(ref gaj) = GETS_AS_JSON {
//                 assert_eq!(gaj.len(), 2);
//                 assert_eq!(gaj[0], "https://www.udemy.com/api-2.0/users/me/subscribed-courses?fields[course]=id,url,published_title&page=1&page_size=1000&ordering=-access_time&search=css-the-complete-guide-incl-flexbox-grid-sass");
//                 assert_eq!(gaj[1], "https://www.udemy.com/api-2.0/courses/54321/cached-subscriber-curriculum-items?fields[asset]=results,external_url,time_estimation,download_urls,slide_urls,filename,asset_type,captions,stream_urls,body&fields[chapter]=object_index,title,sort_order&fields[lecture]=id,title,object_index,asset,supplementary_assets,view_html&page_size=10000");
//             }
//             if let Some(ref psc) = PARSE {
//                 assert_eq!(psc.len(), 2);
//                 assert_eq!(psc[0], "Object({\"url\": String(\"https://www.udemy.com/api-2.0/users/me/subscribed-courses?fields[course]=id,url,published_title&page=1&page_size=1000&ordering=-access_time&search=css-the-complete-guide-incl-flexbox-grid-sass\")})");
//                 assert_eq!(psc[1], "Object({\"url\": String(\"https://www.udemy.com/api-2.0/courses/54321/cached-subscriber-curriculum-items?fields[asset]=results,external_url,time_estimation,download_urls,slide_urls,filename,asset_type,captions,stream_urls,body&fields[chapter]=object_index,title,sort_order&fields[lecture]=id,title,object_index,asset,supplementary_assets,view_html&page_size=10000\")})");
//             }
//         }
//     }

//     #[test]
//     fn download() {
//         unsafe {
//             PARSE = Some(vec![]);
//             GETS_AS_JSON = Some(vec![]);
//             GETS_CONTENT_LENGTH = Some(vec![]);
//             GETS_AS_DATA = Some(vec![]);
//         }

//         let fs_helper = MockFsHelper {};

//         let mock_http_client = MockHttpClient {};
//         let mock_parser = MockParser::new();
//         let udemy_helper = UdemyHelper::new(&fs_helper);
//         let auth = Auth::with_token("blah");
//         let dl = UdemyDownloader::new(
//             "https://www.udemy.com/css-the-complete-guide-incl-flexbox-grid-sass",
//             &mock_http_client,
//             &mock_parser,
//             &udemy_helper,
//             auth,
//         )
//         .unwrap();

//         let download.

//         let result = dl.download(Some(1), Some(1), None, None, "~/Downloads", false, false);

//         assert!(result.is_ok());

//         unsafe {
//             if let Some(ref gaj) = GETS_AS_JSON {
//                 assert_eq!(gaj.len(), 2);
//                 assert_eq!(gaj[0], "https://www.udemy.com/api-2.0/users/me/subscribed-courses?fields[course]=id,url,published_title&page=1&page_size=1000&ordering=-access_time&search=css-the-complete-guide-incl-flexbox-grid-sass");
//                 assert_eq!(gaj[1], "https://www.udemy.com/api-2.0/courses/54321/cached-subscriber-curriculum-items?fields[asset]=results,external_url,time_estimation,download_urls,slide_urls,filename,asset_type,captions,stream_urls,body&fields[chapter]=object_index,title,sort_order&fields[lecture]=id,title,object_index,asset,supplementary_assets,view_html&page_size=10000");
//             }
//             if let Some(ref gcl) = GETS_CONTENT_LENGTH {
//                 assert_eq!(gcl.len(), 1);
//                 assert_eq!(gcl[0], "http://host-name/the-filename.mp4");
//             }
//             if let Some(ref gad) = GETS_AS_DATA {
//                 assert_eq!(gad.len(), 1);
//                 assert_eq!(gad[0], "http://host-name/the-filename.mp4");
//             }
//             if let Some(ref psc) = PARSE {
//                 assert_eq!(psc.len(), 2);
//                 assert_eq!(psc[0], "Object({\"url\": String(\"https://www.udemy.com/api-2.0/users/me/subscribed-courses?fields[course]=id,url,published_title&page=1&page_size=1000&ordering=-access_time&search=css-the-complete-guide-incl-flexbox-grid-sass\")})");
//                 assert_eq!(psc[1], "Object({\"url\": String(\"https://www.udemy.com/api-2.0/courses/54321/cached-subscriber-curriculum-items?fields[asset]=results,external_url,time_estimation,download_urls,slide_urls,filename,asset_type,captions,stream_urls,body&fields[chapter]=object_index,title,sort_order&fields[lecture]=id,title,object_index,asset,supplementary_assets,view_html&page_size=10000\")})");
//             }
//         }
//     }

//     #[test]
//     fn determine_quality_for_best() {
//         let fs_helper = MockFsHelper {};

//         let mock_http_client = MockHttpClient {};
//         let mock_parser = MockParser::new();
//         let udemy_helper = UdemyHelper::new(&fs_helper);
//         let auth = Auth::with_token("blah");
//         let dl = UdemyDownloader::new(
//             "https://www.udemy.com/css-the-complete-guide-incl-flexbox-grid-sass",
//             &mock_http_client,
//             &mock_parser,
//             &udemy_helper,
//             auth,
//         )
//         .unwrap();

//         let download_urls = vec![
//             DownloadUrl {
//                 label: "480".into(),
//                 file: "the-file-video-480".into(),
//                 r#type: Some("video/mp4".into()),
//             },
//             DownloadUrl {
//                 label: "720".into(),
//                 file: "the-file-video-720".into(),
//                 r#type: Some("video/mp4".into()),
//             },
//             DownloadUrl {
//                 label: "1720".into(),
//                 file: "the-file-720".into(),
//                 r#type: Some("other/mp4".into()),
//             },
//         ];
//         let wanted_quality = None;

//         let actual = dl.determine_quality(&download_urls, wanted_quality);

//         assert_eq!(actual.is_ok(), true);
//         assert_eq!(actual.unwrap(), "720");
//     }

//     #[test]
//     fn determine_quality_for_wanted_480() {
//         let fs_helper = MockFsHelper {};

//         let mock_http_client = MockHttpClient {};
//         let mock_parser = MockParser::new();
//         let udemy_helper = UdemyHelper::new(&fs_helper);
//         let auth = Auth::with_token("blah");
//         let dl = UdemyDownloader::new(
//             "https://www.udemy.com/css-the-complete-guide-incl-flexbox-grid-sass",
//             &mock_http_client,
//             &mock_parser,
//             &udemy_helper,
//             auth,
//         )
//         .unwrap();

//         let download_urls = vec![
//             DownloadUrl {
//                 label: "480".into(),
//                 file: "the-file-video-480".into(),
//                 r#type: Some("video/mp4".into()),
//             },
//             DownloadUrl {
//                 label: "360".into(),
//                 file: "the-file-video-360".into(),
//                 r#type: Some("video/mp4".into()),
//             },
//             DownloadUrl {
//                 label: "720".into(),
//                 file: "the-file-video-720".into(),
//                 r#type: Some("video/mp4".into()),
//             },
//             DownloadUrl {
//                 label: "1720".into(),
//                 file: "the-file-720".into(),
//                 r#type: Some("other/mp4".into()),
//             },
//         ];
//         let wanted_quality = Some(480u64);

//         let actual = dl.determine_quality(&download_urls, wanted_quality);

//         assert_eq!(actual.is_ok(), true);
//         assert_eq!(actual.unwrap(), "480");
//     }

//     #[test]
//     fn determine_quality_for_wanted_470() {
//         let fs_helper = MockFsHelper {};

//         let mock_http_client = MockHttpClient {};
//         let mock_parser = MockParser::new();
//         let udemy_helper = UdemyHelper::new(&fs_helper);
//         let auth = Auth::with_token("blah");
//         let dl = UdemyDownloader::new(
//             "https://www.udemy.com/css-the-complete-guide-incl-flexbox-grid-sass",
//             &mock_http_client,
//             &mock_parser,
//             &udemy_helper,
//             auth,
//         )
//         .unwrap();

//         let download_urls = vec![
//             DownloadUrl {
//                 label: "480".into(),
//                 file: "the-file-video-480".into(),
//                 r#type: Some("video/mp4".into()),
//             },
//             DownloadUrl {
//                 label: "360".into(),
//                 file: "the-file-video-360".into(),
//                 r#type: Some("video/mp4".into()),
//             },
//             DownloadUrl {
//                 label: "720".into(),
//                 file: "the-file-video-720".into(),
//                 r#type: Some("video/mp4".into()),
//             },
//             DownloadUrl {
//                 label: "1720".into(),
//                 file: "the-file-720".into(),
//                 r#type: Some("other/mp4".into()),
//             },
//         ];
//         let wanted_quality = Some(470u64);

//         let actual = dl.determine_quality(&download_urls, wanted_quality);

//         assert_eq!(actual.is_ok(), true);
//         assert_eq!(actual.unwrap(), "480");
//     }
// }
