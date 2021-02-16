use argon2;
use http::StatusCode;
use rand::Rng;

use crate::state::{State};
use crate::models::user::*;
use crate::models::*;

use actix_web::*;

pub fn get_routes() -> Scope {
    web::scope("/user/")
        .service(create)
}

#[post("create")]
pub async fn create(state: web::Data<State>, mut data: web::Json<UserCreateForm>) -> impl Responder {
    // Check if username length is within bounds
    let username_length = data.username.len();
    if username_length < 4 {
        return HttpResponse::BadRequest()
                            .json(new_error(StatusCode::BAD_REQUEST, "Username too short (minimum 4 characters)"));
    } else if username_length > 15 {
        return HttpResponse::BadRequest()
                            .json(new_error(StatusCode::BAD_REQUEST, "Username too long (maximum 15 characters)"));
    }

    // Check if password length is within bounds
    let password_length = data.password.len();
    if password_length < 6 {
        return HttpResponse::BadRequest()
                            .json(new_error(StatusCode::BAD_REQUEST, "Password too short (minimum 6 characters)"));
    } else if password_length > 128 {
        return HttpResponse::BadRequest()
                            .json(new_error(StatusCode::BAD_REQUEST, "Password too long (maximum 128 characters)"));
    }

    // Check if user with same email was found
    if state.database.get_user_by_email(&data.email).await.is_ok() {
        return HttpResponse::Conflict()
                            .json(new_error(StatusCode::CONFLICT, "An account with that email already exists!"));
    }

    // Check if user with same username was found
    if state.database.get_user_by_username(&data.username).await.is_ok() {
        return HttpResponse::Conflict()
                            .json(new_error(StatusCode::CONFLICT, "An account with that username already exists!"));
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
            return new_internal_server_error();
        }
    };

    data.password = hash;

    if state.database.create_user(&data).await.is_err() {
        return new_internal_server_error();
    }

    HttpResponse::Ok()
        .json(new_message(StatusCode::OK, "User has successfully been created"))
}