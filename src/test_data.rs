use crate::model::*;

pub fn make_course() -> Course {
  Course {
    id: 54321,
    url: "the-url".into(),
    published_title: "css-the-complete-guide-incl-flexbox-grid-sass".into(),
  }
}

pub fn make_test_course_content() -> CourseContent {
  CourseContent {
    chapters: vec![Chapter {
      object_index: 1,
      title: "The Chapter".into(),
      lectures: vec![Lecture {
        id: 4321,
        object_index: 1,
        title: "The Lecture".into(),
        asset: Asset {
          filename: "the-filename.mp4".into(),
          asset_type: "Video".into(),
          time_estimation: 321,
          download_urls: Some(vec![DownloadUrl {
            r#type: Some("video/mp4".into()),
            file: "http://host-name/the-filename.mp4".into(),
            label: "720".into(),
          }]),
        },
        supplementary_assets: vec![],
      }],
    }],
  }
}

pub const TEST_SUBSCRIBED_COURSES: &str = r#"{
      "count": 13,
      "next": null,
      "previous": null,
      "results": [
        {
          "_class": "course",
          "id": 1561458,
          "url": "/css-the-complete-guide-incl-flexbox-grid-sass/learn/v4/",
          "published_title": "css-the-complete-guide-incl-flexbox-grid-sass"
        },
        {
          "_class": "course",
          "id": 995016,
          "url": "/vuejs-2-the-complete-guide/learn/v4/",
          "published_title": "vuejs-2-the-complete-guide"
        },
        {
          "_class": "course",
          "id": 1362070,
          "url": "/react-the-complete-guide-incl-redux/learn/v4/",
          "published_title": "react-the-complete-guide-incl-redux"
        }
      ],
      "aggregations": null
    }"#;

pub const TEST_SUP_ASSETS: &str = r#"[
        {
          "body": "",
          "stream_urls": "",
          "filename": "css-in-action.zip",
          "time_estimation": 0,
          "slide_urls": [],
          "download_urls": {
            "File": [
              {
                "file": "https://udemy-assets-on-demand2.udemy.com/2018-03-19_09-26-58-99eef8df19e7a36d975a4c78e6307a2b/original.zip?nva=20190204223948&filename=css-in-action.zip&download=True&token=08102139951046a068948",
                "label": "download"
              }
            ]
          },
          "asset_type": "File",
          "_class": "asset",
          "id": 11743084,
          "external_url": ""
        }
      ]"#;

