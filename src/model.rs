use serde_derive::{Deserialize, Serialize};

pub type CourseId = u64;
pub type LectureId = u64;
pub type ObjectIndex = u64;
pub type VideoQuality = u64;

/// Course information coming from subscribed courses.
#[derive(Serialize, Deserialize, Debug)]
pub struct Course {
    pub id: CourseId,
    pub url: String,
    pub published_title: String,
}

/// Information on downloadable media.
#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadUrl {
    pub r#type: Option<String>,
    pub file: String,
    pub label: String,
}

/// Asset information, either for video or supplementary asset.
#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    pub title: String,
    pub asset_type: String,
    pub time_estimation: u64,
    pub download_urls: Option<Vec<DownloadUrl>>,
}

/// Lecture information. Coming from lecture detail.
#[derive(Serialize, Deserialize, Debug)]
pub struct LectureDetail {
    pub id: LectureId,
    pub title: String,
    pub asset: Asset,
}

/// Lecture information. Coming from genaral course information.
#[derive(Serialize, Deserialize, Debug)]
pub struct Lecture {
    pub id: LectureId,
    pub object_index: ObjectIndex,
    pub title: String,
    pub filename: String,
    pub has_video: bool,
}

/// Chapter information.
#[derive(Serialize, Deserialize, Debug)]
pub struct Chapter {
    pub object_index: ObjectIndex,
    pub title: String,
    pub lectures: Vec<Lecture>,
}

// Full course information.
#[derive(Serialize, Deserialize, Debug)]
pub struct CourseContent {
    pub chapters: Vec<Chapter>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthResponse {
    pub _class: String,
    pub id: u32,
    pub access_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompleteRequest {
    pub lecture_id: LectureId,
    pub downloaded: bool,
}

#[derive(Debug)]
pub struct Auth {
    pub access_token: Option<String>,
}

impl Auth {
    pub fn with_token(access_token: &str) -> Auth {
        Auth {
            access_token: Some(access_token.into()),
        }
    }
}
