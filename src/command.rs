use std::any::Any;

use failure::{format_err, Error};
use regex::Regex;

use crate::http_client::HttpClient;
use crate::model::*;
use crate::parser::*;
use crate::udemy_helper::*;

pub struct CommandContext<'a> {
    pub course_name: String,
    pub portal_name: String,
    pub course: Option<Course>,
    pub course_content: Option<CourseContent>,
    pub auth: Auth,
    pub parser: &'a Parser,
    pub client: &'a HttpClient,
    pub udemy_helper: &'a UdemyHelper<'a>,
}

impl<'a> CommandContext<'a> {
    pub fn new(
        url: &str,
        client: &'a HttpClient,
        parser: &'a Parser,
        udemy_helper: &'a UdemyHelper,
        auth: Auth,
    ) -> Result<CommandContext<'a>, Error> {
        let re = Regex::new(
            r"(?i)(?://(?P<portal_name>.+?).udemy.com/(?P<course_name>[a-zA-Z0-9_-]+))",
        )?;
        let captures = re
            .captures(url)
            .ok_or_else(|| format_err!("Could not parse provide url <{}>", url))?;
        let course_name = String::from(
            captures
                .name("course_name")
                .ok_or_else(|| format_err!("Could not compute course name out of url <{}>", url))?
                .as_str(),
        );
        let portal_name = String::from(
            captures
                .name("portal_name")
                .ok_or_else(|| format_err!("Could not compute portal name out of url <{}>", url))?
                .as_str(),
        );
        Ok(CommandContext {
            course: None,
            course_content: None,
            course_name,
            portal_name,
            client,
            parser,
            udemy_helper,
            auth,
        })
    }
}

pub trait Command {
    fn set_params(&mut self, params: &dyn Any);
    fn execute(&self, command_context: &CommandContext) -> Result<(), Error>;
}
