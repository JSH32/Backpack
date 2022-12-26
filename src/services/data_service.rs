//! Data service is a service with an associated table and attached operations.

use crate::database::entity::{sea_orm_active_enums::Role, users};

use super::{ServiceError, ServicePage, ServiceResult};
use heck::AsTitleCase;
use sea_orm::{prelude::*, Condition, FromQueryResult, IntoActiveModel};
use std::sync::Arc;

/// Automatically implement a [`DataService`] with an associated entity.
/// The service must have a `database` member of type [`Arc<DatabaseConnection>`].
macro_rules! data_service {
    ($service:ty, $entity_module:ident) => {
        #[async_trait::async_trait]
        impl
            crate::services::data_service::DataService<
                $entity_module::Entity,
                $entity_module::Model,
                $entity_module::ActiveModel,
            > for $service
        {
            fn get_data_source(
                &self,
            ) -> (std::sync::Arc<DatabaseConnection>, $entity_module::Entity) {
                (self.database.clone(), $entity_module::Entity)
            }
        }
    };
}

/// Same as [`data_service`] but with functions dealing with resources that can be owned by [`users::Model`].
macro_rules! data_service_owned {
    ($service:ty, $entity_module:ident) => {
        crate::services::data_service::data_service!($service, $entity_module);
        impl
            crate::services::data_service::UserOwnedService<
                $entity_module::Entity,
                $entity_module::Model,
                $entity_module::ActiveModel,
            > for $service
        {
        }
    };
}

pub(crate) use data_service;
pub(crate) use data_service_owned;

#[async_trait::async_trait]
pub trait UserOwnedService<
    E: EntityTrait<Model = M>,
    M: ModelTrait + FromQueryResult + Sync + IntoActiveModel<AM>,
    AM: ActiveModelTrait + ActiveModelBehavior + Send,
