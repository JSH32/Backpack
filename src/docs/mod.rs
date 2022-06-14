use utoipa::openapi::security::ApiKey;
use utoipa::openapi::security::ApiKeyValue;
use utoipa::openapi::security::SecurityScheme;
use utoipa::Modify;
use utoipa::OpenApi;

use crate::models::*;
use crate::routes;

/// Backpack API Documentation
#[derive(OpenApi)]
#[openapi(
    handlers(
		routes::info,
		routes::user::info,
		routes::user::settings,
		routes::user::create,
		routes::user::verify,
		routes::user::resend_verify
	),
    components(
		AppInfo,
		MessageResponse,
		UserData,
		UserRole,
		UpdateUserSettings,
		UserCreateForm
	),
    modifiers(&APIAuth)
)]
pub struct ApiDoc;

/// OpenAPI modifier specifying auth requirement
struct APIAuth;

impl Modify for APIAuth {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
        components.add_security_scheme(
            "apiKey",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::with_description(
                "auth-token",
                include_str!("ApiKey.md"),
            ))),
        );
    }
}
