mod providers;

use image::{io::Reader, ImageError};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, ModelTrait, QueryFilter, QuerySelect, QueryTrait, Set,
};
use sha2::{Digest, Sha256};
use std::{
    collections::HashSet,
    ffi::OsStr,
    io::Cursor,
    path::{Path, PathBuf},
    sync::Arc,
};

use self::providers::StorageProvider;

use super::prelude::*;
use crate::{
    config::StorageConfig,
    database::entity::files,
    models::{BatchDeleteResponse, BatchFileError, FileData, FileStats},
};

/// Service for managing files.
///
/// Files contain extra fields outside of the model which are located in the [`FileData`] model.
/// Most operations in [`FileService`] return [`FileData`] instead of [`files::Model`].
pub struct FileService {
    /// Public storage handle.
    /// Use at your own risk.
    pub storage: Box<dyn StorageProvider>,
    database: Arc<DatabaseConnection>,
    storage_url: String,
    file_size_limit: usize,
}

data_service!(FileService, files);

/// Result from uploading a file.
pub enum UploadResult {
    /// Upload success.
    Success(FileData),
    /// File was already uploaded.
    Conflict(FileData),
}

impl FileService {
    pub async fn new(
        database: Arc<DatabaseConnection>,
        config: StorageConfig,
        storage_url: &str,
        file_size_limit: usize,
    ) -> Self {
        Self {
            database,
            storage: providers::new_storage(config).await,
            storage_url: storage_url.into(),
            file_size_limit: file_size_limit * 1000 * 1000,
        }
    }

    /// Get a file. If you don't need ownership validation use `by_id`
    ///
    /// # Arguments
    ///
    /// * `id` - File ID.
    /// * `user_id` - User who owns this file. This will validate ownership.
    pub async fn get_file(&self, id: &str, user_id: Option<&str>) -> ServiceResult<FileData> {
        let file = self.by_id(id.into()).await?;

        if let Some(user_id) = user_id {
            if file.uploader != user_id {
                return Err(ServiceError::Forbidden {
                    id: id.into(),
                    resource: self.resource_name(),
                });
            }
        }

        Ok(self.to_file_data(file))
    }

