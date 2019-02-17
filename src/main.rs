#![allow(dead_code)]

use clap::{App, AppSettings, Arg, SubCommand};

use failure::Error;

mod downloader;
mod fs_helper;
mod http_client;
mod model;
mod parser;
mod test_data;
mod udemy_helper;
mod utils;

use downloader::UdemyDownloader;
use fs_helper::UdemyFsHelper;
use http_client::UdemyHttpClient;
use model::Auth;
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
        .subcommand(
            SubCommand::with_name("info")
                .about("Query course information")
                .arg(
                    Arg::with_name("save")
                        .short("s")
                        .long("save")
                        .takes_value(true)
                        .help("Saves info.json to a file"),
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
                    Arg::with_name("info")
                        .short("i")
                        .long("info")
                        .value_name("INFO_FILE")
                        .takes_value(true)
                        .help("Load course info from specified file."),
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
    let mut udemy_downloader =
        UdemyDownloader::new(url, &client, &parser, &udemy_helper, auth).unwrap();

    let result: Result<(), Error> = match matches.subcommand() {
        ("info", Some(sub_m)) => {
            if verbose {
                println!(
                    "Request information from {}",
                    matches.value_of("url").unwrap()
                );
            }
            let wanted_save = sub_m.value_of("save");

            udemy_downloader
                .authenticate()
                .and(udemy_downloader.info(verbose, wanted_save))
        }
        ("download", Some(sub_m)) => {
            // println!("Downloading from {}", matches.value_of("url").unwrap());
            let wanted_chapter = sub_m
                .value_of("chapter")
                .and_then(|v| v.parse::<u64>().ok());
            let wanted_lecture = sub_m
                .value_of("lecture")
                .and_then(|v| v.parse::<u64>().ok());
            let wanted_quality = sub_m
                .value_of("quality")
                .and_then(|v| v.parse::<u64>().ok());
            let dry_run = sub_m.is_present("dry-run");
            let output = sub_m.value_of("output").unwrap();
            let wanted_info = sub_m.value_of("info");

            udemy_downloader
                .authenticate()
                .and(udemy_downloader.download(
                    wanted_chapter,
                    wanted_lecture,
                    wanted_quality,
                    wanted_info,
                    output,
                    dry_run,
                    verbose,
                ))
        }
        _ => Ok(()),
    };

    if let Err(err) = result {
        eprintln!("An error Occured: {}", err);
    }
}
