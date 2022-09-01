use sea_orm::{DatabaseConnection, EntityName, EntityTrait};
use thiserror::Error;
use uuid::Uuid;

use crate::database::entity::{registration_keys, users};

use super::{get_record, registration_key::RegistrationKeyService, ServiceError, ServiceResult};

pub struct UserService<'a> {
    database: &'a DatabaseConnection,
    registration_key_service: &'a RegistrationKeyService<'a>,
}

#[derive(Error, Debug)]
pub enum UserCreateError {
    #[error("Invalid registration key")]
    InvalidRegistrationKey,
}

impl<'a> UserService<'a> {
    pub fn new(
        database: &'a DatabaseConnection,
        registration_key_service: &'a RegistrationKeyService<'a>,
    ) -> Self {
        Self {
            database,
            registration_key_service,
        }
    }

    /// Get a user by ID.
    /// This returns either the user model or an error string if not found.
    pub async fn get_user(&self, id: &str) -> ServiceResult<users::Model> {
        get_record(self.database, users::Entity, id.to_string()).await
    }

    /// Create a user.
    ///
    /// # Arguments
    ///
    /// * `username` - Users username
    /// * `email` - User email
    /// * `password` - User password (this will be hashed)
    /// * `registration_key` - Should registration key be used and the provided registration key
    pub async fn create_user(
        &self,
        username: String,
        email: String,
        password: String,
        registration_key: Option<(bool, String)>,
    ) -> Result<users::Model, UserCreateError> {
        if let Some((true, key)) = registration_key {
            // self.registration_key_service.get_registration_key()
        }
    }

    pub async fn search_users(&self) -> ServiceResult<users::Model> {}
}