pub const TEST_ASSET: &str = r#"{
        "body": "",
        "filename": "getting-started-01-welcome.mp4",
        "stream_urls": {
          "Video": [
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand2.udemy.com/2018-03-16_18-03-45-cb7a7f9f7ce092310d2ba43b50b0d2b8/WebHD_720p.mp4?nva=20190204223948&token=04bb9b4a0a10a29605c40",
              "label": "720"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand2.udemy.com/2018-03-16_18-03-45-cb7a7f9f7ce092310d2ba43b50b0d2b8/WebHD_480.mp4?nva=20190204223948&token=085495f55793206b7e605",
              "label": "480"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand2.udemy.com/2018-03-16_18-03-45-cb7a7f9f7ce092310d2ba43b50b0d2b8/WebHD.mp4?nva=20190204223948&token=0e2e9f8a3a371bed4e444",
              "label": "360"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand.udemy.com/2018-03-16_18-03-45-cb7a7f9f7ce092310d2ba43b50b0d2b8/Web_144.mp4?nva=20190204223948&token=0e02a605a7b4f8ba32689",
              "label": "144"
            },
            {
              "type": "application/x-mpegURL",
              "file": "HTTPS://adaptive-streaming.udemy.com/1561458/11715366/2018-03-16_18-03-45-cb7a7f9f7ce092310d2ba43b50b0d2b8/aa005d8deaea759234a5881219ba51f6fa82.m3u8?nva=1549319988&ttl=16200&ip=None&token=0979767d9dfcb0233a814",
              "label": "Auto"
            }
          ]
        },
        "asset_type": "Video",
        "time_estimation": 99,
        "slide_urls": [],
        "download_urls": {
          "Video": [
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand2.udemy.com/2018-03-16_18-03-45-cb7a7f9f7ce092310d2ba43b50b0d2b8/WebHD_720p.mp4?nva=20190204223948&filename=getting-started-01-welcome.mp4&download=True&token=068ae457bbe97231de938",
              "label": "720"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand2.udemy.com/2018-03-16_18-03-45-cb7a7f9f7ce092310d2ba43b50b0d2b8/WebHD_480.mp4?nva=20190204223948&filename=getting-started-01-welcome.mp4&download=True&token=02914f4f323d7d099c25c",
              "label": "480"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand2.udemy.com/2018-03-16_18-03-45-cb7a7f9f7ce092310d2ba43b50b0d2b8/WebHD.mp4?nva=20190204223948&filename=getting-started-01-welcome.mp4&download=True&token=017b2032fc384d4648a2e",
              "label": "360"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand.udemy.com/2018-03-16_18-03-45-cb7a7f9f7ce092310d2ba43b50b0d2b8/Web_144.mp4?nva=20190204223948&filename=getting-started-01-welcome.mp4&download=True&token=04de75ff83dac62b12705",
              "label": "144"
            }
          ]
        },
        "captions": [
          {
            "status": 1,
            "created": "2018-07-03T10:26:54Z",
            "locale": {
              "locale": "en_US",
              "_class": "locale"
            },
            "file_name": "d2f78d68-35b0-4da6-847e-c92eec99bc8c.vtt",
            "title": "introductionautogenerated.vtt",
            "video_label": "English",
            "url": "https://udemy-captions.s3.amazonaws.com:443/11715366/en_US/d2f78d68-35b0-4da6-847e-c92eec99bc8c.vtt?Signature=Q5jBBGmoUXwWWMcPZ1K3Mucqr2g%3D&Expires=1549318188&AWSAccessKeyId=AKIAJA6MCXJVONCBES7A&response-content-disposition=attachment%3B%20filename%3D%22introductionautogenerated.vtt%22",
            "source": "manual",
            "_class": "caption",
            "id": 5581306
          },
          {
            "status": 1,
            "created": "2018-04-24T20:03:28Z",
            "locale": {
              "locale": "es_ES",
              "_class": "locale"
            },
            "file_name": "2018-04-24_20-03-28-60e48a0294c7b9d55670147a16d173a3.vtt",
            "title": "es_ES.introduction.autogenerated.vtt",
            "video_label": "Spanish [Auto]",
            "url": "https://udemy-captions.s3.amazonaws.com:443/11715366/es_ES/2018-04-24_20-03-28-60e48a0294c7b9d55670147a16d173a3.vtt?Signature=WEAgiizlBAGqXaymiQCgJHm94Yk%3D&Expires=1549318188&AWSAccessKeyId=AKIAJA6MCXJVONCBES7A&response-content-disposition=attachment%3B%20filename%3D%22es_esintroductionautogenerated.vtt%22",
            "source": "auto",
            "_class": "caption",
            "id": 4918390
          },
          {
            "status": 1,
            "created": "2018-04-24T11:06:24Z",
            "locale": {
              "locale": "pt_BR",
              "_class": "locale"
            },
            "file_name": "2018-04-24_11-06-24-dc3b869e5e089ea2e53005cb3e54963d.vtt",
            "title": "pt_PT.introduction.autogenerated.vtt",
            "video_label": "Portuguese [Auto]",
            "url": "https://udemy-captions.s3.amazonaws.com:443/11715366/pt_BR/2018-04-24_11-06-24-dc3b869e5e089ea2e53005cb3e54963d.vtt?Signature=sMS5Wg%2FNgqa6Fvjb%2BVNysnyLVFQ%3D&Expires=1549318188&AWSAccessKeyId=AKIAJA6MCXJVONCBES7A&response-content-disposition=attachment%3B%20filename%3D%22pt_ptintroductionautogenerated.vtt%22",
            "source": "auto",
            "_class": "caption",
            "id": 4901360
          }
        ],
        "_class": "asset",
        "id": 11715366,
        "external_url": ""
      }"#;

