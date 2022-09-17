use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set,
};

use super::{
    auth::AuthService,
    prelude::{data_service, DataService},
    ServiceError, ServiceResult,
};
use crate::{
    database::entity::applications,
    models::{ApplicationData, TokenResponse},
};
use std::sync::Arc;

pub struct ApplicationService {
    database: Arc<DatabaseConnection>,
    auth_service: Arc<AuthService>,
}

data_service!(ApplicationService, applications);

impl ApplicationService {
    pub fn new(database: Arc<DatabaseConnection>, auth_service: Arc<AuthService>) -> Self {
        Self {
            database,
            auth_service,
        }
    }

    /// Get a token for an application.
    ///
    /// # Arguments
    /// * `id` - ID of the application.
    /// * `user_id` - User who owns the application, if there is a mismatch this will return not found.
    pub async fn generate_token(
        &self,
        id: &str,
        user_id: Option<&str>,
    ) -> ServiceResult<TokenResponse> {
        let mut condition = Condition::all().add(applications::Column::Id.eq(id.to_owned()));

        if let Some(user_id) = user_id {
            condition = condition.add(applications::Column::UserId.eq(user_id));
        }

        let application = self.by_condition(condition).await?;

        self.auth_service
            .new_jwt(&application.user_id, Some(application.id))
    }

    /// Update the last accessed date on an application to the current time.
    pub async fn update_accessed(&self, application_id: &str) -> ServiceResult<()> {
        applications::ActiveModel {
            id: Set(application_id.to_owned()),
            last_accessed: Set(Utc::now()),
            ..Default::default()
        }
        .update(self.database.as_ref())
        .await
        .map_err(|e| ServiceError::DbErr(e))?;

        Ok(())
    }

    /// Create an application.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User who owns the application.
    /// * `name` - Name of the application (must be unique).
    ///
    /// Returns [`ApplicationData`] with a token.
    pub async fn create_application(
        &self,
        user_id: &str,
        name: &str,
    ) -> ServiceResult<ApplicationData> {
        if name.len() > 16 {
            return Err(ServiceError::InvalidData(
                "Application name too long (maximum 16 characters)".into(),
            ));
        } else if name.len() < 4 {
            return Err(ServiceError::InvalidData(
                "Application name too short (minimum 4 characters)".into(),
            ));
        }

        // Application with the same name owned by the same user already exists.
        if let Some(_) = applications::Entity::find()
            .filter(applications::Column::Name.eq(name.to_owned()))
            .filter(applications::Column::UserId.eq(user_id.to_owned()))
            .one(self.database.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?
        {
            return Err(ServiceError::InvalidData(
                "An application with that name already exists".into(),
            ));
        }

        // Create an application token and send JWT to user
        let mut token_data = ApplicationData::from(
            applications::ActiveModel {
                user_id: Set(user_id.to_owned()),
                name: Set(name.to_owned()),
                ..Default::default()
            }
            .insert(self.database.as_ref())
            .await
            .map_err(|e| ServiceError::DbErr(e))?,
        );

        token_data.token = Some(
            self.auth_service
                .new_jwt(user_id, Some(token_data.id.clone()))?
                .token,
        );

        Ok(token_data)
    }
}
