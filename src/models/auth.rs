use serde::{Deserialize};

#[derive(Deserialize)]
pub struct BasicAuthForm {
    pub email: String,
    pub password: String
}