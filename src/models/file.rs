use chrono::{DateTime, Utc};
use serde::{Serialize};

#[derive(Serialize)]
pub struct FileData {
    pub file: String,
    pub hash: String,
    pub uploaded: DateTime<Utc>,

    #[serde(skip_serializing)]
    pub owner_id: i32,
}