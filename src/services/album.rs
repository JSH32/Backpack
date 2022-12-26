use migration::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel,
    IntoActiveValue, ModelTrait, QueryFilter, Set,
};

use crate::{
    database::entity::{albums, sea_orm_active_enums::Role, uploads, users},
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
        page: usize,
        page_size: usize,
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
            self.file_service
                .delete_batch(
                    &album
                        .find_related(uploads::Entity)
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
            uploads::Entity::update_many()
                .col_expr(
                    uploads::Column::AlbumId,
                    Expr::value::<Option<String>>(None),
                )
                .filter(uploads::Column::AlbumId.eq(album.id.to_owned()))
                .exec(self.database.as_ref())
                .await
                .map_err(|e| ServiceError::DbErr(e))?;
        }

        Ok(album)
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
            validate_length("Album description", 1, 512, &description)?;
        }

        // Album with the same name owned by the same user already exists.
        if let Some(_) = albums::Entity::find()
            .filter(albums::Column::Name.eq(model.name.as_ref().to_string()))
            .filter(albums::Column::UserId.eq(model.user_id.as_ref().to_string()))
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
