use crate::{
    models::{auth::BasicAuthForm, AuthRequest, TokenResponse},
    services::{
        auth::{oauth::OAuthProvider, AuthService},
        ServiceError, ToResponse,
    },
};

use actix_http::header;
use actix_web::{get, http::StatusCode, post, web, HttpResponse, Responder, Scope};

pub fn get_routes() -> Scope {
    web::scope("/auth")
        .service(basic)
        .service(google::google_login)
        .service(google::google_auth)
        .service(github::github_login)
        .service(github::github_auth)
        .service(discord::discord_login)
        .service(discord::discord_auth)
}

/// Login with email and password.
#[utoipa::path(
    context_path = "/api/auth",
    tag = "authentication",
    responses(
        (status = 200, body = TokenResponse),
        (status = 400, body = MessageResponse, description = "Invalid credentials"),
    ),
    request_body(content = BasicAuthForm)
)]
#[post("/basic")]
async fn basic(service: web::Data<AuthService>, form: web::Json<BasicAuthForm>) -> impl Responder {
    service
        .password_auth(&form.auth, &form.password)
        .await
        .to_response::<TokenResponse>(StatusCode::OK)
}

// INFO: Should we actually be documenting these routes in OpenAPI?
// It will generate a bunch of API routes on the client and most of the time this is not directly accessed.
// This is especially true for the oauth callback.
macro_rules! define_oauth_route {(
    $provider_name:ident,
    $provider:expr $(,)?
) => (
    pub mod $provider_name {
        use super::*;
        use paste::paste;
        use with_builtin_macros::with_builtin;

        with_builtin!(let $path = concat!("/", stringify!($provider_name), "/login") in {
            paste! {
                #[doc = [<$provider_name:camel>] " OAuth2 authentication."]
                #[doc = "Redirects to " [<$provider_name:camel>] " to authenticate the user."]
                #[utoipa::path(
                    context_path = "/api/auth",
                    tag = "authentication",
                )]
                #[get($path)]
                pub async fn [<$provider_name _login>](service: web::Data<AuthService>) -> impl Responder {
                    match service.oauth_login($provider) {
                        Ok(v) => HttpResponse::Found()
                            .append_header((header::LOCATION, v.to_string()))
                            .finish(),
                        Err(e) => e.to_response(),
                    }
                }
            }
        });

        with_builtin!(let $path = concat!("/", stringify!($provider_name), "/auth") in {
            paste! {
                #[doc = "Callback for " [<$provider_name:camel>] " OAuth provider."]
                #[doc = "This redirects to frontend with token if a valid user was found with the parameters."]
                #[utoipa::path(
                    context_path = "/api/auth",
                    tag = "authentication",
                    request_body(content = AuthRequest)
                )]
                #[get($path)]
                pub async fn [<$provider_name _auth>](
                    service: web::Data<AuthService>,
                    params: web::Query<AuthRequest>,
                ) -> impl Responder {
                    HttpResponse::Found()
                        .append_header((
                            header::LOCATION,
                            match service
                                .oauth_authenticate($provider, &params)
                                .await
                            {
                                // TODO: Allow frontend to be disabled and instead just return the raw token response.
                                // Frontend should get this parameter on load and put the token into headers.
                                Ok(v) => format!("{}/user/login?token={}", service.client_url, v.token),
                                Err(e) => format!(
                                    "{}/user/login?fail={}",
                                    service.client_url,
                                    match e {
                                        ServiceError::ServerError(_) | ServiceError::DbErr(_) =>
                                            return e.to_response(),
                                        _ => e.to_string(),
                                    }
                                ),
                            }
                        )).finish()
                }
            }
        });
    }
)}

define_oauth_route!(google, OAuthProvider::Google);
define_oauth_route!(github, OAuthProvider::Github);
define_oauth_route!(discord, OAuthProvider::Discord);
