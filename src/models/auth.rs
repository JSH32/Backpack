use serde::Deserialize;

#[derive(Deserialize)]
pub struct BasicAuthForm {
    pub auth: String,
    pub password: String
}