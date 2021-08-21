use actix_web::*;
use actix_web::cookie::Cookie;
use actix_web::http::StatusCode;
use time::OffsetDateTime;
use chrono::{DateTime, Utc};

use crate::{state::State, util::auth::{Auth, auth_role}};
use crate::util::auth::create_jwt_string;
use crate::models::{MessageResponse, auth::BasicAuthForm};

pub fn get_routes() -> Scope {
    web::scope("/auth/")
        .service(basic)
        .service(logout)
}

/// Login with email and password
#[post("basic")]
async fn basic(state: web::Data<State>, data: web::Json<BasicAuthForm>) -> impl Responder {
    // Get user data from database
    let user_data = match state.database.get_user_by_email(&data.email).await {
        Ok(user_data) => user_data,
        Err(_) => return MessageResponse::new(StatusCode::BAD_REQUEST, "Invalid credentials provided!").http_response()
    };

    // Check if password is valid to password hash
    let matches = match argon2::verify_encoded(&user_data.password, data.password.as_bytes()) {
        Ok(matches) => matches,
        Err(_) => return MessageResponse::internal_server_error().http_response()
    };

    if !matches {
        return MessageResponse::new(StatusCode::BAD_REQUEST, "Invalid credentials provided!").http_response();
    }

    let expire_time = (Utc::now() + chrono::Duration::weeks(1)).timestamp();
    let jwt = match create_jwt_string(user_data.id, None, "localhost", Some(expire_time), &state.jwt_key) {
        Ok(jwt) => jwt,
        Err(_) => return MessageResponse::internal_server_error().http_response()
    };

    // Set JWT token as cookie
    HttpResponse::Ok()
        .cookie(
            Cookie::build("auth-token", jwt)
            .secure(false)
            .http_only(true)
            .path("/")
            .expires(OffsetDateTime::from_unix_timestamp(expire_time))
            .finish()
        )
        .json(user_data)
}

/// Remove httponly cookie
#[post("logout")]
async fn logout(_: Auth<auth_role::User, true, false>) -> impl Responder {
    HttpResponse::Ok()
        .cookie(
            Cookie::build("auth-token", "")
            .secure(false)
            .http_only(true)
            .path("/")
            // Cookie expires instantly when issued, will remove the cookie
            .expires(OffsetDateTime::from_unix_timestamp(Utc::now().timestamp()))
            .finish()
        )
        .json(MessageResponse::new(StatusCode::OK, "Successfully logged out"))
}