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
        path_buf.push(format!("{:03} {}", chapter.object_index, UdemyHelper::sanitize(chapter.title.as_str())));
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
            UdemyHelper::sanitize(lecture.title.as_str()),
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

    /// Returns a cross-platform-filename-safe version of any string.
    ///
    /// This is used internally to generate app data directories based on app
    /// name/author. App developers can use it for consistency when dealing with
    /// file system operations.
    ///
    /// Do not apply this function to full paths, as it will sanitize '/' and '\';
    /// it should only be used on directory or file names (i.e. path segments).
    pub fn sanitize(component: &str) -> String {
        let mut buf = String::with_capacity(component.len());
        for (i, c) in component.chars().enumerate() {
            let is_lower = 'a' <= c && c <= 'z';
            let is_upper = 'A' <= c && c <= 'Z';
            let is_letter = is_upper || is_lower;
            let is_number = '0' <= c && c <= '9';
            let is_space = c == ' ';
            let is_hyphen = c == '-';
            let is_underscore = c == '_';
            let is_period = c == '.' && i != 0; // Disallow accidentally hidden folders
            let is_valid =
                is_letter || is_number || is_space || is_hyphen || is_underscore || is_period;
            if is_valid {
                buf.push(c);
            } else {
                buf.push_str("_");
            }
        }
        buf
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

    #[test]
    fn sanitize_normal() {
        let actual = UdemyHelper::sanitize("the-filename.mp4");

        assert_eq!(actual, "the-filename.mp4");
    }

    #[test]
    fn sanitize_illegal() {
        let actual = UdemyHelper::sanitize(
            r#"087 Styling & Positioning our Badge with "absolute" and "relative".mp4"#,
        );

        assert_eq!(
            actual,
            "087 Styling _ Positioning our Badge with _absolute_ and _relative_.mp4"
        );
    }
}
