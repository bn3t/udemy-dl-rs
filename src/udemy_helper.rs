#[allow(unused_imports)]
use failure::{format_err, Error};

use std::fs::DirBuilder;
use std::path::{Path, PathBuf};

use crate::model::*;

pub struct UdemyHelper {}

impl UdemyHelper {
    pub fn calculate_target_dir(
        target_dir: &str,
        chapter: &Chapter,
        course_name: &str,
    ) -> Result<String, Error> {
        let mut path_buf = PathBuf::from(target_dir);
        path_buf.push(course_name);
        path_buf.push(format!("{:03} {}", chapter.object_index, chapter.title));
        let path = String::from(
            path_buf
                .to_str()
                .ok_or_else(|| format_err!("Could not obtain target_dir"))?,
        );
        Ok(path)
    }

    pub fn calculate_target_filename(target_dir: &str, lecture: &Lecture) -> Result<String, Error> {
        let mut path_buf = PathBuf::from(target_dir);
        let extension = Path::new(lecture.asset.filename.as_str())
            .extension()
            .unwrap();
        path_buf.push(format!(
            "{:03} {}.{}",
            lecture.object_index,
            lecture.title,
            extension.to_string_lossy()
        ));
        let path = String::from(
            path_buf
                .to_str()
                .ok_or_else(|| format_err!("Could not obtain target_dir"))?,
        );
        Ok(path)
    }

    pub fn create_target_dir(path: &str) -> Result<(), Error> {
        DirBuilder::new().recursive(true).create(path)?;
        Ok(())
    }
}

#[cfg(test)]
mod test_udemy_helper {
    use super::*;

    #[test]
    fn calculate_target_dir() {
        let chapter = Chapter {
            object_index: 23,
            title: "The Title".into(),
            lectures: vec![],
        };
        let actual = UdemyHelper::calculate_target_dir("./", &chapter, "my-course");

        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), "./my-course/023 The Title");
    }

    #[test]
    fn calculate_target_file() {
        let lecture = Lecture {
            object_index: 32,
            title: "The Lecture".into(),
            asset: Asset {
                filename: "blah-blah.mp4".into(),
                asset_type: "Video".into(),
                time_estimation: 234,
                download_urls: Some(vec![DownloadUrl {
                    r#type: Some("video/mp4".into()),
                    file: "http://the-host/thefile.mp4".into(),
                    label: "720".into(),
                }]),
            },
            supplementary_assets: vec![],
        };
        let actual = UdemyHelper::calculate_target_filename("./", &lecture);

        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), "./032 The Lecture.mp4");
    }

}
