use actix_web::http::StatusCode;
use rand::Rng;

use crate::models::MessageResponse;

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

    let hash = match argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &argon2::Config::default()) {
        Ok(hash) => hash,
        Err(_) => {
            // Return error if hash could not be produced for whatever reason
            return Err(MessageResponse::internal_server_error());
        }
    };

    Ok(hash)
}