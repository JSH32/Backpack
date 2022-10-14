use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::database::entity::{sea_orm_active_enums::Role, users};

#[derive(Serialize, ToSchema)]
pub struct UserData {
    pub id: String,
    pub username: String,
    /// This will not be present if accessed by another user.
    pub email: Option<String>,
    /// This will not be present if accessed by another user.
    pub verified: Option<bool>,
    /// Has the user already verified with a registration key?
    /// This will be true always if service is in `invite_only` mode.
    /// This will not be present if accessed by another user.
    pub registered: Option<bool>,
    pub role: UserRole,
}

impl From<users::Model> for UserData {
    fn from(user: users::Model) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: Some(user.email),
            verified: Some(user.verified),
            registered: Some(user.registered),
            role: UserRole::from(user.role),
        }
    }
}

/// User access level (user, admin)
#[derive(Serialize, Deserialize, Eq, PartialEq, PartialOrd, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum UserRole {
    User,
    Admin,
}

impl From<Role> for UserRole {
    fn from(role: Role) -> Self {
        match role {
            Role::Admin => UserRole::Admin,
            Role::User => UserRole::User,
        }
    }
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserCreateForm {
    pub username: String,
    pub email: String,
    pub password: String,
    /// Required when creating a user with password.
    pub registration_key: Option<String>,
}

/// Used when registering an existing account with a registration key.
#[derive(Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationParams {
    /// This doesn't have to be provided if an admin is calling this route.
    pub key: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct UserDeleteForm {
    /// This is required if a password has been set prior.
    pub password: Option<String>,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_password: Option<String>,

    /// This is required if a password has been set prior.
    /// This is optional if the requesting user is an admin modifying another user.
    pub current_password: Option<String>,
}
