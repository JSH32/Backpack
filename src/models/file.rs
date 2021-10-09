use serde::Serialize;

use chrono::{DateTime, Utc};

#[derive(Serialize)]
pub struct FileData {
    pub id: String,
    pub uploader: String,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    pub hash: String,
    pub uploaded: DateTime<Utc>,
    pub size: i64,
}

#[derive(Serialize)]
pub struct FilePage {
    pub page: u32,
    pub pages: u32,
    pub files: Vec<FileData>,
}
