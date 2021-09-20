use chrono::{DateTime, Utc};
use serde::{Serialize};

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