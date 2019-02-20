use std::any::Any;

use failure::{format_err, Error};

use crate::command::*;
use crate::model::*;

pub struct CompleteParams {
    pub wanted_chapter: u64,
    pub wanted_lecture: Option<u64>,
    pub verbose: bool,
}

/// Complete files to a specified location. It is possible to specify
/// which chapter / lecture to download.
pub struct Complete {
    params: Option<CompleteParams>,
}

impl Complete {
    pub fn new() -> Complete {
        Complete { params: None }
    }
}

impl Command for Complete {
    fn set_params(&mut self, params: &dyn Any) {
        if let Some(params) = params.downcast_ref::<CompleteParams>() {
            self.params = Some(CompleteParams {
                wanted_chapter: params.wanted_chapter,
                wanted_lecture: params.wanted_lecture,
                verbose: params.verbose,
            });
        }
    }

    fn execute(&self, context: &CommandContext) -> Result<(), Error> {
        if let Some(params) = self.params.as_ref() {
            self.complete(
                context,
                params.wanted_chapter,
                params.wanted_lecture,
                params.verbose,
            )?;
            Ok(())
        } else {
            Err(format_err!(
                "Params should be populated for executing command"
            ))
        }
    }
}

impl Complete {
    fn complete_chapter(
        &self,
        context: &CommandContext,
        chapter: &Chapter,
        wanted_lecture: Option<u64>,
        verbose: bool,
    ) -> Result<(), Error> {
        if verbose {
            println!(
                "Completing chapter {} - {}",
                chapter.object_index, chapter.title
            );
        }
        chapter
            .lectures
            .iter()
            .filter(|lecture| {
                wanted_lecture.is_none() || wanted_lecture.unwrap() == lecture.object_index
            })
            .for_each(move |lecture| {
                match self.complete_lecture(context, &lecture, verbose) {
                    Ok(_) => {
                        // if verbose {
                        //     println!("Lecture downloaded");
                        // }
                    }
                    Err(e) => {
                        eprintln!("Error while completing {}: {}", lecture.title, e);
                    }
                };
            });
        Ok(())
    }

    fn complete_lecture(
        &self,
        context: &CommandContext,
        lecture: &Lecture,
        verbose: bool,
    ) -> Result<(), Error> {
        if verbose {
            println!("Completing lecture {}", lecture.title);
        }
        let url = format!(
            "https://{portal_name}.udemy.com/api-2.0/users/me/subscribed-courses/{course_id}/completed-lectures/",
            portal_name = context.portal_name,
            course_id = context.course.as_ref().unwrap().id
        );
        let complete_request = CompleteRequest {
            lecture_id: lecture.id,
            downloaded: false,
        };
        let value = serde_json::to_value(complete_request)?;
        context
            .client
            .post_json(url.as_str(), &value, &context.auth)?;
        Ok(())
    }

    /// Complete chapters and lectures.
    pub fn complete(
        &self,
        context: &CommandContext,
        wanted_chapter: u64,
        wanted_lecture: Option<u64>,
        verbose: bool,
    ) -> Result<(), Error> {
        if verbose {
            println!(
                "Complete chapter: {}, lecture: {:?}",
                wanted_chapter, wanted_lecture
            );
        }

        for chapter in context.course_content.as_ref().unwrap().chapters.iter() {
            if wanted_chapter == chapter.object_index {
                self.complete_chapter(context, &chapter, wanted_lecture, verbose)?;
            }
        }
        Ok(())
    }
}
