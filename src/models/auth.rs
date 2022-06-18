use serde::Deserialize;
use utoipa::Component;

#[derive(Deserialize, Component)]
pub struct BasicAuthForm {
    pub auth: String,
    pub password: String,
}
