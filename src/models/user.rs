use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct UserCreateForm {
    pub username: String,
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename = "role")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all(serialize  = "lowercase"))]
#[serde(rename_all(deserialize  = "UPPERCASE"))]
pub enum UserRole {
    USER,
    ADMIN
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