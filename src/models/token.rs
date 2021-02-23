use serde::Serialize;

#[derive(Serialize)]
pub struct TokenData {
    pub name: String,
    pub description: String,
    pub token: String
}