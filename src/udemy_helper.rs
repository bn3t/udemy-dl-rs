#[allow(unused_imports)]
use failure::{format_err, Error};

use std::path::{Path, PathBuf};

use crate::fs_helper::*;
use crate::model::*;
use crate::utils::*;

pub struct UdemyHelper<'a> {
    fs_helper: &'a dyn FsHelper,
}

impl<'a> UdemyHelper<'a> {
    pub fn new(fs_helper: &'a dyn FsHelper) -> UdemyHelper<'a> {
        UdemyHelper { fs_helper }
    }

    pub fn calculate_target_dir(
        &self,
        target_dir: &str,
        chapter: &Chapter,
        course_name: &str,
    ) -> Result<String, Error> {
        let mut path_buf = PathBuf::from(target_dir);
        path_buf.push(course_name);
        path_buf.push(format!(
            "{:03} {}",
            chapter.object_index,
            sanitize(chapter.title.as_str())
        ));
        let path = String::from(
            path_buf
                .to_str()
                .ok_or_else(|| format_err!("Could not obtain target_dir"))?,
        );
        Ok(path)
    }

    pub fn calculate_target_filename(
        &self,
        target_dir: &str,
        lecture: &Lecture,
    ) -> Result<String, Error> {
        let mut path_buf = PathBuf::from(target_dir);
        let extension = Path::new(lecture.filename.as_str()).extension().unwrap();
        path_buf.push(format!(
            "{:03} {}.{}",
            lecture.object_index,
            sanitize(lecture.title.as_str()),
            extension.to_string_lossy()
        ));
        let path = String::from(
            path_buf
                .to_str()
                .ok_or_else(|| format_err!("Could not obtain target_dir"))?,
        );
        Ok(path)
    }

    pub fn create_target_dir(&self, path: &str) -> Result<(), Error> {
        self.fs_helper.create_dir_recursive(path)?;
        Ok(())
    }
}

#[cfg(test)]
mod test_udemy_helper {
    use super::*;

    struct MockFsHelper {}

    impl FsHelper for MockFsHelper {
        fn create_dir_recursive(&self, _path: &str) -> Result<(), Error> {
            Ok(())
        }
    }

    #[test]
    fn calculate_target_dir() {
        let chapter = Chapter {
            object_index: 23,
            title: "The Title".into(),
            lectures: vec![],
        };

        let fs_helper = MockFsHelper {};
        let udemy_helper = UdemyHelper::new(&fs_helper);
        let actual = udemy_helper.calculate_target_dir("./", &chapter, "my-course");

        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), "./my-course/023 The Title");
    }

    #[test]
    fn calculate_target_file() {
        let lecture = Lecture {
            has_video: true,
            filename: "blah-blah.mp4".into(),
            id: 4321,
            object_index: 32,
            title: "The Lecture".into(),
        };

        let fs_helper = MockFsHelper {};
        let udemy_helper = UdemyHelper::new(&fs_helper);

        let actual = udemy_helper.calculate_target_filename("./", &lecture);

        assert!(actual.is_ok());
    }

}
