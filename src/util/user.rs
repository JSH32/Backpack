use actix_web::http::StatusCode;
use lettre::Message;
use rand::Rng;
use regex::Regex;

use crate::models::MessageResponse;

lazy_static! {
    pub static ref EMAIL_REGEX: regex::Regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
}

/// Checks and generates a new hashed password
pub fn new_password(password: &str) -> Result<String, MessageResponse> {
    let password_length = password.len();
    if password_length < 6 {
        return Err(MessageResponse::new(StatusCode::BAD_REQUEST, "Password too short (minimum 6 characters)"));
    } else if password_length > 128 {
        return Err(MessageResponse::new(StatusCode::BAD_REQUEST, "Password too long (maximum 128 characters)"));
    }

    // Generate a random salt
    let salt: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(36)
        .map(char::from)
        .collect();
        
    Ok(argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &argon2::Config::default())
        .map_err(|_| MessageResponse::internal_server_error())?)
}

pub fn verification_email(base_url: &str, from_email: &str, email: &str, code: &str,) -> Message {
    Message::builder()
        .from(from_email.parse().unwrap())
        .to(email.parse().unwrap())
        .subject("Verify your account")
        .body(String::from(format!("Please click on this link to verify your account\n{}/user/verify?code={}", base_url, code)))
        .unwrap()
}