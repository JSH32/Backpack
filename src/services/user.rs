use sea_orm::DatabaseConnection;
use std::sync::Arc;
use thiserror::Error;

use super::{prelude::*, registration_key::RegistrationKeyService};
use crate::database::entity::users;

pub struct UserService {
    database: Arc<DatabaseConnection>,
    registration_key_service: Arc<RegistrationKeyService>,
}

data_service!(UserService, users);

#[derive(Error, Debug)]
pub enum UserCreateError {
    #[error("Invalid registration key")]
    InvalidRegistrationKey,
}

impl UserService {
    pub fn new(
        database: Arc<DatabaseConnection>,
        registration_key_service: Arc<RegistrationKeyService>,
    ) -> Self {
        Self {
            database,
            registration_key_service,
        }
    }

    // / Create a user.
    // /
    // / # Arguments
    // /
    // / * `username` - Users username
    // / * `email` - User email
    // / * `password` - User password (this will be hashed)
    // / * `registration_key` - Should registration key be used and the provided registration key
    pub async fn create_user(
        &self,
        username: String,
        email: String,
        password: String,
        registration_key: Option<(bool, String)>,
    ) {
    }

    // pub async fn search_users(&self) -> ServiceResult<users::Model> {}
}
