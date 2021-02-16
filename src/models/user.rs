use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct UserData {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct UserCreateForm {
    pub username: String,
    pub email: String,
    pub password: String
}