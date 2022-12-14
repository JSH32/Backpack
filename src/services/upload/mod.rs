mod providers;

use image::{io::Reader, ImageError};
use migration::Alias;
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

use super::{album::AlbumService, prelude::*};
use crate::{
    config::StorageConfig,
    database::entity::{sea_orm_active_enums::Role, uploads, users},
    models::{BatchDeleteResponse, BatchFileError, UploadData, UploadStats},
};

/// Service for managing files.
///
/// Files contain extra fields outside of the model which are located in the [`UploadData`] model.
/// Most operations in [`UploadService`] return [`UploadData`] instead of [`uploads::Model`].
#[derive(Debug)]
pub struct UploadService {
    /// Public storage handle.
    /// Use at your own risk.
    pub storage: Box<dyn StorageProvider>,
    database: Arc<DatabaseConnection>,
    album_service: Arc<AlbumService>,
    storage_url: String,
    file_size_limit: usize,
}

data_service_owned!(UploadService, uploads);

/// Result from uploading a file.
pub enum UploadResult {
    /// Upload success.
    Success(UploadData),
    /// File was already uploaded.
    Conflict(UploadData),
}

impl UploadService {
    pub async fn new(
        database: Arc<DatabaseConnection>,
        album_service: Arc<AlbumService>,
        config: StorageConfig,
        storage_url: &str,
        file_size_limit: usize,
    ) -> Self {
        Self {
            database,
            album_service,
            storage: providers::new_storage(config).await,
            storage_url: storage_url.into(),
            file_size_limit: file_size_limit * 1000 * 1000,
        }
    }

    /// Get a file. If you don't need access validation use `by_id`
    ///
    /// # Arguments
    ///
    /// * `id` - File ID.
    /// * `accessing_user` - User who is accessing this file.
    pub async fn get_file(
        &self,
        id: &str,
        accessing_user: Option<&users::Model>,
    ) -> ServiceResult<UploadData> {
        let file = self.by_id(id.into()).await?;

        // Validate access if private.
        if !file.public {
            let _ = self.validate_access(&file, accessing_user, true).await?;
        }

        Ok(self.to_upload_data(file))
    }

