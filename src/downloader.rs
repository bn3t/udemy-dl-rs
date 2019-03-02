use failure::{format_err, Error};

use crate::command::*;
use crate::model::*;

const PORTAL_NAME: &str = "www";
const COURSE_SEARCH: &str = "https://{portal_name}.udemy.com/api-2.0/users/me/subscribed-courses?fields[course]=id,url,published_title&page=1&page_size=1000&ordering=-access_time&search={course_name}";
const LOGIN_URL: &str =
    "https://www.udemy.com/api-2.0/auth/udemy-auth/login/?fields[user]=access_token";

pub struct UdemyDownloader<'a> {
    command_context: &'a mut CommandContext<'a>,
}

type CourseId = u64;

impl<'a> UdemyDownloader<'a> {
    pub fn new(context: &'a mut CommandContext<'a>) -> UdemyDownloader<'a> {
        UdemyDownloader {
            command_context: context,
        }
    }

    fn get_subscribed_course(&self, verbose: bool) -> Result<Course, Error> {
        if verbose {
            println!("Requesting subscribed courses");
        }
        let url = format!(
            "https://{portal_name}.udemy.com/api-2.0/users/me/subscribed-courses?fields[course]=id,url,published_title&page=1&page_size=1000&ordering=-access_time&search={course_name}",
            portal_name = self.command_context.portal_name,
            course_name = self.command_context.course_name
        );
        let value = self
            .command_context
            .client
            .get_as_json(url.as_str(), &self.command_context.auth)?;
        self.command_context
            .parser
            .parse_subscribed_courses(&value)?
            .into_iter()
            .find(|course| course.published_title == self.command_context.course_name)
            .ok_or_else(|| {
                format_err!(
                    "{} was not found in subscribed courses",
                    self.command_context.course_name
                )
            })
    }

    fn get_info(&self, course: &Course, verbose: bool) -> Result<String, Error> {
        let url = format!("https://{portal_name}.udemy.com/api-2.0/courses/{course_id}/subscriber-curriculum-items/?page_size=1400&fields[lecture]=@min,object_index,asset,supplementary_assets,sort_order,is_published,is_free&fields[quiz]=@min,object_index,title,sort_order,is_published&fields[practice]=@min,object_index,title,sort_order,is_published&fields[chapter]=@min,description,object_index,title,sort_order,is_published&fields[asset]=@min,title,filename,asset_type,external_url,length,status",
        portal_name = self.command_context.portal_name, course_id=course.id);

        if verbose {
            println!("Requesting info for course");
        }
        self.command_context
            .client
            .get_as_text(url.as_str(), &self.command_context.auth)
    }

    fn parse_info(&self, info: &str) -> Result<CourseContent, Error> {
        let value = serde_json::from_str(info)?;
        let course_content = self.command_context.parser.parse_course_content(&value)?;
        Ok(course_content)
    }

    pub fn authenticate(&mut self) -> Result<(), Error> {
        if self.command_context.auth.access_token.is_none() {
            let access_token = self
                .command_context
                .client
                .post_login_form(LOGIN_URL, &self.command_context.auth)?;
            self.command_context.auth.access_token = Some(access_token);
        }
        Ok(())
    }

    pub fn execute(&self, command: &Command) -> Result<(), Error> {
        command.execute(&self.command_context)
    }

    pub fn prepare_course_info(&mut self, verbose: bool) -> Result<(), Error> {
        let course = self.get_subscribed_course(verbose)?;
        let info = self.get_info(&course, verbose)?;
        let course_content = self.parse_info(info.as_str())?;

        self.command_context.course_content = Some(course_content);
        self.command_context.course = Some(course);

        Ok(())
    }
}

#[cfg(test)]
mod test_udemy_downloader {

    use crate::command::*;
    use crate::mocks::*;
    use crate::model::*;
    use crate::udemy_helper::UdemyHelper;

    #[test]
    fn parse_url() {
        unsafe {
            PARSE = Some(vec![]);
            GETS_AS_JSON_URL = Some(vec![]);
            GETS_CONTENT_LENGTH_URL = Some(vec![]);
            GETS_AS_DATA_URL = Some(vec![]);
        }

        let fs_helper = MockFsHelper {};

        let mock_http_client = MockHttpClient {};
        let mock_parser = MockParser::new();
        let udemy_helper = UdemyHelper::new(&fs_helper);
        let auth = Auth::with_token("blah");

        let context = CommandContext::new(
            "https://www.udemy.com/css-the-complete-guide-incl-flexbox-grid-sass",
            &mock_http_client,
            &mock_parser,
            &udemy_helper,
            auth,
        )
        .unwrap();
        assert_eq!(
            context.course_name,
            "css-the-complete-guide-incl-flexbox-grid-sass"
        );
        assert_eq!(context.portal_name, "www");
    }
}
