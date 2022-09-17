use crate::{
    models::{auth::BasicAuthForm, TokenResponse},
    services::{auth::AuthService, ToResponse},
};

use actix_web::{http::StatusCode, post, web, Responder, Scope};

pub fn get_routes() -> Scope {
    web::scope("/auth").service(basic)
}

/// Login with email and password
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
