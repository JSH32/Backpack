use crate::{
    database::entity::{auth_methods, sea_orm_active_enums::AuthMethod, users},
    models::AuthMethods,
    services::{
        prelude::{data_service, DataService},
        ServiceError, ServiceResult, ToOption,
    },
};

use chrono::Utc;
use sea_orm::{prelude::*, Condition, IntoActiveModel, Set};
use std::sync::Arc;

use super::{new_password, validate_password};

#[derive(Debug)]
pub struct AuthMethodService {
    database: Arc<DatabaseConnection>,
}

data_service!(AuthMethodService, auth_methods);

impl AuthMethodService {
    pub fn new(database: Arc<DatabaseConnection>) -> Self {
        Self { database }
    }

    /// Get authentication method for a user.
    pub async fn get_auth_method(
        &self,
        user_id: &str,
        method: AuthMethod,
    ) -> ServiceResult<auth_methods::Model> {
        self.by_condition(
            Condition::all()
                .add(auth_methods::Column::AuthMethod.eq(method))
                .add(auth_methods::Column::UserId.eq(user_id.to_owned())),
        )
        .await
    }

    /// Get all methods enabled for a user.
    pub async fn get_enabled_methods(&self, user_id: &str) -> ServiceResult<AuthMethods> {
        let found_methods = auth_methods::Entity::find()
            .filter(auth_methods::Column::UserId.eq(user_id))
            .all(self.database.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?;

        let mut methods = AuthMethods::default();

        for method in found_methods {
            // These should all be true except for password.
            match method.auth_method {
                AuthMethod::Discord => methods.discord = method.cached_username,
                AuthMethod::Github => methods.github = method.cached_username,
                AuthMethod::Google => methods.google = method.cached_username,
                AuthMethod::Password => methods.password = true,
            };
        }

        Ok(methods)
    }

    /// Unlink an auth method from a user.
    pub async fn unlink_method(
        &self,
        user_id: &str,
        method: AuthMethod,
        password: Option<String>,
    ) -> ServiceResult<AuthMethods> {
        let method = self.get_auth_method(user_id, method).await?;

        // Validate password if exists
        if let Some(password_method) = self
            .get_auth_method(user_id, AuthMethod::Password)
            .await
            .to_option()?
        {
            if let Some(password) = password {
                validate_password(&password_method.value, &password)?;
            } else {
                return Err(ServiceError::InvalidData(
                    "Password is required to remove an auth method.".into(),
                ));
            }
        }

        let methods = self.get_enabled_methods(user_id).await?;
        if methods.enabled_methods() <= 1 {
            return Err(ServiceError::InvalidData(
                "You need at least one active authentication method.".into(),
            ));
        }

        method
            .delete(self.database.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?;

        self.get_enabled_methods(user_id).await
    }

    /// Get a user by authentication method and value.
    /// This also updates `last_accessed`
    pub async fn get_user_by_value(
        &self,
        method: AuthMethod,
        value: &str,
        new_cached_username: Option<String>,
    ) -> ServiceResult<Option<users::Model>> {
        if let Some(v) = self
            .by_condition(
                Condition::all()
                    .add(auth_methods::Column::AuthMethod.eq(method))
                    .add(auth_methods::Column::Value.eq(value.to_owned())),
            )
            .await
            .to_option()?
        {
            let user = v
                .find_related(users::Entity)
                .one(self.database.as_ref())
                .await
                .map_err(|e| ServiceError::DbErr(e))?;

            // Update `last_accessed`.
            let mut active_method = v.into_active_model();
            active_method.last_accessed = Set(Utc::now());
            active_method.cached_username = Set(new_cached_username);

            active_method
                .update(self.database.as_ref())
                .await
                .map_err(|e| ServiceError::DbErr(e))?;

            Ok(user)
        } else {
            Ok(None)
        }
    }

    /// Creates or sets the value of an auth method on a user.
    pub async fn create_or_set_method(
        &self,
        user_id: &str,
        method: AuthMethod,
        cached_username: Option<String>,
        value: &str,
    ) -> ServiceResult<auth_methods::Model> {
        match auth_methods::Entity::find()
            .filter(auth_methods::Column::AuthMethod.eq(method.clone()))
            .filter(auth_methods::Column::UserId.eq(user_id.to_owned()))
            .one(self.database.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?
        {
            Some(v) => {
                let mut active_method = v.clone().into_active_model();
                active_method.value = Set(match v.auth_method {
                    AuthMethod::Password => new_password(&value)?,
                    _ => value.to_owned(),
                });

                active_method.last_accessed = Set(Utc::now());
                active_method
                    .update(self.database.as_ref())
                    .await
                    .map_err(|e| ServiceError::DbErr(e))
            }
            None => {
                self.create_auth_method(user_id, method, cached_username, value)
                    .await
            }
        }
    }

    /// Create and validate auth methods.
    pub async fn create_auth_method(
        &self,
        user_id: &str,
        method: AuthMethod,
        cached_username: Option<String>,
        value: &str,
    ) -> ServiceResult<auth_methods::Model> {
        let value = match method {
            AuthMethod::Password => new_password(&value)?,
            _ => value.to_owned(),
        };

        auth_methods::ActiveModel {
            user_id: Set(user_id.to_owned()),
            auth_method: Set(method),
            cached_username: Set(cached_username),
            value: Set(value),
            ..Default::default()
        }
        .insert(self.database.as_ref())
        .await
        .map_err(|e| ServiceError::DbErr(e))
    }
}
