use crate::{
    internal::auth::{
        auth_role, get_token, AllowUnregistered, AllowUnverified, Auth, DenyApplication,
    },
    models::{
        auth::BasicAuthForm, AuthMethods, LoginRedirectUrl, OAuthLoginQuery, OAuthRequest,
        TokenResponse, UnlinkAuthMethod,
    },
    services::{
        auth::{auth_method::AuthMethodService, oauth::OAuthProvider, AuthService},
        ToResponse,
    },
};

use actix_http::header;
use actix_web::{get, http::StatusCode, post, web, HttpRequest, HttpResponse, Responder, Scope};

pub fn get_routes() -> Scope {
    web::scope("/auth")
        .service(basic)
        .service(enabled_methods)
        .service(unlink_method)
        .service(oauth_login)
        .service(oauth_callback)
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

/// Get all enabled auth methods for this user.
#[utoipa::path(
    context_path = "/api/auth",
    tag = "authentication",
    responses((status = 200, body = AuthMethods)),
)]
#[get("/methods")]
async fn enabled_methods(
    service: web::Data<AuthMethodService>,
    user: Auth<auth_role::User, AllowUnverified, DenyApplication, AllowUnregistered>,
) -> impl Responder {
    service
        .get_enabled_methods(&user.id)
        .await
        .to_response::<AuthMethods>(StatusCode::OK)
}

/// Unlink an OAuth method from a user.
#[utoipa::path(
    context_path = "/api/auth",
    tag = "authentication",
    responses(
        (status = 200, body = AuthMethods),
        (status = 400, body = MessageResponse, description = "Need at least one auth provider.")
    ),
    request_body(content = UnlinkAuthMethod)
)]
#[post("/unlink")]
async fn unlink_method(
    service: web::Data<AuthMethodService>,
    user: Auth<auth_role::User, AllowUnverified, DenyApplication, AllowUnregistered>,
    body: web::Json<UnlinkAuthMethod>,
) -> impl Responder {
    service
        .unlink_method(&user.id, body.method.into(), body.password.clone())
        .await
        .to_response::<AuthMethods>(StatusCode::OK)
}

/// Get URL for OAuth2 authentication.
/// If token is provided, this will link to the existing account.
#[utoipa::path(
    context_path = "/api/auth",
    tag = "authentication",
    security(("apiKey" = [])),
    responses((status = 200, body = LoginRedirectUrl)),
    params(OAuthLoginQuery)
)]
#[get("/oauth")]
pub async fn oauth_login(
    req: HttpRequest,
    service: web::Data<AuthService>,
    query: web::Query<OAuthLoginQuery>,
) -> impl Responder {
    let token = match get_token(&req) {
        Some(v) => match service.validate_jwt(true, &v).await {
            Ok(v) => Some(v.0.id),
            Err(_) => None,
        },
        None => None,
    };

    match service
        .oauth_login(
            query.provider,
            token,
            query.redirect.clone(),
            query.include_token,
        )
        .await
    {
        Ok(v) => HttpResponse::Ok().json(LoginRedirectUrl { url: v.to_string() }),
        Err(e) => e.to_response(),
    }
}

/// Callback for OAuth providers.
/// This redirects to the redirect provided in the oauth initialization route.
#[utoipa::path(
    context_path = "/api/auth",
    tag = "authentication",
    request_body(content = OAuthRequest),
    params(
        ("provider" = str, Path, description = "Provider to callback to.")
    )
)]
#[get("/{provider}/callback")]
pub async fn oauth_callback(
    service: web::Data<AuthService>,
    params: web::Query<OAuthRequest>,
    provider: web::Path<OAuthProvider>,
) -> impl Responder {
    match service.oauth_authenticate(*provider, &params).await {
        Ok((token, redirect)) => match redirect {
            Some(redirect) => HttpResponse::Found()
                .append_header((header::LOCATION, redirect))
                .finish(),
            None => HttpResponse::Ok().json(token),
        },
        Err(e) => e.to_response(),
    }
}
