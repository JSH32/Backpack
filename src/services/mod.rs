use actix_web::dev::ServiceResponse;
use heck::AsTitleCase;
use migration::DbErr;
use sea_orm::{QueryFilter, EntityName};
use sea_orm::{ColumnTrait, DatabaseConnection, DeriveEntityModel, EntityTrait, PrimaryKeyTrait};
use thiserror::Error;

pub mod registration_key;
pub mod user;

pub type ServiceResult<T> = Result<T, ServiceError>;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    DbErr(sea_orm::DbErr),
    #[error("{0} was not found")]
    NotFound(String),
}

// trait Service {
//     fn get_record<T: sea_orm::EntityTrait>() -> ServiceResponse<T::Model> {
//         T::find_by_id("d")
//     }
// }

/// Utility function for getting any record as a [`ServiceResult`]
pub async fn get_record_by_id<T: sea_orm::EntityTrait>(
    database: &DatabaseConnection,
    entity: T,
    id: <T::PrimaryKey as PrimaryKeyTrait>::ValueType,
) -> ServiceResult<T::Model> {
    record_service_result(entity, T::find_by_id(id)
    .one(database)
    .await)
}

pub async fn get_record_by_condition<T: sea_orm::EntityTrait>(
    database: &DatabaseConnection,
    entity: T,
    condition: sea_orm::Condition,
) -> ServiceResult<T::Model> {
    record_service_result(entity, T::find()
    .filter(condition)
    .one(database)
    .await)
}

fn record_service_result<T: sea_orm::EntityTrait>(
    entity: T,
    result: Result<Option<<T as EntityTrait>::Model>, DbErr>,
) -> ServiceResult<T::Model> {
    match result.map_err(|e| ServiceError::DbErr(e))? {
        Some(v) => Ok(v),
        // This assumes that every single table name is plural and ends with "s"
        None => Err(ServiceError::NotFound(
            AsTitleCase(result.table_name().trim_end_matches("s")).to_string(),
        )),
    }
}
