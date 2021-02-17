use argon2;
use http::StatusCode;
use rand::Rng;

use crate::{models::auth::UserID, state::State};
use crate::models::user::*;
use crate::models::*;

use actix_web::*;

pub fn get_routes() -> Scope {
    web::scope("/user/")
        .service(create)
        .service(info)
}

#[get("info")]
async fn info(state: web::Data<State>, user_id: UserID) -> Response {
    match state.database.get_user_by_id(user_id.0 as u32).await {
        Ok(user_data) => Response::new_data(StatusCode::OK, false, serde_json::to_value(user_data).unwrap()),
        Err(_) => Response::internal_server_error()
    }
}

#[post("create")]
async fn create(state: web::Data<State>, mut data: web::Json<UserCreateForm>) -> Response {
    // Check if username length is within bounds
    let username_length = data.username.len();
    if username_length < 4 {
        return Response::new_message(StatusCode::BAD_REQUEST, false, "Username too short (minimum 4 characters)");
    } else if username_length > 15 {
        return Response::new_message(StatusCode::BAD_REQUEST, false, "Username too long (maximum 15 characters)");
    }

    // Check if password length is within bounds
    let password_length = data.password.len();
    if password_length < 6 {
        return Response::new_message(StatusCode::BAD_REQUEST, false, "Password too short (minimum 6 characters)");
    } else if password_length > 128 {
        return Response::new_message(StatusCode::BAD_REQUEST, false, "Password too long (maximum 128 characters)");
    }

    // Check if user with same email was found
    if state.database.get_user_by_email(&data.email).await.is_ok() {
        return Response::new_message(StatusCode::CONFLICT, true, "An account with that email already exists!");
    }

    // Check if user with same username was found
    if state.database.get_user_by_username(&data.username).await.is_ok() {
        return Response::new_message(StatusCode::CONFLICT, true, "An account with that username already exists!");
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
            return Response::internal_server_error();
        }
    };

    data.password = hash;

    if state.database.create_user(&data).await.is_err() {
        return Response::internal_server_error();
    }

    Response::new_message(StatusCode::OK, false, "User has successfully been created")
}