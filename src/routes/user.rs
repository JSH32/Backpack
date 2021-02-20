use argon2;
use http::StatusCode;

use crate::state::State;
use crate::util::auth;
use crate::models::*;
use crate::util;

use actix_web::*;

pub fn get_routes() -> Scope {
    web::scope("/user/")
        .service(create)
        .service(info)
        .service(password)
}

#[get("info")]
async fn info(state: web::Data<State>, auth: auth::middleware::User) -> impl Responder {
    match state.database.get_user_by_id(auth.0.id as u32).await {
        Ok(user_data) => HttpResponse::Ok().json(user_data),
        Err(_) => MessageResponse::internal_server_error().http_response()
    }
}

#[post("password")]
async fn password(state: web::Data<State>, auth: auth::middleware::User, form: web::Json<PasswordChangeForm>) -> impl Responder {
    let user = match state.database.get_user_by_id(auth.0.id as u32).await {
        Ok(data) => data,
        Err(_) => return MessageResponse::internal_server_error()
    };

    // Check if password is valid to password hash
    let matches = match argon2::verify_encoded(&user.password, form.current_password.as_bytes()) {
        Ok(matches) => matches,
        Err(_) => return MessageResponse::internal_server_error()
    };

    if !matches {
        return MessageResponse::new(StatusCode::BAD_REQUEST, "Incorrect password entered");
    }

    // Get new password hash
    let new_hash = match util::user::new_password(&form.new_password) {
        Ok(hash) => hash,
        Err(err) => return err
    };

    match state.database.change_password(auth.0.id as u32, &new_hash).await {
        Ok(_) => MessageResponse::new(StatusCode::OK, "Password changed successfully"),
        Err(_) => MessageResponse::internal_server_error()
    }
}

#[post("create")]
async fn create(state: web::Data<State>, mut form: web::Json<UserCreateForm>) -> impl Responder {
    // Check if username length is within bounds
    let username_length = form.username.len();
    if username_length < 4 {
        return MessageResponse::new(StatusCode::BAD_REQUEST, "Username too short (minimum 4 characters)");
    } else if username_length > 15 {
        return MessageResponse::new(StatusCode::BAD_REQUEST, "Username too long (maximum 15 characters)");
    }

    // Check if user with same email was found
    if state.database.get_user_by_email(&form.email).await.is_ok() {
        return MessageResponse::new(StatusCode::CONFLICT, "An account with that email already exists!");
    }

    // Check if user with same username was found
    if state.database.get_user_by_username(&form.username).await.is_ok() {
        return MessageResponse::new(StatusCode::CONFLICT, "An account with that username already exists!");
    }
    
    form.password = match util::user::new_password(&form.password) {
        Ok(password_hashed) => password_hashed,
        Err(err) => return err
    };

    if state.database.create_user(&form).await.is_err() {
        return MessageResponse::internal_server_error();
    }

    MessageResponse::new(StatusCode::OK, "User has successfully been created")
}