>: DataService<E, M, AM> where
    M::Entity: EntityTrait<Model = M> + Related<users::Entity>,
{
    /// Return the record if access granted. Otherwise [`ServiceError`]
    async fn validate_access(
        &self,
        record: &M,
        user: Option<&users::Model>,
        require_user: bool,
    ) -> ServiceResult<M> {
        if let Some(user) = user {
            if user.role == Role::Admin {
                return Ok(record.clone());
            }

            let (db, _) = self.get_data_source();

            // Get the related user or resource owner
            match record
                .find_related(users::Entity)
                .one(db.as_ref())
                .await
                .map_err(|e| ServiceError::DbErr(e))?
            {
                // At this point we have verified that a user owns a resource and we can return it.
                Some(found_user) => {
                    // Check if user associated with resource is the same as the original user provided.
                    if user.id == found_user.id {
                        Ok(record.clone())
                    } else {
                        Err(ServiceError::Forbidden {
                            id: None,
                            resource: self.resource_name(),
                        })
                    }
                }
                None => Err(ServiceError::Forbidden {
                    id: None,
                    resource: self.resource_name(),
                }),
            }
        } else if require_user && user.is_none() {
            return Err(ServiceError::Forbidden {
                id: None,
                resource: self.resource_name(),
            });
        } else {
            Ok(record.clone())
        }
    }

    /// Just like [`delete`] but this will error if the user isn't allowed to **fully** access this resource.
    /// The check will be overridden if the provided user was an admin.
    /// If [`None`] is provided this will return the resource.
    async fn delete_authorized(
        &self,
        id: <E::PrimaryKey as PrimaryKeyTrait>::ValueType,
        condition: Option<Condition>,
        user: Option<&users::Model>,
    ) -> ServiceResult<String> {
        // We need to do this workaround to use both IDs and a condition in one.
        let mut select = E::find_by_id(id);

        if let Some(condition) = condition {
            select = select.filter(condition);
        }

        let (db, _) = self.get_data_source();

        let model = match select
            .one(db.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?
        {
            Some(v) => v,
            None => return Err(ServiceError::NotFound(self.resource_name())),
        };

        self.validate_access(&model, user, false)
            .await?
            .into_active_model()
            .delete(db.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?;

        Ok(format!("{} was deleted", self.resource_name()))
    }

    /// Just like [`by_id`] but this will error if the user isn't allowed to **fully** access this resource.
    /// The check will be overridden if the provided user was an admin.
    /// If [`None`] is provided this will return the resource.
    async fn by_id_authorized(
        &self,
        id: <E::PrimaryKey as PrimaryKeyTrait>::ValueType,
        user: Option<&users::Model>,
        require_user: bool,
    ) -> ServiceResult<M> {
        self.validate_access(&self.by_id(id).await?, user, require_user)
            .await
    }

    /// Just like [`by_condition`] but this will error if the user isn't allowed to **fully** access this resource.
    /// The check will be overridden if the provided user was an admin.
    /// If [`None`] is provided this will return the resource.
    async fn by_condition_authorized(
        &self,
        condition: Condition,
        user: Option<&users::Model>,
        require_user: bool,
    ) -> ServiceResult<M> {
        self.validate_access(&self.by_condition(condition).await?, user, require_user)
            .await
    }
}

/// Provides standard operations for services which are associated with a db table.
#[async_trait::async_trait]
pub trait DataService<
    E: EntityTrait<Model = M>,
    M: ModelTrait + FromQueryResult + Sync + IntoActiveModel<AM>,
    AM: ActiveModelTrait + ActiveModelBehavior + Send,
>
{
    /// Get the data source and entity.
    /// This is used for internal operations.
    fn get_data_source(&self) -> (Arc<DatabaseConnection>, E);

    /// Get name of resource as string.
    fn resource_name(&self) -> String {
        // Get a table name as a string representation.
        // This assumes that every single table name is plural and ends with "s"
        let (_, entity) = self.get_data_source();
        AsTitleCase(entity.table_name().trim_end_matches("s"))
            .to_string()
            .to_lowercase()
    }

    /// Get [`Self::M`] by ID.
    async fn by_id(&self, id: <E::PrimaryKey as PrimaryKeyTrait>::ValueType) -> ServiceResult<M> {
        let (db, _) = self.get_data_source();
        match E::find_by_id(id)
            .one(db.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?
        {
            Some(v) => Ok(v),
            None => Err(ServiceError::NotFound(self.resource_name())),
        }
    }

    /// Get [`T`] by a [`sea_orm::Condition`].
    async fn by_condition(&self, condition: sea_orm::Condition) -> ServiceResult<M> {
        let (db, _) = self.get_data_source();
        match E::find()
            .filter(condition)
            .one(db.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?
        {
            Some(v) => Ok(v),
            None => Err(ServiceError::NotFound(self.resource_name())),
        }
    }

    /// Delete a record by id.
    ///
    /// # Arguments
    ///
    /// * `id` - Id of the record
    /// * `condition` - Additional conditions for finding record to delete.    
    async fn delete(
        &self,
        id: <E::PrimaryKey as PrimaryKeyTrait>::ValueType,
        condition: Option<Condition>,
    ) -> ServiceResult<String> {
        // We need to do this workaround to use both IDs and a condition in one.
        let mut select = E::find_by_id(id);

        if let Some(condition) = condition {
            select = select.filter(condition);
        }

        let (db, _) = self.get_data_source();

        let model = match select
            .one(db.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?
        {
            Some(v) => v,
            None => return Err(ServiceError::NotFound(self.resource_name())),
        };

        model
            .into_active_model()
            .delete(db.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?;

        Ok(format!("{} was deleted", self.resource_name()))
    }

    /// This is just a simple wrapper around [`get_page`] that ensures `user_id` and `accessing_user`'s ID are equal (or admin).
    /// This doesn't have special relation logic like `by_id_authenticated`.
    /// This accepts `@me` for `user_id` which will resolve to `accessing_user`'s ID.
    async fn get_page_authorized(
        &self,
        page: usize,
        page_size: usize,
        condition: Option<Condition>,
        user_id: &str,
        accessing_user: &users::Model,
    ) -> ServiceResult<ServicePage<M>> {
        if accessing_user.id != user_id && accessing_user.role != Role::Admin && user_id != "@me" {
            Err(ServiceError::Forbidden {
                id: None,
                resource: format!("{} page", self.resource_name()),
            })
        } else {
            self.get_page(page, page_size, condition).await
        }
    }

    /// Get a [`ServicePage`] of [`M`].
    async fn get_page(
        &self,
        page: usize,
        page_size: usize,
        condition: Option<Condition>,
    ) -> ServiceResult<ServicePage<M>> {
        let (db, _) = self.get_data_source();

        let paginator = match condition {
            Some(condition) => E::find().filter(condition),
            None => E::find(),
        }
        .into_model::<M>()
        .paginate(db.as_ref(), page_size);

        let total_pages = paginator
            .num_pages()
            .await
            .map_err(|e| ServiceError::DbErr(e))?;

        // Pages start at 1
        if page < 1 {
            Err(ServiceError::InvalidData("Pages start at 1".into()))
        } else if total_pages < page {
            Err(ServiceError::InvalidData(format!(
                "There are only {} pages",
                total_pages
            )))
        } else {
            Ok(ServicePage {
                page: page,
                pages: total_pages,
                items: paginator
                    .fetch_page(page - 1)
                    .await
                    .map_err(|e| ServiceError::DbErr(e))?,
            })
        }
    }
}
