use actix_web::http::StatusCode;
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use lettre::Message;
use rand::rngs::OsRng;

use crate::models::MessageResponse;

/// Checks and generates a new hashed password
pub fn new_password(password: &str) -> Result<String, MessageResponse> {
    let password_length = password.len();
    if password_length < 6 {
        return Err(MessageResponse::new(
            StatusCode::BAD_REQUEST,
            "Password too short (minimum 6 characters)",
        ));
    } else if password_length > 128 {
        return Err(MessageResponse::new(
            StatusCode::BAD_REQUEST,
            "Password too long (maximum 128 characters)",
        ));
    }

    Ok(Argon2::default()
        .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
        .map_err(|err| MessageResponse::internal_server_error(&err.to_string()))?
        .to_string())
}

pub fn validate_username(username: &str) -> Result<(), MessageResponse> {
    let username_length = username.len();
    if username_length < 4 {
        Err(MessageResponse::new(
            StatusCode::BAD_REQUEST,
            "Username too short (minimum 4 characters)",
        ))
    } else if username_length > 15 {
        Err(MessageResponse::new(
            StatusCode::BAD_REQUEST,
            "Username too long (maximum 15 characters)",
        ))
    } else {
        Ok(())
    }
}

pub fn verification_email(
    base_url: &str,
    from_email: &str,
    email: &str,
    code: &str,
    serve_frontend: bool,
) -> Message {
    Message::builder()
        .from(from_email.parse().unwrap())
        .to(email.parse().unwrap())
        .subject("Verify your account")
        .body(
            if serve_frontend {
                format!(
                    "Please click on this link to verify your account\n{}user/verify?code={}",
                    base_url, code
                )
            } else {
                format!(
                    "Please POST to the URL in order to verify your account\n{}api/user/verify/{}",
                    base_url, code
                )
            }
            .to_string(),
        )
        .unwrap()
}
