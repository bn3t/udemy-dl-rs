use failure::format_err;

mod command;
mod complete;
mod download;
mod downloader;
mod fs_helper;
mod http_client;
mod info;
mod mocks;
mod model;
mod options;
mod parser;
mod result;
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
use result::Result;
use udemy_helper::UdemyHelper;

fn main() {
    let udemy_cmd = options::parse_options();

    let verbose = udemy_cmd.verbose;
    let url = udemy_cmd.url;
    let access_token = udemy_cmd.access_token;

    let fs_helper = UdemyFsHelper {};
    let udemy_helper = UdemyHelper::new(&fs_helper);
    let client = UdemyHttpClient::new();
    let auth = Auth::with_token(access_token.as_str());
    let parser = UdemyParser::new();

    let command: Option<Box<dyn Command>> = match udemy_cmd.command {
        options::Command::Info => {
            if verbose {
                println!("Request information from {}", url);
            }

            let mut info = Info::new();
            info.set_params(&InfoParams { verbose });
            Some(Box::new(info))
        }
        options::Command::Download {
            dry_run,
            chapter,
            lecture,
            quality,
            output,
        } => {
            println!("Downloading from {}", url);

            let mut download = Download::new();
            download.set_params(&DownloadParams {
                wanted_chapter: chapter,
                wanted_lecture: lecture,
                wanted_quality: quality,
                output: output.into(),
                verbose,
                dry_run,
            });
            Some(Box::new(download))
        }
        _ => None,
    };

    let result: Result<()> = match command {
        Some(command) => {
            if verbose {
                println!("Request information from {}", url);
            }

            let mut context =
                CommandContext::new(url.as_str(), &client, &parser, &udemy_helper, auth).unwrap();
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
