use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct BasicAuthForm {
    pub auth: String,
    pub password: String,
}
