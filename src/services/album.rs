use migration::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveValue, ModelTrait,
    QueryFilter, Set,
};

use crate::{
    database::entity::{albums, files, users},
    internal::{lateinit::LateInit, validate_length},
};
use std::sync::Arc;

use super::{file::FileService, prelude::*};

#[derive(Debug)]
pub struct AlbumService {
    database: Arc<DatabaseConnection>,
    file_service: Arc<LateInit<FileService>>,
}

data_service_owned!(AlbumService, albums);

impl AlbumService {
    pub fn new(
        database: Arc<DatabaseConnection>,
        file_service: Arc<LateInit<FileService>>,
    ) -> Self {
        Self {
            database,
            file_service,
        }
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
    pub async fn delete(
        &self,
        id: &str,
        delete_files: bool,
        accessing_user: Option<&users::Model>,
    ) -> ServiceResult<()> {
        let album = self
            .by_id_authorized(id.into(), accessing_user, true)
            .await?;

        if delete_files {
            self.file_service
                .delete_batch(
                    &album
                        .find_related(files::Entity)
                        .all(self.database.as_ref())
                        .await
                        .map_err(|e| ServiceError::DbErr(e))?
                        .iter()
                        .map(|f| f.id.clone())
                        .collect(),
                    None,
                )
                .await?;
        } else {
            files::Entity::update_many()
                .col_expr(files::Column::AlbumId, Expr::value::<Option<String>>(None))
                .filter(files::Column::AlbumId.eq(album.id))
                .exec(self.database.as_ref())
                .await
                .map_err(|e| ServiceError::DbErr(e))?;
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
        validate_length("Album name", 4, 16, name)?;

        if let Some(description) = &description {
            validate_length("Album description", 1, 512, description)?;
        }

        // Application with the same name owned by the same user already exists.
        if let Some(_) = albums::Entity::find()
            .filter(albums::Column::Name.eq(name.to_owned()))
            .filter(albums::Column::UserId.eq(user_id.to_owned()))
            .one(self.database.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?
        {
            return Err(ServiceError::InvalidData(
                "An album with that name already exists".into(),
            ));
        }

        let album_data = albums::ActiveModel {
            user_id: Set(user_id.to_owned()),
            name: Set(name.to_owned()),
            description: description.into_active_value(),
            public: Set(public),
            ..Default::default()
        }
        .insert(self.database.as_ref())
        .await
        .map_err(|e| ServiceError::DbErr(e))?;

        Ok(album_data)
    }
}
