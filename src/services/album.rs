use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel,
    IntoActiveValue, QueryFilter, Set,
};

use crate::{
    database::entity::{album_uploads, albums, sea_orm_active_enums::Role, uploads, users},
    internal::{lateinit::LateInit, validate_length},
};

use std::sync::Arc;

use super::{prelude::*, upload::UploadService};

#[derive(Debug)]
pub struct AlbumService {
    database: Arc<DatabaseConnection>,
    file_service: Arc<LateInit<UploadService>>,
}

data_service_owned!(AlbumService, albums);

impl AlbumService {
    pub fn new(
        database: Arc<DatabaseConnection>,
        file_service: Arc<LateInit<UploadService>>,
    ) -> Self {
        Self {
            database,
            file_service,
        }
    }

    /// This should be used instead of [`DataService`]'s `get_page` for most cases.
    /// This will only return public albums unless the `accessing_user` is an admin or owns the albums.
    ///
    /// # Arguments
    ///
    /// * `page` - Page number
    /// * `page_size` - Size of each page
    /// * `user_id` - User who should own these albums
    /// * `accessing_user` - User accessing the albums
    pub async fn get_album_page(
        &self,
        page: u64,
        page_size: u64,
        user_id: &str,
        accessing_user: Option<&users::Model>,
    ) -> ServiceResult<ServicePage<albums::Model>> {
        let user_id = if let (Some(accessing_user), "@me") = (accessing_user, user_id.as_ref()) {
            accessing_user.id.to_owned()
        } else {
            user_id.to_owned()
        };

        // Don't show private albums if the user is unauthorized.
        let mut conditions = Condition::all().add(albums::Column::UserId.eq(user_id.to_owned()));
        if let Some(accessing_user) = accessing_user {
            if user_id != accessing_user.id && accessing_user.role != Role::Admin {
                conditions = conditions.add(albums::Column::Public.eq(true))
            }
        } else {
            conditions = conditions.add(albums::Column::Public.eq(true))
        }

        Ok(self.get_page(page, page_size, Some(conditions)).await?)
    }

    /// Get an album. If you don't need access validation use `by_id`
    /// This exists because the normal authorized methods aren't aware of the publicity flag.
    ///
    /// # Arguments
    ///
    /// * `id` - Album ID.
    /// * `accessing_user` - User who is accessing this album.
    pub async fn get_album(
        &self,
        id: &str,
        accessing_user: Option<&users::Model>,
    ) -> ServiceResult<albums::Model> {
        let album = self.by_id(id.into()).await?;

        // Validate access if private.
        if !album.public {
            let _ = self.validate_access(&album, accessing_user, true).await?;
        }

        Ok(album)
    }

