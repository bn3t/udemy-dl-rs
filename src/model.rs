use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Course {
    pub id: u64,
    pub url: String,
    pub published_title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadUrl {
    pub r#type: Option<String>,
    pub file: String,
    pub label: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    pub filename: String,
    pub asset_type: String,
    pub time_estimation: u64,
    pub download_urls: Option<Vec<DownloadUrl>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Lecture {
    pub object_index: u64,
    pub title: String,
    pub asset: Asset,
    pub supplementary_assets: Vec<Asset>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chapter {
    pub object_index: u64,
    pub title: String,
    pub lectures: Vec<Lecture>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CourseContent {
    pub chapters: Vec<Chapter>,
}
