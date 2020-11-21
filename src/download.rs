#![allow(clippy::too_many_arguments)]

use std::any::Any;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

use failure::format_err;
use indicatif::{ProgressBar, ProgressStyle};

use crate::command::*;
use crate::model::*;
use crate::result::Result;
use crate::utils::*;

pub struct DownloadParams {
    pub wanted_chapter: Option<ObjectIndex>,
    pub wanted_lecture: Option<LectureId>,
    pub wanted_quality: Option<VideoQuality>,
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

    fn execute(&self, context: &CommandContext) -> Result<()> {
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
    ) -> Result<()> {
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
        wanted_quality: Option<VideoQuality>,
    ) -> Result<String> {
        let quality = match wanted_quality {
            Some(quality) => download_urls
                .iter()
                .filter(|url| url.r#type == Some("video/mp4".into()))
                .map(|url| &url.label)
                .filter_map(|label| label.parse::<VideoQuality>().ok())
                .filter(|label| *label >= quality)
                .min(),
            None => download_urls
                .iter()
                .filter(|url| url.r#type == Some("video/mp4".into()))
                .map(|url| &url.label)
                .filter_map(|label| label.parse::<VideoQuality>().ok())
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
        wanted_lecture: Option<LectureId>,
        wanted_quality: Option<VideoQuality>,
        output: &str,
        dry_run: bool,
        verbose: bool,
    ) -> Result<()> {
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
                .filter(|lecture| {
                    wanted_lecture.is_none() || wanted_lecture.unwrap() == lecture.object_index
                })
                .filter(|lecture| lecture.has_video)
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
        wanted_quality: Option<VideoQuality>,
        path: &str,
        dry_run: bool,
        verbose: bool,
    ) -> Result<()> {
        let target_filename = context
            .udemy_helper
            .calculate_target_filename(path, &lecture)
            .unwrap();
        let url = format!(
            "https://{portal_name}.udemy.com/api-2.0/users/me/subscribed-courses/{course_id}/lectures/{lecture_id}?fields[asset]=@min,download_urls,external_url,slide_urls,status,captions,thumbnail_url,time_estimation,stream_urls&fields[caption]=@default,is_translation&fields[course]=id,url,locale&fields[lecture]=@default,course,can_give_cc_feedback,download_url",
            portal_name = context.portal_name,
            course_id = context.course.as_ref().unwrap().id, lecture_id=lecture.id
        );

        let lecture_detail = context.client.get_as_json(url.as_str(), &context.auth)?;
        let lecture_detail = context.parser.parse_lecture_detail(&lecture_detail)?;
        if let Some(download_urls) = &lecture_detail.asset.download_urls {
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
        wanted_chapter: Option<ObjectIndex>,
        wanted_lecture: Option<LectureId>,
        wanted_quality: Option<VideoQuality>,
        output: &str,
        dry_run: bool,
        verbose: bool,
    ) -> Result<()> {
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

#[cfg(test)]
mod test {
    use super::*;

    use crate::mocks::test::*;
    use crate::test_data::test::*;
    use crate::udemy_helper::UdemyHelper;

    #[test]
    fn download() {
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

        let mut download = Download::new();
        download.set_params(&DownloadParams {
            wanted_chapter: Some(1),
            wanted_lecture: Some(1),
            wanted_quality: None,
            output: "~/Downloads".into(),
            dry_run: false,
            verbose: false,
        });

        let result = download.execute(&context);

        assert!(result.is_ok());
        unsafe {
            if let Some(ref gcl) = GETS_CONTENT_LENGTH_URL {
                assert_eq!(gcl.len(), 1);
                assert_eq!(gcl[0], "http://host-name/the-filename.mp4");
            }
            if let Some(ref gad) = GETS_AS_DATA_URL {
                assert_eq!(gad.len(), 1);
                assert_eq!(gad[0], "http://host-name/the-filename.mp4");
            }
        }
    }

    #[test]
    fn determine_quality_for_best() {
        let download_urls = vec![
            DownloadUrl {
                label: "480".into(),
                file: "the-file-video-480".into(),
                r#type: Some("video/mp4".into()),
            },
            DownloadUrl {
                label: "720".into(),
                file: "the-file-video-720".into(),
                r#type: Some("video/mp4".into()),
            },
            DownloadUrl {
                label: "1720".into(),
                file: "the-file-720".into(),
                r#type: Some("other/mp4".into()),
            },
        ];
        let wanted_quality = None;

        let mut download = Download::new();
        download.set_params(&DownloadParams {
            wanted_chapter: Some(1),
            wanted_lecture: Some(1),
            wanted_quality: None,
            output: "~/Downloads".into(),
            dry_run: false,
            verbose: false,
        });

        let actual = download.determine_quality(&download_urls, wanted_quality);

        assert_eq!(actual.is_ok(), true);
        assert_eq!(actual.unwrap(), "720");
    }

    #[test]
    fn determine_quality_for_wanted_480() {
        let download_urls = vec![
            DownloadUrl {
                label: "480".into(),
                file: "the-file-video-480".into(),
                r#type: Some("video/mp4".into()),
            },
            DownloadUrl {
                label: "360".into(),
                file: "the-file-video-360".into(),
                r#type: Some("video/mp4".into()),
            },
            DownloadUrl {
                label: "720".into(),
                file: "the-file-video-720".into(),
                r#type: Some("video/mp4".into()),
            },
            DownloadUrl {
                label: "1720".into(),
                file: "the-file-720".into(),
                r#type: Some("other/mp4".into()),
            },
        ];
        let wanted_quality = Some(480u64);
        let mut download = Download::new();
        download.set_params(&DownloadParams {
            wanted_chapter: Some(1),
            wanted_lecture: Some(1),
            wanted_quality: None,
            output: "~/Downloads".into(),
            dry_run: false,
            verbose: false,
        });

        let actual = download.determine_quality(&download_urls, wanted_quality);

        assert_eq!(actual.is_ok(), true);
        assert_eq!(actual.unwrap(), "480");
    }

    #[test]
    fn determine_quality_for_wanted_470() {
        let download_urls = vec![
            DownloadUrl {
                label: "480".into(),
                file: "the-file-video-480".into(),
                r#type: Some("video/mp4".into()),
            },
            DownloadUrl {
                label: "360".into(),
                file: "the-file-video-360".into(),
                r#type: Some("video/mp4".into()),
            },
            DownloadUrl {
                label: "720".into(),
                file: "the-file-video-720".into(),
                r#type: Some("video/mp4".into()),
            },
            DownloadUrl {
                label: "1720".into(),
                file: "the-file-720".into(),
                r#type: Some("other/mp4".into()),
            },
        ];
        let wanted_quality = Some(470u64);
        let mut download = Download::new();
        download.set_params(&DownloadParams {
            wanted_chapter: Some(1),
            wanted_lecture: Some(1),
            wanted_quality: None,
            output: "~/Downloads".into(),
            dry_run: false,
            verbose: false,
        });

        let actual = download.determine_quality(&download_urls, wanted_quality);

        assert_eq!(actual.is_ok(), true);
        assert_eq!(actual.unwrap(), "480");
    }
}