    /// Delete an album.
    ///
    /// # Arguments
    ///
    /// * `id` - Album ID.
    /// * `delete_files` - Should all files in this album be deleted?
    /// * `accessing_user` - User who is accessing this album.
    ///
    /// # Returns
    /// Old album record before deletion
    pub async fn delete(
        &self,
        id: &str,
        delete_files: bool,
        accessing_user: Option<&users::Model>,
    ) -> ServiceResult<albums::Model> {
        let album = self
            .by_id_authorized(id.into(), accessing_user, true)
            .await?;

        if delete_files {
            // Get all upload IDs linked to this album
            let upload_ids: Vec<String> = album_uploads::Entity::find()
                .filter(album_uploads::Column::AlbumId.eq(&album.id))
                .all(self.database.as_ref())
                .await
                .map_err(|e| ServiceError::DbErr(e))?
                .into_iter()
                .map(|au| au.upload_id)
                .collect();

            if !upload_ids.is_empty() {
                self.file_service.delete_batch(&upload_ids, None).await?;
            }
        }

        album
            .clone()
            .into_active_model()
            .delete(self.database.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?;

        Ok(album)
    }

    /// Add uploads to an album by creating links in the junction table.
    ///
    /// # Arguments
    ///
    /// * `album_id` - Album ID to add uploads to
    /// * `upload_ids` - List of upload IDs to add
    /// * `accessing_user` - User performing the operation
    ///
    /// # Returns
    /// Number of uploads successfully added
    pub async fn add_to_album(
        &self,
        album_id: &str,
        upload_ids: &[String],
        accessing_user: Option<&users::Model>,
    ) -> ServiceResult<usize> {
        let album = self
            .by_id_authorized(album_id.into(), accessing_user, true)
            .await?;

        if upload_ids.is_empty() {
            return Ok(0);
        }

        // Get all uploads that exist
        let uploads = uploads::Entity::find()
            .filter(uploads::Column::Id.is_in(upload_ids.iter().cloned()))
            .all(self.database.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?;

        // Filter uploads based on access rules
        let accessible_uploads: Vec<&uploads::Model> = uploads
            .iter()
            .filter(|upload| {
                // Public uploads can always be added
                if upload.public {
                    return true;
                }

                // Private uploads can only be added if owned by the accessing user
                if let Some(accessing_user) = accessing_user {
                    upload.uploader == accessing_user.id
                } else {
                    false
                }
            })
            .collect();

        if accessible_uploads.is_empty() {
            return Ok(0);
        }

        // Get existing links to avoid duplicates
        let existing_links: std::collections::HashSet<String> = album_uploads::Entity::find()
            .filter(album_uploads::Column::AlbumId.eq(&album.id))
            .filter(
                album_uploads::Column::UploadId
                    .is_in(accessible_uploads.iter().map(|u| u.id.clone())),
            )
            .all(self.database.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?
            .into_iter()
            .map(|link| link.upload_id)
            .collect();

        // Create new links for uploads not already in the album
        let new_links: Vec<album_uploads::ActiveModel> = accessible_uploads
            .into_iter()
            .filter(|upload| !existing_links.contains(&upload.id))
            .map(|upload| album_uploads::ActiveModel {
                album_id: Set(album.id.clone()),
                upload_id: Set(upload.id.clone()),
                ..Default::default()
            })
            .collect();

        if new_links.is_empty() {
            return Ok(0);
        }

        // Insert all new links
        album_uploads::Entity::insert_many(new_links.clone())
            .exec(self.database.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?;

        Ok(new_links.len())
    }

    pub async fn update(
        &self,
        id: &str,
        name: Option<String>,
        description: Option<Option<String>>,
        public: Option<bool>,
        accessing_user: Option<&users::Model>,
    ) -> ServiceResult<albums::Model> {
        let mut album = self
            .by_id_authorized(id.to_owned(), accessing_user, true)
            .await?
            .into_active_model();

        if let Some(name) = name {
            album.name = Set(name);
        }

        if let Some(description) = description {
            album.description = Set(description);
        }

        if let Some(public) = public {
            album.public = Set(public);
        }

        self.validate(&album).await?;

        album
            .update(self.database.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))
    }

    async fn validate(&self, model: &albums::ActiveModel) -> ServiceResult<()> {
        validate_length("Album name", 4, 16, model.name.as_ref())?;

        if let Some(description) = model.description.as_ref() {
            validate_length("Album description", 0, 512, &description)?;
        }

        // Album with the same name owned by the same user already exists.
        if let Some(_) = albums::Entity::find()
            .filter(albums::Column::Name.eq(model.name.as_ref().to_string()))
            .filter(albums::Column::UserId.eq(model.user_id.as_ref().to_string()))
            .filter(albums::Column::Id.ne(model.id.as_ref().to_string()))
            .one(self.database.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?
        {
            return Err(ServiceError::InvalidData(
                "An album with that name already exists".into(),
            ));
        }

        Ok(())
    }

    /// Create an album.
    pub async fn create_album(
        &self,
        user_id: &str,
        name: &str,
        description: Option<String>,
        public: bool,
    ) -> ServiceResult<albums::Model> {
        let album_data = albums::ActiveModel {
            user_id: Set(user_id.to_owned()),
            name: Set(name.to_owned()),
            description: description.into_active_value(),
            public: Set(public),
            ..Default::default()
        };

        self.validate(&album_data).await?;

        Ok(album_data
            .insert(self.database.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?)
    }
}
