use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use actix_extract_multipart::File;
use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};
use utoipa::Component;

use crate::util::file::IMAGE_EXTS;

use crate::database::entity::files;

#[derive(Serialize, Component)]
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

    #[component(value_type = f64)]
    pub uploaded: DateTime<Utc>,
    pub size: i64,
}

impl From<files::Model> for FileData {
    fn from(file: files::Model) -> Self {
        Self {
            id: file.id,
            uploader: file.uploader,
            name: file.name,
            original_name: file.original_name,
            hash: file.hash,
            uploaded: file.uploaded.into(),
            size: file.size,
            // These fields are not stored in database
            // They are filled in by the route returning it
            url: None,
            thumbnail_url: None,
        }
    }
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
pub struct FileStats {
    pub usage: i64,
}

/// Upload a file
#[derive(Deserialize, Component, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UploadFile {
    #[component(value_type = String, format = ComponentFormat::Binary)]
    pub upload_file: File,
}
