use serde::{Deserialize, Serialize};

use crate::database::entity::{sea_orm_active_enums::Role, users};

#[derive(Serialize)]
pub struct UserData {
    pub id: String,

    #[serde(skip_serializing)]
    pub password: String,

    pub username: String,
    pub email: String,
    pub verified: bool,
    pub role: UserRole,
}

impl From<users::Model> for UserData {
    fn from(user: users::Model) -> Self {
        Self {
            id: user.id,
            password: user.password,
            username: user.username,
            email: user.email,
            verified: user.verified,
            role: UserRole::from(user.role),
        }
    }
}

/// User access level
#[derive(Serialize, Deserialize, Eq, PartialEq, PartialOrd)]
#[serde(rename_all(serialize = "lowercase", deserialize = "PascalCase"))]
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserCreateForm {
    pub username: String,
    pub email: String,
    pub password: String,
    // Only needed when invite_only
    pub registration_key: Option<String>,
}

#[derive(Deserialize)]
pub struct UserDeleteForm {
    pub password: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_password: Option<String>,

    // Always require old password to change options
    pub current_password: String,
}
