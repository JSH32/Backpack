use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::database::entity::{sea_orm_active_enums::Role, users};

#[derive(Serialize, ToSchema)]
pub struct UserData {
    pub id: String,
    pub username: String,
    pub email: String,
    pub verified: bool,
    pub role: UserRole,
}

impl From<users::Model> for UserData {
    fn from(user: users::Model) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            verified: user.verified,
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
    /// Only needed when service is invite_only.
    pub registration_key: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct UserDeleteForm {
    pub password: String,
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

    /// Always require old password to change options.
    pub current_password: String,
}
