use serde_derive::{Deserialize, Serialize};

/// Course information coming from subscribed courses.
#[derive(Serialize, Deserialize, Debug)]
pub struct Course {
    pub id: u64,
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
    pub filename: String,
    pub asset_type: String,
    pub time_estimation: u64,
    pub download_urls: Option<Vec<DownloadUrl>>,
}

/// Lecture information.
#[derive(Serialize, Deserialize, Debug)]
pub struct Lecture {
    pub object_index: u64,
    pub title: String,
    pub asset: Asset,
    pub supplementary_assets: Vec<Asset>,
}

/// Chapter information.
#[derive(Serialize, Deserialize, Debug)]
pub struct Chapter {
    pub object_index: u64,
    pub title: String,
    pub lectures: Vec<Lecture>,
}

// Full course information.
#[derive(Serialize, Deserialize, Debug)]
pub struct CourseContent {
    pub chapters: Vec<Chapter>,
}