pub const TEST_FULL_COURSE: &str = r#"{
  "count": 327,
  "previous": "",
  "results": [
    {
      "object_index": 1,
      "_class": "chapter",
      "sort_order": 337,
      "id": 2274098,
      "title": "Getting Started"
    },
    {
      "title": "Introduction",
      "object_index": 1,
      "asset": {
        "body": "",
        "filename": "getting-started-01-welcome.mp4",
        "stream_urls": {
          "Video": [
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand2.udemy.com/2018-03-16_18-03-45-cb7a7f9f7ce092310d2ba43b50b0d2b8/WebHD_720p.mp4?nva=20190204223948&token=04bb9b4a0a10a29605c40",
              "label": "720"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand2.udemy.com/2018-03-16_18-03-45-cb7a7f9f7ce092310d2ba43b50b0d2b8/WebHD_480.mp4?nva=20190204223948&token=085495f55793206b7e605",
              "label": "480"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand2.udemy.com/2018-03-16_18-03-45-cb7a7f9f7ce092310d2ba43b50b0d2b8/WebHD.mp4?nva=20190204223948&token=0e2e9f8a3a371bed4e444",
              "label": "360"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand.udemy.com/2018-03-16_18-03-45-cb7a7f9f7ce092310d2ba43b50b0d2b8/Web_144.mp4?nva=20190204223948&token=0e02a605a7b4f8ba32689",
              "label": "144"
            },
            {
              "type": "application/x-mpegURL",
              "file": "HTTPS://adaptive-streaming.udemy.com/1561458/11715366/2018-03-16_18-03-45-cb7a7f9f7ce092310d2ba43b50b0d2b8/aa005d8deaea759234a5881219ba51f6fa82.m3u8?nva=1549319988&ttl=16200&ip=None&token=0979767d9dfcb0233a814",
              "label": "Auto"
            }
          ]
        },
        "asset_type": "Video",
        "time_estimation": 99,
        "slide_urls": [],
        "download_urls": {
          "Video": [
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand2.udemy.com/2018-03-16_18-03-45-cb7a7f9f7ce092310d2ba43b50b0d2b8/WebHD_720p.mp4?nva=20190204223948&filename=getting-started-01-welcome.mp4&download=True&token=068ae457bbe97231de938",
              "label": "720"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand2.udemy.com/2018-03-16_18-03-45-cb7a7f9f7ce092310d2ba43b50b0d2b8/WebHD_480.mp4?nva=20190204223948&filename=getting-started-01-welcome.mp4&download=True&token=02914f4f323d7d099c25c",
              "label": "480"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand2.udemy.com/2018-03-16_18-03-45-cb7a7f9f7ce092310d2ba43b50b0d2b8/WebHD.mp4?nva=20190204223948&filename=getting-started-01-welcome.mp4&download=True&token=017b2032fc384d4648a2e",
              "label": "360"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand.udemy.com/2018-03-16_18-03-45-cb7a7f9f7ce092310d2ba43b50b0d2b8/Web_144.mp4?nva=20190204223948&filename=getting-started-01-welcome.mp4&download=True&token=04de75ff83dac62b12705",
              "label": "144"
            }
          ]
        },
        "captions": [
          {
            "status": 1,
            "created": "2018-07-03T10:26:54Z",
            "locale": {
              "locale": "en_US",
              "_class": "locale"
            },
            "file_name": "d2f78d68-35b0-4da6-847e-c92eec99bc8c.vtt",
            "title": "introductionautogenerated.vtt",
            "video_label": "English",
            "url": "https://udemy-captions.s3.amazonaws.com:443/11715366/en_US/d2f78d68-35b0-4da6-847e-c92eec99bc8c.vtt?Signature=Q5jBBGmoUXwWWMcPZ1K3Mucqr2g%3D&Expires=1549318188&AWSAccessKeyId=AKIAJA6MCXJVONCBES7A&response-content-disposition=attachment%3B%20filename%3D%22introductionautogenerated.vtt%22",
            "source": "manual",
            "_class": "caption",
            "id": 5581306
          },
          {
            "status": 1,
            "created": "2018-04-24T20:03:28Z",
            "locale": {
              "locale": "es_ES",
              "_class": "locale"
            },
            "file_name": "2018-04-24_20-03-28-60e48a0294c7b9d55670147a16d173a3.vtt",
            "title": "es_ES.introduction.autogenerated.vtt",
            "video_label": "Spanish [Auto]",
            "url": "https://udemy-captions.s3.amazonaws.com:443/11715366/es_ES/2018-04-24_20-03-28-60e48a0294c7b9d55670147a16d173a3.vtt?Signature=WEAgiizlBAGqXaymiQCgJHm94Yk%3D&Expires=1549318188&AWSAccessKeyId=AKIAJA6MCXJVONCBES7A&response-content-disposition=attachment%3B%20filename%3D%22es_esintroductionautogenerated.vtt%22",
            "source": "auto",
            "_class": "caption",
            "id": 4918390
          },
          {
            "status": 1,
            "created": "2018-04-24T11:06:24Z",
            "locale": {
              "locale": "pt_BR",
              "_class": "locale"
            },
            "file_name": "2018-04-24_11-06-24-dc3b869e5e089ea2e53005cb3e54963d.vtt",
            "title": "pt_PT.introduction.autogenerated.vtt",
            "video_label": "Portuguese [Auto]",
            "url": "https://udemy-captions.s3.amazonaws.com:443/11715366/pt_BR/2018-04-24_11-06-24-dc3b869e5e089ea2e53005cb3e54963d.vtt?Signature=sMS5Wg%2FNgqa6Fvjb%2BVNysnyLVFQ%3D&Expires=1549318188&AWSAccessKeyId=AKIAJA6MCXJVONCBES7A&response-content-disposition=attachment%3B%20filename%3D%22pt_ptintroductionautogenerated.vtt%22",
            "source": "auto",
            "_class": "caption",
            "id": 4901360
          }
        ],
        "_class": "asset",
        "id": 11715366,
        "external_url": ""
      },
      "_class": "lecture",
      "id": 9429312,
      "supplementary_assets": []
    },
    {
      "title": "What is CSS?",
      "object_index": 2,
      "asset": {
        "body": "",
        "filename": "getting-started-02-what-is-css.mp4",
        "stream_urls": {
          "Video": [
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand2.udemy.com/2018-03-16_18-03-45-955186a1e66901e0ce9a98ea29d58460/WebHD_720p.mp4?nva=20190204223948&token=067b9d13d61b62db82ecf",
              "label": "720"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand2.udemy.com/2018-03-16_18-03-45-955186a1e66901e0ce9a98ea29d58460/WebHD_480.mp4?nva=20190204223948&token=09727707cbb7ce7ea0137",
              "label": "480"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand2.udemy.com/2018-03-16_18-03-45-955186a1e66901e0ce9a98ea29d58460/WebHD.mp4?nva=20190204223948&token=08e4109865d9ab72fb8a6",
              "label": "360"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand.udemy.com/2018-03-16_18-03-45-955186a1e66901e0ce9a98ea29d58460/Web_144.mp4?nva=20190204223948&token=00633e7271b548f2600d0",
              "label": "144"
            },
            {
              "type": "application/x-mpegURL",
              "file": "HTTPS://adaptive-streaming.udemy.com/1561458/11715370/2018-03-16_18-03-45-955186a1e66901e0ce9a98ea29d58460/aa00d7c98b49caecbaef0bc98a964b500fb7.m3u8?nva=1549319988&ttl=16200&ip=None&token=0979767d9dfcb0233a814",
              "label": "Auto"
            }
          ]
        },
        "asset_type": "Video",
        "time_estimation": 179,
        "slide_urls": [],
        "download_urls": {
          "Video": [
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand2.udemy.com/2018-03-16_18-03-45-955186a1e66901e0ce9a98ea29d58460/WebHD_720p.mp4?nva=20190204223948&filename=getting-started-02-what-is-css.mp4&download=True&token=0e1ab7e4e0e3ceae8a967",
              "label": "720"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand2.udemy.com/2018-03-16_18-03-45-955186a1e66901e0ce9a98ea29d58460/WebHD_480.mp4?nva=20190204223948&filename=getting-started-02-what-is-css.mp4&download=True&token=0cc947cf9f36bd3a23a9a",
              "label": "480"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand2.udemy.com/2018-03-16_18-03-45-955186a1e66901e0ce9a98ea29d58460/WebHD.mp4?nva=20190204223948&filename=getting-started-02-what-is-css.mp4&download=True&token=00f82f43ff787efcc80b8",
              "label": "360"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand.udemy.com/2018-03-16_18-03-45-955186a1e66901e0ce9a98ea29d58460/Web_144.mp4?nva=20190204223948&filename=getting-started-02-what-is-css.mp4&download=True&token=00c7edc1eaeb99b3c5192",
              "label": "144"
            }
          ]
        },
        "captions": [
          {
            "status": 1,
            "created": "2018-07-03T10:10:52Z",
            "locale": {
              "locale": "en_US",
              "_class": "locale"
            },
            "file_name": "2a2150e3-41ad-4cfd-b500-3ffb5aefa3a8.vtt",
            "title": "what-is-cssautogenerated.vtt",
            "video_label": "English",
            "url": "https://udemy-captions.s3.amazonaws.com:443/11715370/en_US/2a2150e3-41ad-4cfd-b500-3ffb5aefa3a8.vtt?Signature=C1p27zM41yHKfM70VVUD%2BTGPCgE%3D&Expires=1549318188&AWSAccessKeyId=AKIAJA6MCXJVONCBES7A&response-content-disposition=attachment%3B%20filename%3D%22what-is-cssautogenerated.vtt%22",
            "source": "manual",
            "_class": "caption",
            "id": 5581242
          },
          {
            "status": 1,
            "created": "2018-04-24T20:03:28Z",
            "locale": {
              "locale": "es_ES",
              "_class": "locale"
            },
            "file_name": "2018-04-24_20-03-28-d26563bec2a29bd912b9ccb04cf03caf.vtt",
            "title": "es_ES.what-is-css.autogenerated.vtt",
            "video_label": "Spanish [Auto]",
            "url": "https://udemy-captions.s3.amazonaws.com:443/11715370/es_ES/2018-04-24_20-03-28-d26563bec2a29bd912b9ccb04cf03caf.vtt?Signature=cF%2FpVfkaiYqa3qSHXK0pnfbPx5A%3D&Expires=1549318188&AWSAccessKeyId=AKIAJA6MCXJVONCBES7A&response-content-disposition=attachment%3B%20filename%3D%22es_eswhat-is-cssautogenerated.vtt%22",
            "source": "auto",
            "_class": "caption",
            "id": 4918392
          },
          {
            "status": 1,
            "created": "2018-04-24T11:06:24Z",
            "locale": {
              "locale": "pt_BR",
              "_class": "locale"
            },
            "file_name": "2018-04-24_11-06-24-e18a54629dd82f09da3f3d7b280e7d04.vtt",
            "title": "pt_PT.what-is-css.autogenerated.vtt",
            "video_label": "Portuguese [Auto]",
            "url": "https://udemy-captions.s3.amazonaws.com:443/11715370/pt_BR/2018-04-24_11-06-24-e18a54629dd82f09da3f3d7b280e7d04.vtt?Signature=EiL4SuPAxEVk4ef6wOhI6Hv21xg%3D&Expires=1549318188&AWSAccessKeyId=AKIAJA6MCXJVONCBES7A&response-content-disposition=attachment%3B%20filename%3D%22pt_ptwhat-is-cssautogenerated.vtt%22",
            "source": "auto",
            "_class": "caption",
            "id": 4901362
          }
        ],
        "_class": "asset",
        "id": 11715370,
        "external_url": ""
      },
      "_class": "lecture",
      "id": 9429320,
      "supplementary_assets": [
        {
          "body": "",
          "stream_urls": "",
          "filename": "css-in-action.zip",
          "time_estimation": 0,
          "slide_urls": [],
          "download_urls": {
            "File": [
              {
                "file": "https://udemy-assets-on-demand2.udemy.com/2018-03-19_09-26-58-99eef8df19e7a36d975a4c78e6307a2b/original.zip?nva=20190204223948&filename=css-in-action.zip&download=True&token=08102139951046a068948",
                "label": "download"
              }
            ]
          },
          "asset_type": "File",
          "_class": "asset",
          "id": 11743084,
          "external_url": ""
        }
      ]
    },
    {
      "object_index": 2,
      "_class": "chapter",
      "sort_order": 326,
      "id": 2274112,
      "title": "Diving Into the Basics of CSS"
    },
    {
      "title": "Module Introduction",
      "object_index": 11,
      "asset": {
        "body": "",
        "filename": "basics-01-intro.mp4",
        "stream_urls": {
          "Video": [
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand2.udemy.com/2018-02-22_11-30-41-f0e44f49e6e9cacdf48ee25b646a11a0/WebHD_720p.mp4?nva=20190204223948&token=0c79085c78eedc0298511",
              "label": "720"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand.udemy.com/2018-02-22_11-30-41-f0e44f49e6e9cacdf48ee25b646a11a0/WebHD_480.mp4?nva=20190204223948&token=01f341de77abf888b3924",
              "label": "480"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand.udemy.com/2018-02-22_11-30-41-f0e44f49e6e9cacdf48ee25b646a11a0/WebHD.mp4?nva=20190204223948&token=0ca8e2c5cd35851050584",
              "label": "360"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand2.udemy.com/2018-02-22_11-30-41-f0e44f49e6e9cacdf48ee25b646a11a0/Web_144.mp4?nva=20190204223948&token=0fa7392adddae980f509a",
              "label": "144"
            },
            {
              "type": "application/x-mpegURL",
              "file": "HTTPS://adaptive-streaming.udemy.com/1561458/11425492/2018-02-22_11-30-41-f0e44f49e6e9cacdf48ee25b646a11a0/aa007027cbbc2238787ce43fa365529d41f8.m3u8?nva=1549319988&ttl=16200&ip=None&token=0979767d9dfcb0233a814",
              "label": "Auto"
            }
          ]
        },
        "asset_type": "Video",
        "time_estimation": 55,
        "slide_urls": [],
        "download_urls": {
          "Video": [
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand2.udemy.com/2018-02-22_11-30-41-f0e44f49e6e9cacdf48ee25b646a11a0/WebHD_720p.mp4?nva=20190204223948&filename=basics-01-intro.mp4&download=True&token=01cc1cfb2fef48a73bc3f",
              "label": "720"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand.udemy.com/2018-02-22_11-30-41-f0e44f49e6e9cacdf48ee25b646a11a0/WebHD_480.mp4?nva=20190204223948&filename=basics-01-intro.mp4&download=True&token=046fede444146e1f2a71b",
              "label": "480"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand.udemy.com/2018-02-22_11-30-41-f0e44f49e6e9cacdf48ee25b646a11a0/WebHD.mp4?nva=20190204223948&filename=basics-01-intro.mp4&download=True&token=0180fd246676f5e002c5c",
              "label": "360"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand2.udemy.com/2018-02-22_11-30-41-f0e44f49e6e9cacdf48ee25b646a11a0/Web_144.mp4?nva=20190204223948&filename=basics-01-intro.mp4&download=True&token=0794e78629353dd2f5aa2",
              "label": "144"
            }
          ]
        },
        "captions": [
          {
            "status": 1,
            "created": "2018-04-24T20:03:28Z",
            "locale": {
              "locale": "es_ES",
              "_class": "locale"
            },
            "file_name": "2018-04-24_20-03-28-c7cb409eb0d8a095bd5c738ffbd7e5f0.vtt",
            "title": "es_ES.module-introduction.autogenerated.vtt",
            "video_label": "Spanish [Auto]",
            "url": "https://udemy-captions.s3.amazonaws.com:443/11425492/es_ES/2018-04-24_20-03-28-c7cb409eb0d8a095bd5c738ffbd7e5f0.vtt?Signature=pPFPXYwgOY434t0NeM5CCtzqLiM%3D&Expires=1549318188&AWSAccessKeyId=AKIAJA6MCXJVONCBES7A&response-content-disposition=attachment%3B%20filename%3D%22es_esmodule-introductionautogenerated.vtt%22",
            "source": "auto",
            "_class": "caption",
            "id": 4918404
          },
          {
            "status": 1,
            "created": "2018-04-24T11:06:25Z",
            "locale": {
              "locale": "pt_BR",
              "_class": "locale"
            },
            "file_name": "2018-04-24_11-06-25-5cbf27b274a7bdf22da97d2117cfda82.vtt",
            "title": "pt_PT.module-introduction.autogenerated.vtt",
            "video_label": "Portuguese [Auto]",
            "url": "https://udemy-captions.s3.amazonaws.com:443/11425492/pt_BR/2018-04-24_11-06-25-5cbf27b274a7bdf22da97d2117cfda82.vtt?Signature=FReuCbb0oUb1wgngEkQ4wQvPySA%3D&Expires=1549318188&AWSAccessKeyId=AKIAJA6MCXJVONCBES7A&response-content-disposition=attachment%3B%20filename%3D%22pt_ptmodule-introductionautogenerated.vtt%22",
            "source": "auto",
            "_class": "caption",
            "id": 4901374
          },
          {
            "status": 1,
            "created": "2018-03-19T17:51:46Z",
            "locale": {
              "locale": "en_US",
              "_class": "locale"
            },
            "file_name": "2018-03-19_17-51-46-2ede7b32ab5eb172189bada263dcb994.vtt",
            "title": "module-introduction.autogenerated.vtt",
            "video_label": "English [Auto]",
            "url": "https://udemy-captions.s3.amazonaws.com:443/11425492/en_US/2018-03-19_17-51-46-2ede7b32ab5eb172189bada263dcb994.vtt?Signature=l7TT4a2WrzgI2t2dj8JMyC7Y9H0%3D&Expires=1549318188&AWSAccessKeyId=AKIAJA6MCXJVONCBES7A&response-content-disposition=attachment%3B%20filename%3D%22module-introductionautogenerated.vtt%22",
            "source": "auto",
            "_class": "caption",
            "id": 4612548
          }
        ],
        "_class": "asset",
        "id": 11425492,
        "external_url": ""
      },
      "_class": "lecture",
      "id": 9429348,
      "supplementary_assets": []
    },
    {
      "title": "Understanding the Course Project Setup",
      "object_index": 12,
      "asset": {
        "body": "",
        "filename": "basics-02-understanding-the-project-setup.mp4",
        "stream_urls": {
          "Video": [
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand.udemy.com/2018-02-22_11-30-41-308c8a4db9ec2ce4fa284d91d19239ba/WebHD_720p.mp4?nva=20190204223948&token=08138a94768a1536980d8",
              "label": "720"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand2.udemy.com/2018-02-22_11-30-41-308c8a4db9ec2ce4fa284d91d19239ba/WebHD_480.mp4?nva=20190204223948&token=02cdfb4fc680a054ad53f",
              "label": "480"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand2.udemy.com/2018-02-22_11-30-41-308c8a4db9ec2ce4fa284d91d19239ba/WebHD.mp4?nva=20190204223948&token=0e539ccf88c112dbf6d4d",
              "label": "360"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand.udemy.com/2018-02-22_11-30-41-308c8a4db9ec2ce4fa284d91d19239ba/Web_144.mp4?nva=20190204223948&token=0a78d1790d0b1f4769255",
              "label": "144"
            },
            {
              "type": "application/x-mpegURL",
              "file": "HTTPS://adaptive-streaming.udemy.com/1561458/11425508/2018-02-22_11-30-41-308c8a4db9ec2ce4fa284d91d19239ba/aa0045dcc2e73ec079f8d357660fac5ad2db.m3u8?nva=1549319988&ttl=16200&ip=None&token=0979767d9dfcb0233a814",
              "label": "Auto"
            }
          ]
        },
        "asset_type": "Video",
        "time_estimation": 164,
        "slide_urls": [],
        "download_urls": {
          "Video": [
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand.udemy.com/2018-02-22_11-30-41-308c8a4db9ec2ce4fa284d91d19239ba/WebHD_720p.mp4?nva=20190204223948&filename=basics-02-understanding-the-project-setup.mp4&download=True&token=0d3c6a65e1463570a32d8",
              "label": "720"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand2.udemy.com/2018-02-22_11-30-41-308c8a4db9ec2ce4fa284d91d19239ba/WebHD_480.mp4?nva=20190204223948&filename=basics-02-understanding-the-project-setup.mp4&download=True&token=02b8423e09f33a8763b0b",
              "label": "480"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand2.udemy.com/2018-02-22_11-30-41-308c8a4db9ec2ce4fa284d91d19239ba/WebHD.mp4?nva=20190204223948&filename=basics-02-understanding-the-project-setup.mp4&download=True&token=0fa56d670f5f1fc01816c",
              "label": "360"
            },
            {
              "type": "video/mp4",
              "file": "https://udemy-assets-on-demand.udemy.com/2018-02-22_11-30-41-308c8a4db9ec2ce4fa284d91d19239ba/Web_144.mp4?nva=20190204223948&filename=basics-02-understanding-the-project-setup.mp4&download=True&token=029305555963496e71861",
              "label": "144"
            }
          ]
        },
        "captions": [
          {
            "status": 1,
            "created": "2018-04-24T20:03:28Z",
            "locale": {
              "locale": "es_ES",
              "_class": "locale"
            },
            "file_name": "2018-04-24_20-03-28-c4c41cd6250f6ab1f9ee832d12c43833.vtt",
            "title": "es_ES.understanding-the-course-project-setup.autogenerated.vtt",
            "video_label": "Spanish [Auto]",
            "url": "https://udemy-captions.s3.amazonaws.com:443/11425508/es_ES/2018-04-24_20-03-28-c4c41cd6250f6ab1f9ee832d12c43833.vtt?Signature=5kXlkG3BK9xrpRQnwrgZQciCqxU%3D&Expires=1549318188&AWSAccessKeyId=AKIAJA6MCXJVONCBES7A&response-content-disposition=attachment%3B%20filename%3D%22es_esunderstanding-the-course-project-setupautogenerated.vtt%22",
            "source": "auto",
            "_class": "caption",
            "id": 4918406
          },
          {
            "status": 1,
            "created": "2018-04-24T11:06:25Z",
            "locale": {
              "locale": "pt_BR",
              "_class": "locale"
            },
            "file_name": "2018-04-24_11-06-25-cc1c353aa3b0163334afe13a967ea98a.vtt",
            "title": "pt_PT.understanding-the-course-project-setup.autogenerated.vtt",
            "video_label": "Portuguese [Auto]",
            "url": "https://udemy-captions.s3.amazonaws.com:443/11425508/pt_BR/2018-04-24_11-06-25-cc1c353aa3b0163334afe13a967ea98a.vtt?Signature=3BN77kF1Th72gC%2FVrxNz%2FAOpfho%3D&Expires=1549318188&AWSAccessKeyId=AKIAJA6MCXJVONCBES7A&response-content-disposition=attachment%3B%20filename%3D%22pt_ptunderstanding-the-course-project-setupautogenerated.vtt%22",
            "source": "auto",
            "_class": "caption",
            "id": 4901376
          },
          {
            "status": 1,
            "created": "2018-03-19T17:51:46Z",
            "locale": {
              "locale": "en_US",
              "_class": "locale"
            },
            "file_name": "2018-03-19_17-51-46-4bfe0262a05dc82f0ffc7cb56a8c93ac.vtt",
            "title": "understanding-the-course-project-setup.autogenerated.vtt",
            "video_label": "English [Auto]",
            "url": "https://udemy-captions.s3.amazonaws.com:443/11425508/en_US/2018-03-19_17-51-46-4bfe0262a05dc82f0ffc7cb56a8c93ac.vtt?Signature=CN0hRJN%2FsKaJDD%2B0OOB8%2FAZvgR4%3D&Expires=1549318188&AWSAccessKeyId=AKIAJA6MCXJVONCBES7A&response-content-disposition=attachment%3B%20filename%3D%22understanding-the-course-project-setupautogenerated.vtt%22",
            "source": "auto",
            "_class": "caption",
            "id": 4612550
          }
        ],
        "_class": "asset",
        "id": 11425508,
        "external_url": ""
      },
      "_class": "lecture",
      "id": 9429350,
      "supplementary_assets": [
        {
          "body": "",
          "stream_urls": "",
          "filename": "basics-01-understanding-the-course-project-starting-code.zip",
          "time_estimation": 0,
          "slide_urls": [],
          "download_urls": {
            "File": [
              {
                "file": "https://udemy-assets-on-demand.udemy.com/2018-03-15_08-02-24-464560f0c980a592c17f7ee04e2f4977/original.zip?nva=20190204223948&filename=basics-01-understanding-the-course-project-starting-code.zip&download=True&token=058fe89ee528415c49250",
                "label": "download"
              }
            ]
          },
          "asset_type": "File",
          "_class": "asset",
          "id": 11695944,
          "external_url": ""
        }
      ]
    }
  ],
  "next": ""
}
"#;
