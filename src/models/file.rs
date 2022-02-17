use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use serde::Serialize;

use chrono::{DateTime, Utc};

use crate::util::file::IMAGE_EXTS;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileData {
    pub id: String,
    pub uploader: String,
    pub name: String,
    pub original_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,

    pub hash: String,
    pub uploaded: DateTime<Utc>,
    pub size: i64,
}

impl FileData {
    /// Computes and sets the URL based on a root storage path
    pub fn set_url(&mut self, mut root_path: PathBuf) {
        root_path.push(&self.name);
        self.url = Some(root_path.as_path().display().to_string().replace("\\", "/"))
    }

    /// Computes and sets the URL based on root storage path
    /// This will only set if a valid image or extension was sent
    pub fn set_thumbnail_url(&mut self, mut root_path: PathBuf) {
        let extension = Path::new(&self.name)
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or("");

        if IMAGE_EXTS
            .into_iter()
            .any(|ext| ext.eq(&extension.to_uppercase()))
        {
            root_path.push(format!("thumb/{}", &self.name));
            self.thumbnail_url = Some(root_path.as_path().display().to_string().replace("\\", "/"));
        }
    }
}

#[derive(Serialize)]
pub struct FilePage {
    pub page: u32,
    pub pages: u32,
    pub files: Vec<FileData>,
}

#[derive(Serialize)]
pub struct FileStats {
    pub usage: i64,
}