    /// Delete a file.
    ///
    /// # Arguments
    ///
    /// * `id` - File ID.
    /// * `accessing_user` - User accessing this file for deletion.
    pub async fn delete_file(
        &self,
        id: &str,
        accessing_user: Option<&users::Model>,
    ) -> ServiceResult<String> {
        let file = self
            .by_id_authorized(id.into(), accessing_user, false)
            .await?;

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
        accessing_user: Option<&users::Model>,
    ) -> ServiceResult<BatchDeleteResponse> {
        let mut response = BatchDeleteResponse::default();

        let files = uploads::Entity::find()
            .filter(uploads::Column::Id.is_in(ids.clone()))
            .all(self.database.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?;

        // All IDs found in the query.
        let found_ids: HashSet<String> = files.iter().map(|f| f.id.to_owned()).collect();

        // Add errors for every ID in the request that was not found in the query.
        for id in ids {
            if !found_ids.contains(id) {
                response.errors.push(BatchFileError {
                    id: id.to_string(),
                    error: "That file does not exist.".to_string(),
                })
            }
        }

        for file in files {
            if let Some(accessing_user) = accessing_user {
                // Don't use `validate_access` because of extra DB calls.
                if file.uploader != accessing_user.id && accessing_user.role != Role::Admin {
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
    /// You cannot upload files for another user.
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

        let file_exists = uploads::Entity::find()
            .filter(uploads::Column::Hash.eq(hash.to_owned()))
            .one(self.database.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?;

        if let Some(file) = file_exists {
            return Ok(UploadResult::Conflict(self.to_upload_data(file)));
        }

        let mut file = uploads::ActiveModel {
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

        Ok(UploadResult::Success(self.to_upload_data(file)))
    }

    pub async fn user_stats(
        &self,
        user_id: &str,
        accessing_user: Option<&users::Model>,
    ) -> ServiceResult<UploadStats> {
        if let Some(accessing_user) = accessing_user {
            if user_id != "@me"
                && accessing_user.id != user_id
                && accessing_user.role != Role::Admin
            {
                return Err(ServiceError::Forbidden {
                    id: Some(user_id.into()),
                    resource: "user's stats".into(),
                });
            }
        }

        let expr = uploads::Entity::find()
            .select_only()
            .filter(uploads::Column::Uploader.eq(user_id.clone()))
            .column_as(
                uploads::Column::Size.sum().cast_as(Alias::new("BIGINT")),
                "sum",
            )
            .build(self.database.get_database_backend())
            .to_owned();

        let usage = self
            .database
            .query_one(expr)
            .await
            .map_err(|e| ServiceError::DbErr(e))?;

        Ok(UploadStats {
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
    ///
    /// # Arguments
    ///
    /// * `page` - Page number
    /// * `page_size` - Size of each page
    /// * `user_id` - User who should own these uploads
    /// * `query` - Name of the upload (search)
    /// * `album_id` - Optional album
    /// * `public` - Public upload (query)
    /// * `accessing_user` - User accessing the uploads
    pub async fn get_upload_page(
        &self,
        page: usize,
        page_size: usize,
        user_id: Option<String>,
        query: Option<String>,
        album_id: Option<String>,
        public: Option<bool>,
        accessing_user: Option<&users::Model>,
    ) -> ServiceResult<ServicePage<UploadData>> {
        let mut conditions = Condition::all();

        if let Some(user_id) = user_id.to_owned() {
            conditions = conditions.add(uploads::Column::Uploader.eq(
                if let (Some(accessing_user), "@me") = (accessing_user, user_id.as_ref()) {
                    accessing_user.id.to_owned()
                } else {
                    user_id
                },
            ));
        }

        if let Some(query) = query {
            conditions = conditions.add(uploads::Column::Name.like(&format!("%{}%", query)));
        }

        if let Some(album_id) = album_id {
            let album = self.album_service.by_id(album_id.to_owned()).await?;

            // Non admin can't access a private album owned by another user.
            if !album.public {
                if let Some(accessing_user) = accessing_user {
                    if accessing_user.role != Role::Admin && album.user_id != accessing_user.id {
                        return Err(ServiceError::InvalidData(
                            "Cannot access private album owned by another user.".into(),
                        ));
                    }
                } else {
                    return Err(ServiceError::InvalidData(
                        "Cannot access private album owned by another user.".into(),
                    ));
                }
            }

            conditions = conditions.add(uploads::Column::AlbumId.eq(album_id));
        }

        if let Some(public) = public {
            // Non admin can't access someone elses private files.
            if let (Some(user_id), false) = (user_id, public) {
                if let Some(accessing_user) = accessing_user {
                    // Not owned by the proper user or admin.
                    if accessing_user.role != Role::Admin && user_id != accessing_user.id {
                        return Err(ServiceError::InvalidData(
                            "Cannot access user's private files".into(),
                        ));
                    }
                } else {
                    // No user provided.
                    return Err(ServiceError::InvalidData(
                        "Cannot access user's private files".into(),
                    ));
                }
            }

            conditions = conditions.add(uploads::Column::Public.eq(public));
        }

        let page = self.get_page(page, page_size, Some(conditions)).await?;

        Ok(ServicePage {
            page: page.page,
            pages: page.pages,
            items: page
                .items
                .into_iter()
                .map(|f| self.to_upload_data(f))
                .collect(),
        })
    }

    /// Convert a model to [`UploadData`].
    fn to_upload_data(&self, model: uploads::Model) -> UploadData {
        let mut upload_data = UploadData::from(model.clone());
        let root_path = PathBuf::from(&self.storage_url);

        upload_data.set_url(root_path.clone());

        if model.has_thumbnail {
            upload_data.set_thumbnail_url(root_path.clone());
        }

        upload_data
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
