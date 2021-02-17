use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct UserCreateForm {
    pub username: String,
    pub email: String,
    pub password: String
}

#[derive(Serialize)]
pub struct UserData {
    #[serde(skip_serializing)]
    pub id: i32,

    #[serde(skip_serializing)]
    pub password: String,

    pub username: String,
    pub email: String,
    pub verified: bool
}