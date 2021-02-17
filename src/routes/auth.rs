use actix_web::*;
use actix_web::http::StatusCode;
use hmac::{Hmac};
use jwt::{RegisteredClaims, SignWithKey};
use models::*;
use sha2::Sha256;
use time::OffsetDateTime;
use chrono::{DateTime, Utc};

use crate::{models::{self, auth::BasicAuthForm}, state::State};

pub fn get_routes() -> Scope {
    web::scope("/auth/")
        .service(basic)
}

// Sign a JWT token and get a string
fn create_jwt_string(id: i32, issuer: &str, timestamp: i64, key: &Hmac<Sha256>) -> Result<String, jwt::Error> {
    let claims = RegisteredClaims {
        issuer: Some(issuer.into()),
        subject: Some(id.to_string().into()),
        expiration: Some(timestamp as u64),
        ..Default::default()
    };

    claims.sign_with_key(key)
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

    let utc: DateTime<Utc> = Utc::now();
    let one_week = chrono::Duration::weeks(1);
    let expire_time = (utc + one_week).timestamp();

    let jwt = match create_jwt_string(user_data.id, "localhost", expire_time, &state.jwt_key) {
        Ok(jwt) => jwt,
        Err(_) => return MessageResponse::internal_server_error().http_response()
    };

    // Set JWT token as cookie
    HttpResponse::Ok()
        .cookie(
            http::Cookie::build("auth-token", jwt)
            .secure(false)
            .http_only(true)
            .path("/")
            .expires(OffsetDateTime::from_unix_timestamp(expire_time))
            .finish()
        )
        .json(MessageResponse::new(StatusCode::OK, "You have logged in"))
}