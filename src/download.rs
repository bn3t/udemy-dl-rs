use std::any::Any;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

use failure::{format_err, Error};
use indicatif::{ProgressBar, ProgressStyle};

use crate::command::*;
use crate::model::*;
use crate::utils::*;

pub struct DownloadParams {
    pub wanted_chapter: Option<u64>,
    pub wanted_lecture: Option<u64>,
    pub wanted_quality: Option<u64>,
    pub output: String,
    pub dry_run: bool,
    pub verbose: bool,
}

/// Download files to a specified location. It is possible to specify
/// which chapter / lecture to download.
pub struct Download {
    params: Option<DownloadParams>,
}

impl Download {
    pub fn new() -> Download {
        Download { params: None }
    }
}

impl Command for Download {
    fn set_params(&mut self, params: &dyn Any) {
        if let Some(params) = params.downcast_ref::<DownloadParams>() {
            self.params = Some(DownloadParams {
                wanted_chapter: params.wanted_chapter,
                wanted_lecture: params.wanted_lecture,
                wanted_quality: params.wanted_quality,
                output: params.output.clone(),
                dry_run: params.dry_run,
                verbose: params.verbose,
            });
        }
    }

    fn execute(&self, context: &CommandContext) -> Result<(), Error> {
        // self.print_course_content(&context.course_content.unwrap());
        if let Some(params) = self.params.as_ref() {
            self.download(
                context,
                params.wanted_chapter,
                params.wanted_lecture,
                params.wanted_quality,
                params.output.as_str(),
                params.dry_run,
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

impl Download {
    fn download_url(
        &self,
        context: &CommandContext,
        lecture_title: &str,
        url: &str,
        target_filename: &str,
    ) -> Result<(), Error> {
        let content_length = context.client.get_content_length(url)?;
        let start = Instant::now();

        let pb = ProgressBar::new(content_length);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta}) ({msg})")
                .progress_chars("#>-"),
        );
        pb.set_message(lecture_title);
        let buf = context.client.get_as_data(url, &mut |size| {
            pb.set_position(size);
        })?;
        let mut file = File::create(target_filename)?;
        let _size = file.write(&buf)?;
        let elapsed = Instant::now().duration_since(start);
        let elapsed = elapsed.as_secs() * 1000u64 + u64::from(elapsed.subsec_millis());
        pb.finish_with_message(
            format!(
                "{:1.2} MB/s",
                calculate_download_speed(content_length, elapsed)
            )
            .as_str(),
        );
        Ok(())
    }

    fn determine_quality(
        &self,
        download_urls: &[DownloadUrl],
        wanted_quality: Option<u64>,
    ) -> Result<String, Error> {
        let quality = match wanted_quality {
            Some(quality) => download_urls
                .iter()
                .filter(|url| url.r#type == Some("video/mp4".into()))
                .map(|url| &url.label)
                .filter_map(|label| label.parse::<u64>().ok())
                .filter(|label| *label >= quality)
                .min(),
            None => download_urls
                .iter()
                .filter(|url| url.r#type == Some("video/mp4".into()))
                .map(|url| &url.label)
                .filter_map(|label| label.parse::<u64>().ok())
                .max(),
        };
        let quality = quality
            .map(|q| q.to_string())
            .ok_or_else(|| format_err!("No best quality could be found"))?;
        Ok(quality)
    }

    fn download_chapter(
        &self,
        context: &CommandContext,
        chapter: &Chapter,
        wanted_lecture: Option<u64>,
        wanted_quality: Option<u64>,
        output: &str,
        dry_run: bool,
        verbose: bool,
    ) -> Result<(), Error> {
        if verbose {
            println!(
                "Downloading chapter {} - {}",
                chapter.object_index, chapter.title
            );
        }
        let chapter_path = context
            .udemy_helper
            .calculate_target_dir(output, &chapter, context.course_name.as_str())
            .unwrap();
        if context
            .udemy_helper
            .create_target_dir(chapter_path.as_str())
            .is_ok()
        {
            chapter
                .lectures
                .iter()
                .filter(|lecture| lecture.asset.asset_type == "Video")
                .filter(|lecture| {
                    wanted_lecture.is_none() || wanted_lecture.unwrap() == lecture.object_index
                })
                .for_each(move |lecture| {
                    match self.download_lecture(
                        context,
                        &lecture,
                        wanted_quality,
                        chapter_path.as_str(),
                        dry_run,
                        verbose,
                    ) {
                        Ok(_) => {
                            // if verbose {
                            //     println!("Lecture downloaded");
                            // }
                        }
                        Err(e) => {
                            eprintln!("Error while saving {}: {}", lecture.title, e);
                        }
                    };
                });
        }
        Ok(())
    }

    fn download_lecture(
        &self,
        context: &CommandContext,
        lecture: &Lecture,
        wanted_quality: Option<u64>,
        path: &str,
        dry_run: bool,
        verbose: bool,
    ) -> Result<(), Error> {
        let target_filename = context
            .udemy_helper
            .calculate_target_filename(path, &lecture)
            .unwrap();
        if let Some(download_urls) = &lecture.asset.download_urls {
            let best_quality = self.determine_quality(&download_urls, wanted_quality)?;
            for url in download_urls {
                if let Some(video_type) = &url.r#type {
                    if url.label == best_quality && video_type == "video/mp4" {
                        if verbose {
                            println!("\tGetting ({}) {}", url.label, url.file);
                            println!("\t\t-> {}", target_filename);
                        }
                        if !dry_run {
                            self.download_url(
                                context,
                                lecture.title.as_str(),
                                url.file.as_str(),
                                target_filename.as_str(),
                            )?
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn download(
        &self,
        context: &CommandContext,
        wanted_chapter: Option<u64>,
        wanted_lecture: Option<u64>,
        wanted_quality: Option<u64>,
        output: &str,
        dry_run: bool,
        verbose: bool,
    ) -> Result<(), Error> {
        if verbose {
            println!(
                "Download request chapter: {:?}, lecture: {:?}, quality: {:?}, dry_run: {}",
                wanted_chapter, wanted_lecture, wanted_quality, dry_run
            );
        }

        for chapter in context.course_content.as_ref().unwrap().chapters.iter() {
            if wanted_chapter.is_none() || wanted_chapter.unwrap() == chapter.object_index {
                self.download_chapter(
                    context,
                    &chapter,
                    wanted_lecture,
                    wanted_quality,
                    output,
                    dry_run,
                    verbose,
                )?;
            }
        }

        Ok(())
    }
}
