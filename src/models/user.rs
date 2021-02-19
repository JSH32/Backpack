use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct UserCreateForm {
    pub username: String,
    pub email: String,
    pub password: String
}

/// User role in database
#[derive(Serialize, Deserialize, sqlx::Type, PartialEq, PartialOrd)]
#[sqlx(rename = "role", rename_all = "lowercase")]
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