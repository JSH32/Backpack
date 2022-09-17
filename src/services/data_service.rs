//! Data service is a service with an associated table and attached operations.

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

pub(crate) use data_service;

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
        AsTitleCase(entity.table_name().trim_end_matches("s")).to_string()
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
    /// * `response_has_id` - Should the returned [`ServiceResult`] string contain the Id of the deleted record.
    /// * `condition` - Additional conditions for finding record to delete.
    async fn delete(
        &self,
        id: <E::PrimaryKey as PrimaryKeyTrait>::ValueType,
        response_has_id: bool,
        condition: Option<Condition>,
    ) -> ServiceResult<String> {
        let id_str = remove_first_and_last(format!("{:?}", id));
        let (db, _) = self.get_data_source();

        // We need to do this workaround to use both IDs and a condition in one.
        let mut select = E::find_by_id(id);

        if let Some(condition) = condition {
            select = select.filter(condition);
        }

        let result = match select
            .one(db.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?
        {
            Some(v) => v,
            None => return Err(ServiceError::NotFound(self.resource_name())),
        };

        result
            .into_active_model()
            .delete(db.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?;

        Ok(format!(
            "{}{} was deleted",
            self.resource_name(),
            if response_has_id {
                format!(" ({})", id_str)
            } else {
                "".into()
            }
        ))
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

/// Remove first and last character of a string.
fn remove_first_and_last(value: String) -> String {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.collect()
}
