use actix_web::http::StatusCode;
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use lettre::Message;
use rand::rngs::OsRng;
use regex::Regex;

use crate::models::{MessageResponse, Response};

/// Checks and generates a new hashed password
pub fn new_password(password: &str) -> Response<Result<String, MessageResponse>> {
    let password_length = password.len();
    if password_length < 6 {
        return Ok(Err(MessageResponse::new(
            StatusCode::BAD_REQUEST,
            "Password too short (minimum 6 characters)",
        )));
    } else if password_length > 128 {
        return Ok(Err(MessageResponse::new(
            StatusCode::BAD_REQUEST,
            "Password too long (maximum 128 characters)",
        )));
    }

    Ok(Ok(Argon2::default()
        .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))?
        .to_string()))
}

lazy_static! {
    static ref USERNAME_REGEX: regex::Regex = Regex::new(r"^[A-Za-z][A-Za-z0-9_]{4,14}$").unwrap();
}

pub fn validate_username(username: &str) -> Result<(), MessageResponse> {
    let username_length = username.len();
    if username_length < 5 {
        Err(MessageResponse::new(
            StatusCode::BAD_REQUEST,
            "Username too short (minimum 5 characters)",
        ))
    } else if username_length > 15 {
        Err(MessageResponse::new(
            StatusCode::BAD_REQUEST,
            "Username too long (maximum 15 characters)",
        ))
    } else if !USERNAME_REGEX.is_match(username) {
        Err(MessageResponse::new(
            StatusCode::BAD_REQUEST,
            "Username may not contain any symbols",
        ))
    } else {
        Ok(())
    }
}

pub fn verification_email(client_url: &str, from_email: &str, email: &str, code: &str) -> Message {
    Message::builder()
        .from(from_email.parse().unwrap())
        .to(email.parse().unwrap())
        .subject("Verify your account")
        .body(
            format!(
                "Please click on this link to verify your account\n{}user/verify?code={}",
                client_url, code
            )
            .to_string(),
        )
        .unwrap()
}
