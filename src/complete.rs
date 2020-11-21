use std::any::Any;

use failure::format_err;

use crate::command::*;
use crate::model::*;
use crate::result::Result;

pub struct CompleteParams {
    pub wanted_chapter: ObjectIndex,
    pub wanted_lecture: Option<LectureId>,
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

    fn execute(&self, context: &CommandContext) -> Result<()> {
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
        wanted_lecture: Option<ObjectIndex>,
        verbose: bool,
    ) -> Result<()> {
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
    ) -> Result<()> {
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
        wanted_chapter: ObjectIndex,
        wanted_lecture: Option<LectureId>,
        verbose: bool,
    ) -> Result<()> {
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

#[cfg(test)]
mod test {
    use super::*;

    use crate::mocks::test::*;
    use crate::test_data::test::*;
    use crate::udemy_helper::UdemyHelper;

    #[test]
    fn complete() {
        unsafe {
            PARSE = Some(vec![]);
            GETS_AS_JSON_URL = Some(vec![]);
            GETS_CONTENT_LENGTH_URL = Some(vec![]);
            GETS_AS_DATA_URL = Some(vec![]);
            POST_JSON_DATA_URL = Some(vec![]);
            POST_JSON_DATA_BODY = Some(vec![]);
        }

        let fs_helper = MockFsHelper {};

        let mock_http_client = MockHttpClient {};
        let mock_parser = MockParser::new();
        let udemy_helper = UdemyHelper::new(&fs_helper);
        let auth = Auth::with_token("blah");

        let mut context = CommandContext::new(
            "https://www.udemy.com/course/css-the-complete-guide-incl-flexbox-grid-sass",
            &mock_http_client,
            &mock_parser,
            &udemy_helper,
            auth,
        )
        .unwrap();

        context.course = Some(make_course());
        context.course_content = Some(make_test_course_content());

        let mut complete = Complete::new();
        complete.set_params(&CompleteParams {
            wanted_chapter: 1,
            wanted_lecture: Some(1),
            verbose: false,
        });

        let result = complete.execute(&context);

        assert!(result.is_ok());

        unsafe {
            if let Some(ref pjd) = POST_JSON_DATA_URL {
                assert_eq!(pjd.len(), 1);
                assert_eq!(pjd[0], "https://www.udemy.com/api-2.0/users/me/subscribed-courses/54321/completed-lectures/");
            }
            if let Some(ref pjdb) = POST_JSON_DATA_BODY {
                assert_eq!(pjdb.len(), 1);
                assert_eq!(pjdb[0], "{\"downloaded\":false,\"lecture_id\":4321}");
            }
        }
    }

    #[test]
    fn complte() {}
}
