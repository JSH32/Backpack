use utoipa::openapi::security::HttpAuthScheme;
use utoipa::openapi::security::HttpBuilder;
use utoipa::openapi::security::SecurityScheme;
use utoipa::openapi::LicenseBuilder;
use utoipa::Modify;
use utoipa::OpenApi;

use crate::models::*;

// Utoipa reads by the tag by name provided
// This is a hack to proxy through modules so it is recognized as "server"
mod server {
    pub(crate) use crate::routes::*;
}

use crate::routes::user;

/// Backpack API Documentation
#[derive(OpenApi)]
#[openapi(
    handlers(
		server::info, 
		user::info, 
		user::settings, 
		user::create, 
		user::verify, 
		user::resend_verify
	),
    components(
		AppInfo, 
		MessageResponse, 
		UserData, 
		UserRole, 
		UpdateUserSettings,
		UserCreateForm
	),
	tags(
		(name = "server", description = "Server information endpoints."),
		(name = "user", description = "User management endpoints.")
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
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .description(Some(include_str!("ApiKey.md")))
                    .build(),
            ),
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
