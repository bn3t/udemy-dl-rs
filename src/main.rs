use clap::{App, AppSettings, Arg, SubCommand};

use failure::{format_err, Error};

mod command;
mod complete;
mod download;
mod downloader;
mod fs_helper;
mod http_client;
mod info;
mod mocks;
mod model;
mod parser;
mod test_data;
mod udemy_helper;
mod utils;

use command::*;
use complete::*;
use download::*;
use downloader::UdemyDownloader;
use fs_helper::UdemyFsHelper;
use http_client::UdemyHttpClient;
use info::*;
use model::{Auth, LectureId, ObjectIndex, VideoQuality};
use parser::UdemyParser;
use udemy_helper::UdemyHelper;

fn main() {
    let matches = App::new("Udemy Downloader")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Bernard Niset")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .after_help(format!("Build: {} - {}", env!("GIT_COMMIT"), env!("BUILD_DATE")).as_str())
        .arg(
            Arg::with_name("url")
                .short("u")
                .long("url")
                .value_name("URL")
                .help("URL of the course to download")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("access_token")
                .short("t")
                .long("access-token")
                .value_name("TOKEN")
                .help("Access token to authenticate to udemy")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("username")
                .short("U")
                .long("username")
                .value_name("USERNAME")
                .help("Username to authenticate to udemy")
                .required_unless("access_token")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("password")
                .short("p")
                .long("password")
                .value_name("PASSWORD")
                .help("Password to authenticate to udemy")
                .required_unless("access_token")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(SubCommand::with_name("info").about("Query course information"))
        .subcommand(
            SubCommand::with_name("complete")
                .about("Mark courses as completed")
                .arg(
                    Arg::with_name("chapter")
                        .short("c")
                        .long("chapter")
                        .takes_value(true)
                        .value_name("CHAPTER")
                        .required(true)
                        .help("Restrict marking a specific chapter."),
                )
                .arg(
                    Arg::with_name("lecture")
                        .short("l")
                        .long("lecture")
                        .value_name("LECTURE")
                        .takes_value(true)
                        .help("Restrict marking a specific lecture."),
                ),
        )
        .subcommand(
            SubCommand::with_name("download")
                .about("Download course content")
                .arg(
                    Arg::with_name("dry-run")
                        .short("d")
                        .long("dry-run")
                        .takes_value(false)
                        .help("Dry run, show what's would be done but don't download anything."),
                )
                .arg(
                    Arg::with_name("chapter")
                        .short("c")
                        .long("chapter")
                        .takes_value(true)
                        .value_name("CHAPTER")
                        .help("Restrict downloads to a specific chapter."),
                )
                .arg(
                    Arg::with_name("lecture")
                        .short("l")
                        .long("lecture")
                        .value_name("LECTURE")
                        .takes_value(true)
                        .help("Restrict download to a specific lecture."),
                )
                .arg(
                    Arg::with_name("quality")
                        .short("q")
                        .long("quality")
                        .value_name("QUALITY")
                        .takes_value(true)
                        .help("Download specific video quality."),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .value_name("OUTPUT_DIR")
                        .takes_value(true)
                        .default_value(".")
                        .help("Directory where to output downloaded files (default to .)."),
                ),
        )
        .get_matches();

    let verbose = matches.is_present("verbose");
    let url = matches.value_of("url").unwrap();
    let access_token = matches.value_of("access_token");
    let username = matches.value_of("username");
    let password = matches.value_of("password");

    let fs_helper = UdemyFsHelper {};
    let udemy_helper = UdemyHelper::new(&fs_helper);
    let client = UdemyHttpClient::new();
    let auth = match access_token {
        Some(access_token) => Auth::with_token(access_token),
        None => Auth::with_username_password(username.unwrap(), password.unwrap()),
    };
    let parser = UdemyParser::new();

    let command: Option<Box<Command>> = match matches.subcommand() {
        ("info", Some(_)) => {
            if verbose {
                println!(
                    "Request information from {}",
                    matches.value_of("url").unwrap()
                );
            }

            let mut info = Info::new();
            info.set_params(&InfoParams { verbose });
            Some(Box::new(info))
        }
        ("download", Some(sub_m)) => {
            // println!("Downloading from {}", matches.value_of("url").unwrap());
            let wanted_chapter = sub_m
                .value_of("chapter")
                .and_then(|v| v.parse::<ObjectIndex>().ok());
            let wanted_lecture = sub_m
                .value_of("lecture")
                .and_then(|v| v.parse::<LectureId>().ok());
            let wanted_quality = sub_m
                .value_of("quality")
                .and_then(|v| v.parse::<VideoQuality>().ok());
            let dry_run = sub_m.is_present("dry-run");
            let output = sub_m.value_of("output").unwrap();

            let mut download = Download::new();
            download.set_params(&DownloadParams {
                wanted_chapter,
                wanted_lecture,
                wanted_quality,
                dry_run,
                verbose,
                output: output.into(),
            });
            Some(Box::new(download))
        }
        ("complete", Some(sub_m)) => {
            // println!("Downloading from {}", matches.value_of("url").unwrap());
            let wanted_chapter = sub_m
                .value_of("chapter")
                .and_then(|v| v.parse::<ObjectIndex>().ok())
                .unwrap();
            let wanted_lecture = sub_m
                .value_of("lecture")
                .and_then(|v| v.parse::<LectureId>().ok());

            let mut complete = Complete::new();
            complete.set_params(&CompleteParams {
                wanted_chapter,
                wanted_lecture,
                verbose,
            });
            Some(Box::new(complete))
        }
        _ => None,
    };

    let result: Result<(), Error> = match command {
        Some(command) => {
            if verbose {
                println!(
                    "Request information from {}",
                    matches.value_of("url").unwrap()
                );
            }

            let mut context =
                CommandContext::new(url, &client, &parser, &udemy_helper, auth).unwrap();
            let mut downloader = UdemyDownloader::new(&mut context);

            downloader
                .authenticate()
                .and_then(|_| downloader.prepare_course_info(verbose))
                .and_then(|_| downloader.execute(&*command))
        }
        None => Err(format_err!("Not a valid command")),
    };

    if let Err(err) = result {
        eprintln!("An error Occured: {}", err);
    }
}
