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
use parser::UdemyParser;
use udemy_helper::UdemyHelper;

fn main() {
    let matches = App::new("Udemy Downloader")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Bernard Niset")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
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
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("client_id")
                .short("c")
                .long("client-id")
                .value_name("CLIENT_ID")
                .help("Client id to authenticate to udemy")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(SubCommand::with_name("info").about("Query course information"))
        .subcommand(
            SubCommand::with_name("download")
                .about("Download course content")
                .arg(
                    Arg::with_name("dry-run")
                        .short("d")
                        .long("dry-run")
                        .takes_value(false)
                        .help("Dry run, show what's would be done but don't download anything"),
                )
                .arg(
                    Arg::with_name("chapter")
                        .short("c")
                        .long("chapter")
                        .takes_value(true)
                        .value_name("CHAPTER")
                        .help("Restrict downloads to a specific chapter"),
                )
                .arg(
                    Arg::with_name("lecture")
                        .short("l")
                        .long("lecture")
                        .value_name("LECTURE")
                        .takes_value(true)
                        .help("Restrict download to a specific lecture"),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .value_name("OUTPUT_DIR")
                        .takes_value(true)
                        .default_value(".")
                        .help("Directory where to output downloaded files (default to .)"),
                ),
        )
        .get_matches();

    let verbose = matches.is_present("verbose");
    let url = matches.value_of("url").unwrap();
    let access_token = matches.value_of("access_token").unwrap();
    let client_id = matches.value_of("client_id").unwrap();

    let fs_helper = UdemyFsHelper {};
    let udemy_helper = UdemyHelper::new(&fs_helper);
    let client = UdemyHttpClient::new(access_token, client_id);
    let parser = UdemyParser::new();
    let udemy_downloader = UdemyDownloader::new(url, &client, &parser, &udemy_helper).unwrap();

    let result: Result<(), Error> = match matches.subcommand() {
        ("info", Some(_sub_m)) => {
            println!(
                "Request information from {}",
                matches.value_of("url").unwrap()
            );
            udemy_downloader.info().map(|_r| ())
        }
        ("download", Some(sub_m)) => {
            println!("Downloading from {}", matches.value_of("url").unwrap());
            let wanted_chapter = sub_m
                .value_of("chapter")
                .map(|v| v.parse::<u64>().ok().unwrap_or(0));
            let wanted_lecture = sub_m
                .value_of("lecture")
                .map(|v| v.parse::<u64>().ok().unwrap_or(0));
            let dry_run = sub_m.is_present("dry-run");
            let output = sub_m.value_of("output").unwrap();

            udemy_downloader.download(wanted_chapter, wanted_lecture, output, dry_run, verbose)
        }
        _ => Ok(()),
    };

    if let Err(err) = result {
        eprintln!("An error Occured: {}", err);
    }
}
