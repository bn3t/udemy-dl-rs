use crate::command::*;
use std::any::Any;

use crate::model::*;
use crate::result::Result;

pub struct InfoParams {
    pub verbose: bool,
}

pub struct Info {
    params: Option<InfoParams>,
}

impl Info {
    pub fn new() -> Info {
        Info { params: None }
    }
}

impl Command for Info {
    fn set_params(&mut self, params: &dyn Any) {
        if let Some(params) = params.downcast_ref::<InfoParams>() {
            self.params = Some(InfoParams {
                verbose: params.verbose,
            });
        }
    }

    fn execute(&self, context: &CommandContext) -> Result<()> {
        self.print_course_content(context.course_content.as_ref().unwrap());
        Ok(())
    }
}

impl Info {
    fn print_course_content(&self, course_content: &CourseContent) {
        for chapter in course_content.chapters.iter() {
            println!("{:03} Chapter {}", chapter.object_index, chapter.title);
            for lecture in chapter.lectures.iter() {
                println!("\t{:03} Lecture {}", lecture.object_index, lecture.title);
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use crate::downloader::UdemyDownloader;
    use crate::mocks::test::*;
    use crate::test_data::test::*;
    use crate::udemy_helper::UdemyHelper;

    #[test]
    fn info() {
        let fs_helper = MockFsHelper {};

        let mock_http_client = MockHttpClient {};
        let mock_parser = MockParser::new();
        let udemy_helper = UdemyHelper::new(&fs_helper);
        let auth = Auth::with_token("blah");

        let mut context = CommandContext::new(
            "https://www.udemy.com/css-the-complete-guide-incl-flexbox-grid-sass",
            &mock_http_client,
            &mock_parser,
            &udemy_helper,
            auth,
        )
        .unwrap();

        context.course = Some(make_course());
        context.course_content = Some(make_test_course_content());
        let downloader = UdemyDownloader::new(&mut context);

        let mut info = Info::new();
        info.set_params(&InfoParams { verbose: true });

        let result = downloader.execute(&info);

        assert!(result.is_ok());
    }
}
