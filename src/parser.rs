use failure::format_err;
use serde_json::Value;

use crate::model::*;
use crate::result::Result;
use crate::utils::{json_get_string, json_get_u64};

pub trait Parser {
    fn parse_subscribed_courses(&self, subscribed_courses: &Value) -> Result<Vec<Course>>;
    fn parse_course_content(&self, full_course: &Value) -> Result<CourseContent>;
    fn parse_lecture_detail(&self, lecture_detail: &Value) -> Result<LectureDetail>;
}

pub struct UdemyParser {}

/// Parser for json data coming from udemy.
impl UdemyParser {
    /// New an instance of this struct.
    pub fn new() -> UdemyParser {
        UdemyParser {}
    }

    /// Parse json from a specific asset.
    fn parse_asset(&self, asset: &Value) -> Result<Asset> {
        let title: String = json_get_string(asset, "title")?.into();
        let asset_type: String = json_get_string(asset, "asset_type")?.into();
        let time_estimation: u64 = json_get_u64(asset, "time_estimation")?;
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
            title,
            asset_type,
            time_estimation,
            download_urls,
        })
    }
}

impl Parser for UdemyParser {
    /// Parse subscribed courses for this user.
    fn parse_subscribed_courses(&self, subscribed_courses: &Value) -> Result<Vec<Course>> {
        let results = subscribed_courses
            .get("results")
            .ok_or_else(|| format_err!("Error parsing json"))?
            .as_array()
            .ok_or_else(|| format_err!("Error parsing json"))?;
        let courses: Vec<Course> = results
            .iter()
            .map(|result| serde_json::from_value(result.clone()))
            .filter(|course| course.is_ok())
            .map(|course| course.unwrap())
            .collect();
        Ok(courses)
    }

    /// Parse full course content.
    fn parse_course_content(&self, full_course: &Value) -> Result<CourseContent> {
        let results = full_course
            .get("results")
            .ok_or_else(|| format_err!("Error parsing json"))?
            .as_array()
            .ok_or_else(|| format_err!("Error parsing json"))?;

        let mut chapters: Vec<Chapter> = Vec::new();
        let mut lectures: Vec<Lecture> = Vec::new();
        let mut current_chapter: Option<Chapter> = None;

        for item in results.iter() {
            if item.get("_class").unwrap() == "chapter" {
                if current_chapter.is_some() {
                    let mut this_chapter = current_chapter.unwrap();
                    this_chapter.lectures = lectures;
                    chapters.push(this_chapter);
                }
                current_chapter = Some(Chapter {
                    object_index: json_get_u64(item, "object_index")?,
                    title: json_get_string(item, "title")?.into(),
                    lectures: Vec::new(),
                });
                lectures = Vec::new();
            }
            if item.get("_class").unwrap() == "lecture" {
                let asset = item
                    .get("asset")
                    .ok_or_else(|| format_err!("Error parsing json (asset)"))?;
                let filename = json_get_string(asset, "title")?.into();
                let has_video = json_get_string(asset, "asset_type")? == "Video";
                lectures.push(Lecture {
                    has_video,
                    filename,
                    id: json_get_u64(item, "id")?,
                    object_index: json_get_u64(item, "object_index")?,
                    title: json_get_string(item, "title")?.into(),
                });
            }
        }
        if current_chapter.is_some() {
            let mut this_chapter = current_chapter.unwrap();
            this_chapter.lectures.append(&mut lectures);
            chapters.push(this_chapter);
        }
        Ok(CourseContent { chapters })
    }

    fn parse_lecture_detail(&self, item: &Value) -> Result<LectureDetail> {
        let asset = self.parse_asset(
            item.get("asset")
                .ok_or_else(|| format_err!("Error parsing json (assets)"))?,
        )?;
        Ok(LectureDetail {
            id: json_get_u64(item, "id")?,
            title: json_get_string(item, "title")?.into(),
            asset,
        })
    }
}

#[cfg(test)]
mod test_udemy_downloader {
    use serde_json::Value;
    use std::fs;

    use super::*;

    #[test]
    fn parse_lecture_detail() {
        let lecture_detail =
            fs::read_to_string("test-data/subscribed-courses-lecture.json").unwrap();
        let lecture_detail = serde_json::from_str(lecture_detail.as_str()).unwrap();

        let parser = UdemyParser::new();

        let actual = parser.parse_lecture_detail(&lecture_detail);

        println!("{:?}", actual);

        assert_eq!(actual.is_ok(), true);
    }

    #[test]
    fn parse_subscribed_courses() {
        let subscribed_courses = fs::read_to_string("test-data/subscribed-courses.json").unwrap();
        let subscribed_courses: Value = serde_json::from_str(subscribed_courses.as_str()).unwrap();

        let parser = UdemyParser::new();

        let actual = parser.parse_subscribed_courses(&subscribed_courses);

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
        let full_course = fs::read_to_string("test-data/subscriber-curriculum-items.json").unwrap();
        let full_course: Value = serde_json::from_str(full_course.as_str()).unwrap();

        let parser = UdemyParser::new();

        let actual = parser.parse_course_content(&full_course);

        assert_eq!(actual.is_ok(), true);
        let course_content = actual.unwrap();
        assert_eq!(course_content.chapters.len(), 31);

        assert_eq!(course_content.chapters[0].object_index, 1);
        assert_eq!(course_content.chapters[0].title, "Getting Started");
        assert_eq!(course_content.chapters[1].title, "Angular Refresher");

        assert_eq!(course_content.chapters[0].lectures.len(), 12);
        assert_eq!(course_content.chapters[0].lectures[0].object_index, 1);
        assert_eq!(
            course_content.chapters[0].lectures[0].title,
            "Course Introduction"
        );
        assert_eq!(
            course_content.chapters[0].lectures[1].title,
            "What Is Ionic?"
        );
        assert_eq!(course_content.chapters[4].lectures[3].has_video, false);
    }

    #[test]
    fn parse_asset() {
        let asset = fs::read_to_string("test-data/asset.json").unwrap();
        let asset: Value = serde_json::from_str(asset.as_str()).unwrap();

        let parser = UdemyParser::new();

        let actual = parser.parse_asset(&asset);

        assert_eq!(actual.is_ok(), true);
        let asset = actual.unwrap();
        // assert_eq!(asset.filename, "getting-started-01-welcome.mp4");
        assert_eq!(asset.asset_type, "Video");
        assert_eq!(asset.time_estimation, 753);
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
        assert_eq!(asset.download_urls.as_ref().unwrap()[0].file, "https://udemy-assets-on-demand2.udemy.com/2019-02-19_17-49-23-1eacdfac67e2b9a4b011e02bbf95cb80/WebHD_720p.mp4?nva=20190301211904&download=True&filename=native-app-03-creating-an-android-app.mp4&token=0954657f324d1c9cc3818");
        assert_eq!(asset.download_urls.as_ref().unwrap()[0].label, "720");
        assert_eq!(asset.download_urls.as_ref().unwrap()[1].label, "480");
        assert_eq!(asset.download_urls.as_ref().unwrap()[2].label, "360");
        assert_eq!(asset.download_urls.as_ref().unwrap()[3].label, "144");
    }
}
