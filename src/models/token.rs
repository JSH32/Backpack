use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TokenData {
    pub description: String,
    pub name: String,
    pub token: String
}