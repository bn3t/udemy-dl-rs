use failure::{format_err, Error};
use serde_json::Value;

use crate::model::*;

pub trait Parser {
    fn parse_subscribed_courses(&self, subscribed_courses: &Value) -> Result<Vec<Course>, Error>;
    fn parse_course_content(&self, full_course: &Value) -> Result<CourseContent, Error>;
}

pub struct UdemyParser {}

/// Parser for json data coming from udemy.
impl UdemyParser {
    /// New an instance of this struct.
    pub fn new() -> UdemyParser {
        UdemyParser {}
    }
    /// Parse assets from the full course data.
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

    /// Parse json from a specific asset.
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
}

impl Parser for UdemyParser {
    /// Parse subscribed courses for this user.
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

    /// Parse full course content.
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
}

#[cfg(test)]
mod test_udemy_downloader {
    use serde_json::Value;

    use super::*;
    use crate::test_data::*;

    #[test]
    fn parse_subscribed_courses() {
        let subscribed_courses: Value = serde_json::from_str(TEST_SUBSCRIBED_COURSES).unwrap();

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
        let full_course: Value = serde_json::from_str(TEST_FULL_COURSE).unwrap();

        let parser = UdemyParser::new();

        let actual = parser.parse_course_content(&full_course);

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

        let parser = UdemyParser::new();

        let actual = parser.parse_asset(&asset);

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

        let parser = UdemyParser::new();

        let actual = parser.parse_assets(&assets);

        assert_eq!(actual.is_ok(), true);
    }
}