    /// Delete a file.
    ///
    /// # Arguments
    ///
    /// * `id` - File ID.
    /// * `user_id` - User who owns this file. If provided this will validate ownership.
    pub async fn delete_file(&self, id: &str, user_id: Option<&str>) -> ServiceResult<String> {
        let file = self.by_id(id.into()).await?;

        if let Some(user_id) = user_id {
            if file.uploader != user_id {
                return Err(ServiceError::Forbidden {
                    id: id.into(),
                    resource: self.resource_name(),
                });
            }
        }

        file.clone()
            .delete(self.database.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?;

        // We dont care about the result of this because of discrepancies
        let _ = self
            .storage
            .delete_objects(vec![file.name.clone(), format!("thumb/{}", &file.name)])
            .await;

        Ok(format!("File {} was deleted", file.name))
    }

    /// Delete multiple files.
    ///
    /// # Arguments
    ///
    /// * `ids` - List of file IDs.
    /// * `user_id` - User who owns this file. If provided this will validate ownership.
    pub async fn delete_batch(
        &self,
        ids: &Vec<String>,
        user_id: Option<&str>,
    ) -> ServiceResult<BatchDeleteResponse> {
        let mut response = BatchDeleteResponse::default();

        let files = files::Entity::find()
            .filter(files::Column::Id.is_in(ids.clone()))
            .all(self.database.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?;

        // All IDs found in the query.
        let ids: HashSet<String> = files.iter().map(|f| f.id.to_owned()).collect();

        // Add errors for every ID in the request that was not found in the query.
        for id in &ids {
            if !ids.contains(id) {
                response.errors.push(BatchFileError {
                    id: id.to_string(),
                    error: "That file does not exist.".to_string(),
                })
            }
        }

        for file in files {
            if let Some(user_id) = user_id {
                if file.uploader != user_id {
                    response.errors.push(BatchFileError {
                        id: file.id,
                        error: "You are not allowed to access this file.".to_string(),
                    });

                    continue;
                }
            }

            file.clone()
                .delete(self.database.as_ref())
                .await
                .map_err(|e| ServiceError::DbErr(e))?;

            // We dont care about the result of this because of possible discrepancies, just pretend the file is deleted.
            let _ = self
                .storage
                .delete_objects(vec![file.name.clone(), format!("thumb/{}", &file.name)])
                .await;

            response.deleted.push(file.id);
        }

        Ok(response)
    }

    /// Upload a file to the storage provider.
    pub async fn upload_file(
        &self,
        user_id: &str,
        name: &str,
        buffer: &Vec<u8>,
    ) -> ServiceResult<UploadResult> {
        if buffer.len() > self.file_size_limit {
            return Err(ServiceError::TooLarge(format!(
                "File was larger than the size limit of {}mb",
                self.file_size_limit / 1000 / 1000
            )));
        }

        let extension = Path::new(&name)
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or("");

        // New filename, collision not likely with NanoID
        let filename = format!("{}.{}", nanoid::nanoid!(10), extension);

        let hash = &format!("{:x}", Sha256::digest(&buffer));

        let file_exists = files::Entity::find()
            .filter(files::Column::Hash.eq(hash.to_owned()))
            .one(self.database.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?;

        if let Some(file) = file_exists {
            return Ok(UploadResult::Conflict(self.to_file_data(file)));
        }

        let mut file = files::ActiveModel {
            uploader: Set(user_id.into()),
            name: Set(filename.to_owned()),
            original_name: Set(name.into()),
            hash: Set(hash.to_owned()),
            size: Set(buffer.len() as i64),
            ..Default::default()
        }
        .insert(self.database.as_ref())
        .await
        .map_err(|e| ServiceError::DbErr(e))?;

        // Upload file to storage provider
        // If this fails attempt to delete the file from database
        if let Err(err) = self.storage.put_object(&filename, &buffer).await {
            let _ = file.delete(self.database.as_ref()).await;
            return Err(ServiceError::ServerError(err));
        }

        // Create thumbnail.
        if IMAGE_EXTS
            .into_iter()
            .any(|ext| ext.eq(&extension.to_uppercase()))
        {
            if let Ok(image) = &create_thumbnail_image(&buffer) {
                if let Ok(_) = self
                    .storage
                    .put_object(&format!("thumb/{}", &filename), image)
                    .await
                {
                    let mut active_file = file.into_active_model();
                    active_file.has_thumbnail = Set(true);
                    file = active_file
                        .update(self.database.as_ref())
                        .await
                        .map_err(|e| ServiceError::DbErr(e))?;
                }
            }
        }

        Ok(UploadResult::Success(self.to_file_data(file)))
    }

    pub async fn user_stats(&self, user_id: &str) -> ServiceResult<FileStats> {
        let expr = files::Entity::find()
            .select_only()
            .filter(files::Column::Uploader.eq(user_id.clone()))
            .column_as(files::Column::Size.sum(), "sum")
            .build(self.database.get_database_backend())
            .to_owned();

        let usage = self
            .database
            .query_one(expr)
            .await
            .map_err(|e| ServiceError::DbErr(e))?;

        Ok(FileStats {
            usage: match usage {
                // The query can fail if no files are uploaded.
                Some(v) => match v.try_get("", "sum") {
                    Ok(v) => v,
                    Err(_) => 0,
                },
                None => 0,
            },
        })
    }

    /// This should be used instead of [`DataService`]'s `get_page` for most cases.
    pub async fn get_file_page(
        &self,
        page: usize,
        page_size: usize,
        user_id: Option<&str>,
        query: Option<&str>,
    ) -> ServiceResult<ServicePage<FileData>> {
        let mut conditions = Condition::all();

        if let Some(user_id) = user_id {
            conditions = conditions.add(files::Column::Uploader.eq(user_id));
        }

        if let Some(query) = query {
            conditions = conditions.add(files::Column::Name.like(&format!("%{}%", query)));
        }

        let page = self.get_page(page, page_size, Some(conditions)).await?;

        Ok(ServicePage {
            page: page.page,
            pages: page.pages,
            items: page
                .items
                .into_iter()
                .map(|f| self.to_file_data(f))
                .collect(),
        })
    }

    /// Convert a model to [`FileData`].
    fn to_file_data(&self, model: files::Model) -> FileData {
        let mut file_data = FileData::from(model.clone());
        let root_path = PathBuf::from(&self.storage_url);

        file_data.set_url(root_path.clone());

        if model.has_thumbnail {
            file_data.set_thumbnail_url(root_path.clone());
        }

        file_data
    }
}

const IMAGE_EXTS: &'static [&'static str] =
    &["PNG", "JPG", "JPEG", "GIF", "WEBP", "JFIF", "PJPEG", "PJP"];

fn create_thumbnail_image(bytes: &Vec<u8>) -> Result<Vec<u8>, ImageError> {
    let mut buf = Vec::new();

    Reader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?
        .thumbnail(500, 500)
        .write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png)?;

    Ok(buf)
}
