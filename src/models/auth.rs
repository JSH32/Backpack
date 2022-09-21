use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct BasicAuthForm {
    pub auth: String,
    pub password: String,
}

/// OAuth redirect request parameters.
#[derive(Deserialize, ToSchema)]
pub struct AuthRequest {
    pub code: String,
    pub state: String,
}

/// Enabled authorization methods.
#[derive(Serialize, ToSchema, Default)]
pub struct AuthMethods {
    /// Password authentication.
    pub password: bool,
    pub google: bool,
    pub github: bool,
    pub discord: bool,
}
