use crate::command::*;
use std::any::Any;

use failure::Error;

use crate::model::*;

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

    fn execute(&self, context: &CommandContext) -> Result<(), Error> {
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
}

#[cfg(test)]
mod test {

    use super::*;

    use crate::downloader::UdemyDownloader;
    use crate::mocks::*;
    use crate::test_data::*;
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
