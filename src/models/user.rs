use macro_rules_attribute::macro_rules_derive;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct UserCreateForm {
    pub username: String,
    pub email: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct UserDeleteForm {
    pub password: String
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordChangeForm {
    pub current_password: String,
    pub new_password: String
}

/// User access level
#[derive(Serialize, Deserialize, sqlx::Type, PartialEq, PartialOrd)]
#[sqlx(type_name = "role", rename_all = "lowercase")]
#[serde(rename_all(serialize  = "lowercase", deserialize  = "PascalCase"))]
pub enum UserRole {
    User,
    Admin
}

#[derive(Serialize)]
pub struct UserData {
    #[serde(skip_serializing)]
    pub id: i32,

    #[serde(skip_serializing)]
    pub password: String,

    pub username: String,
    pub email: String,
    pub verified: bool,
    pub role: UserRole
}