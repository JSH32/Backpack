use crate::{
    models::{auth::BasicAuthForm, AuthRequest, TokenResponse},
    services::{
        auth::{AuthService, OAuthProvider},
        ServiceError, ToResponse,
    },
};

use actix_http::header;
use actix_web::{get, http::StatusCode, post, web, HttpResponse, Responder, Scope};

pub fn get_routes() -> Scope {
    web::scope("/auth")
        .service(basic)
        .service(google_login)
        .service(google_auth)
        .service(github_login)
        .service(github_auth)
        .service(discord_login)
        .service(discord_auth)
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

macro_rules! define_oauth_route_auth {
    ($auth_service:expr, $auth_params:expr, $variant:expr) => {
        HttpResponse::Found()
            .append_header((
                header::LOCATION,
                match $auth_service
                    .oauth_authenticate($variant, $auth_params)
                    .await
                {
                    // TODO: Allow frontend to be disabled and instead just return the raw token response.
                    // Frontend should get this parameter on load and put the token into headers.
                    Ok(v) => format!("{}/user/login?token={}", $auth_service.client_url, v.token),
                    Err(e) => format!(
                        "{}/user/login?fail={}",
                        $auth_service.client_url,
                        match e {
                            ServiceError::ServerError(_) | ServiceError::DbErr(_) =>
                                return e.to_response(),
                            _ => e.to_string(),
                        }
                    ),
                },
            ))
            .finish()
    }
}

macro_rules! define_oauth_route_login {
    ($auth_service:expr, $variant:expr) => {
        match $auth_service.oauth_login($variant) {
            Ok(v) => HttpResponse::Found()
                .append_header((header::LOCATION, v.to_string()))
                .finish(),
            Err(e) => e.to_response(),
        }
    };
}

/// Initiate Google OAuth authentication.
/// This redirects to google to authenticate the user.
#[utoipa::path(
    context_path = "/api/auth",
    tag = "authentication",
    responses((status = 200, body = TokenResponse)),
)]
#[get("/google/login")]
async fn google_login(service: web::Data<AuthService>) -> impl Responder {
    define_oauth_route_login!(service, OAuthProvider::Google)
}

/// Google OAuth redirect URL.
/// This redirects to frontend with token if a valid user was found with the parameters.
#[utoipa::path(
    context_path = "/api/auth",
    tag = "authentication",
    responses((status = 200, body = TokenResponse)),
    request_body(content = AuthRequest)
)]
#[get("/google/auth")]
async fn google_auth(
    service: web::Data<AuthService>,
    params: web::Query<AuthRequest>,
) -> impl Responder {
    define_oauth_route_auth!(service, &params, OAuthProvider::Google)
}

/// Initiate Github OAuth authentication.
/// This redirects to github to authenticate the user.
#[utoipa::path(
    context_path = "/api/auth",
    tag = "authentication",
    responses((status = 200, body = TokenResponse)),
)]
#[get("/github/login")]
async fn github_login(service: web::Data<AuthService>) -> impl Responder {
    define_oauth_route_login!(service, OAuthProvider::Github)
}

/// Github OAuth redirect URL.
/// This redirects to frontend with token if a valid user was found with the parameters.
#[utoipa::path(
    context_path = "/api/auth",
    tag = "authentication",
    responses((status = 200, body = TokenResponse)),
    request_body(content = AuthRequest)
)]
#[get("/github/auth")]
async fn github_auth(
    service: web::Data<AuthService>,
    params: web::Query<AuthRequest>,
) -> impl Responder {
    define_oauth_route_auth!(service, &params, OAuthProvider::Github)
}

/// Initiate Discord OAuth authentication.
/// This redirects to discord to authenticate the user.
#[utoipa::path(
    context_path = "/api/auth",
    tag = "authentication",
    responses((status = 200, body = TokenResponse)),
)]
#[get("/discord/login")]
async fn discord_login(service: web::Data<AuthService>) -> impl Responder {
    define_oauth_route_login!(service, OAuthProvider::Discord)
}

/// Discord OAuth redirect URL.
/// This redirects to frontend with token if a valid user was found with the parameters.
#[utoipa::path(
    context_path = "/api/auth",
    tag = "authentication",
    responses((status = 200, body = TokenResponse)),
    request_body(content = AuthRequest)
)]
#[get("/discord/auth")]
async fn discord_auth(
    service: web::Data<AuthService>,
    params: web::Query<AuthRequest>,
) -> impl Responder {
    define_oauth_route_auth!(service, &params, OAuthProvider::Discord)
}
