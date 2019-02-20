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
