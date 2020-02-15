use structopt::StructOpt;

// TODO: .after_help(format!("Build: {} - {}", env!("GIT_COMMIT"), env!("BUILD_DATE")).as_str())

#[derive(StructOpt, Debug, PartialEq, Eq)]
#[structopt(about, author)]
pub struct UdemyCmd {
    /// URL of the course to download
    #[structopt(short, long, name = "URL")]
    pub url: String,
    /// Access token to authenticate to udemy
    #[structopt(short = "t", long, name = "TOKEN")]
    pub access_token: String,
    /// Sets the level of verbosity
    #[structopt(short, long)]
    pub verbose: bool,
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt, Debug, PartialEq, Eq)]
pub enum Command {
    /// Query course information
    Info,
    /// Mark courses as completed
    Complete {
        /// Restrict marking a specific chapter.
        #[structopt(short, long, name = "CHAPTER")]
        chapter: Option<u64>,
        /// Restrict marking a specific lecture.
        #[structopt(short, long, name = "LECTURE")]
        lecture: Option<u64>,
    },
    /// Download course content
    Download {
        /// Dry run, show what would be done but don't download anything.
        #[structopt(short, long)]
        dry_run: bool,
        /// Restrict marking a specific chapter.
        #[structopt(short, long, name = "CHAPTER")]
        chapter: Option<u64>,
        /// Restrict marking a specific lecture.
        #[structopt(short, long, name = "LECTURE")]
        lecture: Option<u64>,
        /// Download specific video quality.
        #[structopt(short, long, name = "QUALITY")]
        quality: Option<u64>,
        /// Directory where to output downloaded files (default to .).
        #[structopt(short, long, name = "OUTPUT_DIR", default_value = ".")]
        output: String,
    },
}

pub fn parse_options() -> UdemyCmd {
    UdemyCmd::from_args()
}

#[cfg(test)]
mod test_s3cmd {
    use super::*;
    use std::ffi::OsString;

    #[test]
    fn parse_info_command_long() {
        let actual = UdemyCmd::from_iter_safe(vec![
            OsString::from(String::from("udem-dl-rs")),
            OsString::from(String::from("--url")),
            OsString::from(String::from("the-url")),
            OsString::from(String::from("--access-token")),
            OsString::from(String::from("the-token")),
            OsString::from(String::from("info")),
        ]);
        assert_eq!(actual.is_ok(), true);
        assert_eq!(
            actual.unwrap(),
            UdemyCmd {
                url: String::from("the-url"),
                access_token: String::from("the-token"),
                verbose: false,
                command: Command::Info {}
            }
        )
    }

    #[test]
    fn parse_info_command_short() {
        let actual = UdemyCmd::from_iter_safe(vec![
            OsString::from(String::from("udem-dl-rs")),
            OsString::from(String::from("-u")),
            OsString::from(String::from("the-url")),
            OsString::from(String::from("-t")),
            OsString::from(String::from("the-token")),
            OsString::from(String::from("info")),
        ]);
        assert_eq!(actual.is_ok(), true);
        assert_eq!(
            actual.unwrap(),
            UdemyCmd {
                url: String::from("the-url"),
                access_token: String::from("the-token"),
                verbose: false,
                command: Command::Info {}
            }
        )
    }

    #[test]
    fn parse_complete_command_defaults_long() {
        let actual = UdemyCmd::from_iter_safe(vec![
            OsString::from(String::from("udem-dl-rs")),
            OsString::from(String::from("--url")),
            OsString::from(String::from("the-url")),
            OsString::from(String::from("--access-token")),
            OsString::from(String::from("the-token")),
            OsString::from(String::from("complete")),
        ]);
        assert_eq!(actual.is_ok(), true);
        assert_eq!(
            actual.unwrap(),
            UdemyCmd {
                url: String::from("the-url"),
                access_token: String::from("the-token"),
                verbose: false,
                command: Command::Complete {
                    chapter: None,
                    lecture: None
                }
            }
        )
    }

    #[test]
    fn parse_complete_command_specifics_long() {
        let actual = UdemyCmd::from_iter_safe(vec![
            OsString::from(String::from("udem-dl-rs")),
            OsString::from(String::from("--url")),
            OsString::from(String::from("the-url")),
            OsString::from(String::from("--access-token")),
            OsString::from(String::from("the-token")),
            OsString::from(String::from("complete")),
            OsString::from(String::from("--chapter")),
            OsString::from(String::from("23")),
            OsString::from(String::from("--lecture")),
            OsString::from(String::from("34")),
        ]);
        assert_eq!(actual.is_ok(), true);
        assert_eq!(
            actual.unwrap(),
            UdemyCmd {
                url: String::from("the-url"),
                access_token: String::from("the-token"),
                verbose: false,
                command: Command::Complete {
                    chapter: Some(23),
                    lecture: Some(34)
                }
            }
        )
    }

    #[test]
    fn parse_complete_command_specifics_short() {
        let actual = UdemyCmd::from_iter_safe(vec![
            OsString::from(String::from("udem-dl-rs")),
            OsString::from(String::from("--url")),
            OsString::from(String::from("the-url")),
            OsString::from(String::from("--access-token")),
            OsString::from(String::from("the-token")),
            OsString::from(String::from("complete")),
            OsString::from(String::from("-c")),
            OsString::from(String::from("23")),
            OsString::from(String::from("-l")),
            OsString::from(String::from("34")),
        ]);
        assert_eq!(actual.is_ok(), true);
        assert_eq!(
            actual.unwrap(),
            UdemyCmd {
                url: String::from("the-url"),
                access_token: String::from("the-token"),
                verbose: false,
                command: Command::Complete {
                    chapter: Some(23),
                    lecture: Some(34)
                }
            }
        )
    }

    #[test]
    fn parse_download_command_specifics_long() {
        let actual = UdemyCmd::from_iter_safe(vec![
            OsString::from(String::from("udem-dl-rs")),
            OsString::from(String::from("--url")),
            OsString::from(String::from("the-url")),
            OsString::from(String::from("--access-token")),
            OsString::from(String::from("the-token")),
            OsString::from(String::from("download")),
            OsString::from(String::from("--dry-run")),
            OsString::from(String::from("--chapter")),
            OsString::from(String::from("23")),
            OsString::from(String::from("--lecture")),
            OsString::from(String::from("34")),
            OsString::from(String::from("--quality")),
            OsString::from(String::from("720")),
            OsString::from(String::from("--output")),
            OsString::from(String::from("the-output-dir")),
        ]);
        assert_eq!(actual.is_ok(), true);
        assert_eq!(
            actual.unwrap(),
            UdemyCmd {
                url: String::from("the-url"),
                access_token: String::from("the-token"),
                verbose: false,
                command: Command::Download {
                    dry_run: true,
                    chapter: Some(23),
                    lecture: Some(34),
                    quality: Some(720),
                    output: String::from("the-output-dir")
                }
            }
        )
    }

    #[test]
    fn parse_usage() {
        let actual = UdemyCmd::from_iter_safe(vec![
            OsString::from(String::from("udem-dl-rs")),
            OsString::from(String::from("--help")),
        ]);
        assert_eq!(actual.is_err(), true);
        let message = actual.err().unwrap().message;
        assert_eq!(message.contains("USAGE"), true);
    }
}
