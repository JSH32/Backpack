use argon2;
use auth::middleware;
use http::StatusCode;
use rand::Rng;

use crate::{state::State};
use crate::models::user::*;
use crate::models::*;

use actix_web::*;

pub fn get_routes() -> Scope {
    web::scope("/user/")
        .service(create)
        .service(info)
}

#[get("info")]
async fn info(state: web::Data<State>, auth: auth::middleware::User) -> impl Responder {
    match state.database.get_user_by_id(auth.0.id as u32).await {
        Ok(user_data) => HttpResponse::Ok().json(user_data),
        Err(_) => MessageResponse::internal_server_error().http_response()
    }
}

#[post("create")]
async fn create(state: web::Data<State>, mut data: web::Json<UserCreateForm>) -> impl Responder {
    // Check if username length is within bounds
    let username_length = data.username.len();
    if username_length < 4 {
        return MessageResponse::new(StatusCode::BAD_REQUEST, "Username too short (minimum 4 characters)");
    } else if username_length > 15 {
        return MessageResponse::new(StatusCode::BAD_REQUEST, "Username too long (maximum 15 characters)");
    }

    // Check if password length is within bounds
    let password_length = data.password.len();
    if password_length < 6 {
        return MessageResponse::new(StatusCode::BAD_REQUEST, "Password too short (minimum 6 characters)");
    } else if password_length > 128 {
        return MessageResponse::new(StatusCode::BAD_REQUEST, "Password too long (maximum 128 characters)");
    }

    // Check if user with same email was found
    if state.database.get_user_by_email(&data.email).await.is_ok() {
        return MessageResponse::new(StatusCode::CONFLICT, "An account with that email already exists!");
    }

    // Check if user with same username was found
    if state.database.get_user_by_username(&data.username).await.is_ok() {
        return MessageResponse::new(StatusCode::CONFLICT, "An account with that username already exists!");
    }
    
    // Generate a random salt
    let salt: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(36)
        .map(char::from)
        .collect();

    let hash = match argon2::hash_encoded(data.password.as_bytes(), salt.as_bytes(), &argon2::Config::default()) {
        Ok(hash) => hash,
        Err(_) => {
            // Return error if hash could not be produced for whatever reason
            return MessageResponse::internal_server_error();
        }
    };

    data.password = hash;

    if state.database.create_user(&data).await.is_err() {
        return MessageResponse::internal_server_error();
    }

    MessageResponse::new(StatusCode::OK, "User has successfully been created")
}