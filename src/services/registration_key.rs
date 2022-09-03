use chrono::{Duration, Utc};
use sea_orm::{prelude::*, Condition, Set};
use std::sync::Arc;
use uuid::Uuid;

use crate::database::entity::registration_keys;

use super::prelude::*;

pub struct RegistrationKeyService {
    database: Arc<DatabaseConnection>,
}

data_service!(RegistrationKeyService, registration_keys);

impl<'a> RegistrationKeyService {
    pub fn new(database: Arc<DatabaseConnection>) -> Self {
        Self { database }
    }

    /// Get a registration key by UUID.
    pub async fn get_by_code(&self, code: &str) -> ServiceResult<registration_keys::Model> {
        self.by_condition(
            Condition::all().add(registration_keys::Column::Code.eq(Self::to_uuid(code)?)),
        )
        .await
    }

    /// Create a new registration key.
    ///
    /// # Arguments
    ///
    /// * `issuer` - User who issued the registration key.
    /// * `uses_left` - Amount of times the key should be used.
    pub async fn create_registration_key(
        &self,
        issuer: &str,
        uses_left: Option<i32>,
        expiration: Option<i64>,
    ) -> ServiceResult<registration_keys::Model> {
        registration_keys::ActiveModel {
            issuer: Set(issuer.into()),
            uses_left: Set(uses_left),
            expiry_date: Set(match expiration {
                Some(ms) => Some(Utc::now() + Duration::milliseconds(ms)),
                None => None,
            }),
            ..Default::default()
        }
        .insert(self.database.as_ref())
        .await
        .map_err(|e| ServiceError::DbErr(e))
    }

    fn to_uuid(uuid_str: &str) -> ServiceResult<Uuid> {
        Uuid::parse_str(uuid_str)
            .map_err(|_| ServiceError::InvalidData("Invalid registration key".to_string()))
    }
}
