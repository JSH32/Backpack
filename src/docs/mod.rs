use utoipa::openapi::security::ApiKey;
use utoipa::openapi::security::ApiKeyValue;
use utoipa::openapi::security::SecurityScheme;
use utoipa::openapi::LicenseBuilder;
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
		routes::user::resend_verify,
		routes::file::upload
	),
    components(
		AppInfo,
		MessageResponse,
		UserData,
		UserRole,
		UpdateUserSettings,
		UserCreateForm,
		UploadFile
	),
    modifiers(&ApiModifier)
)]
pub struct ApiDoc;

/// Runtime extra configuration for the documentation
struct ApiModifier;

impl Modify for ApiModifier {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        // Authentication
        let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
        components.add_security_scheme(
            "apiKey",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::with_description(
                "auth-token",
                include_str!("ApiKey.md"),
            ))),
        );

        // License
        openapi.info.license = Some(
            LicenseBuilder::default()
                .name("MIT")
                .url(Some(
                    "https://github.com/JSH32/Backpack/blob/rewrite/LICENSE",
                ))
                .build(),
        )
    }
}
