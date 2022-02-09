use argon2;
use lettre::AsyncTransport;

use crate::{
    models::{MessageResponse, PasswordChangeForm, UserCreateForm, UserEmailForm},
    state::State,
    util::{
        auth::{auth_role, Auth},
        random_string,
        user::{new_password, verification_email},
        EMAIL_REGEX,
    },
};

use actix_web::{get, http::StatusCode, post, web, HttpResponse, Responder, Scope};

pub fn get_routes(smtp_verification: bool) -> Scope {
    let scope = web::scope("/user/")
        .service(create)
        .service(info)
        .service(password);

    if smtp_verification {
        scope.service(verify)
    } else {
        scope
    }
}

#[get("info")]
async fn info(auth: Auth<auth_role::User, true, true>) -> impl Responder {
    HttpResponse::Ok().json(auth.user)
}

#[post("password")]
async fn password(
    state: web::Data<State>,
    auth: Auth<auth_role::User, false, false>,
    form: web::Json<PasswordChangeForm>,
) -> impl Responder {
    // Check if password is valid to password hash
    let matches =
        match argon2::verify_encoded(&auth.user.password, form.current_password.as_bytes()) {
            Ok(matches) => matches,
            Err(_) => return MessageResponse::internal_server_error(),
        };

    if !matches {
        return MessageResponse::new(StatusCode::BAD_REQUEST, "Incorrect password entered");
    }

    // Get new password hash
    let new_hash = match new_password(&form.new_password) {
        Ok(hash) => hash,
        Err(err) => return err,
    };

    match state
        .database
        .change_password(&auth.user.id, &new_hash)
        .await
    {
        Ok(_) => MessageResponse::new(StatusCode::OK, "Password changed successfully"),
        Err(_) => MessageResponse::internal_server_error(),
    }
}

#[post("create")]
async fn create(state: web::Data<State>, mut form: web::Json<UserCreateForm>) -> impl Responder {
    // Check if username length is within bounds
    let username_length = form.username.len();
    if username_length < 4 {
        return MessageResponse::new(
            StatusCode::BAD_REQUEST,
            "Username too short (minimum 4 characters)",
        );
    } else if username_length > 15 {
        return MessageResponse::new(
            StatusCode::BAD_REQUEST,
            "Username too long (maximum 15 characters)",
        );
    }

    if !EMAIL_REGEX.is_match(&form.email) {
        return MessageResponse::new(StatusCode::BAD_REQUEST, "Invalid email was provided");
    }

    // Check if user with same email was found
    if state.database.get_user_by_email(&form.email).await.is_ok() {
        return MessageResponse::new(
            StatusCode::CONFLICT,
            "An account with that email already exists!",
        );
    }

    // Check if user with same username was found
    if state
        .database
        .get_user_by_username(&form.username)
        .await
        .is_ok()
    {
        return MessageResponse::new(
            StatusCode::CONFLICT,
            "An account with that username already exists!",
        );
    }

    form.password = match new_password(&form.password) {
        Ok(password_hashed) => password_hashed,
        Err(err) => return err,
    };

    match state.database.create_user(&form).await {
        Ok(user_data) => {
            if let Some(smtp) = &state.smtp_client {
                let random_code = random_string(72);
                if !state
                    .database
                    .create_verification(&user_data.id, &random_code)
                    .await
                    .is_err()
                {
                    let email = verification_email(
                        &state.base_url.to_string(),
                        &smtp.1,
                        &user_data.email,
                        &random_code,
                    );
                    let mailer = smtp.clone().0;
                    tokio::spawn(async move {
                        let _ = mailer.send(email).await;
                    });
                }
            } else {
                // If SMTP is disabled we just verify the user
                let _ = state.database.verify_user(&user_data.id, true).await;
            }
        }
        Err(_) => return MessageResponse::internal_server_error(),
    }

    MessageResponse::new(StatusCode::OK, "User has successfully been created")
}

#[post("/email")]
async fn change_email(
    state: web::Data<State>,
    auth: Auth<auth_role::User, true, false>,
    form: web::Form<UserEmailForm>,
) -> impl Responder {
    if !EMAIL_REGEX.is_match(&form.email) {
        return MessageResponse::new(StatusCode::BAD_REQUEST, "Invalid email was provided");
    }

    if state.database.get_user_by_email(&form.email).await.is_ok() {
        return MessageResponse::new(
            StatusCode::CONFLICT,
            "An account with that email already exists!",
        );
    }

    if let Err(_) = state
        .database
        .change_email(&auth.user.id, &form.email)
        .await
    {
        return MessageResponse::internal_server_error();
    }

    match &state.smtp_client {
        Some(smtp) => {
            if let Err(_) = state.database.verify_user(&auth.user.id, false).await {
                return MessageResponse::internal_server_error();
            }

            let random_code = random_string(72);
            if !state
                .database
                .create_verification(&auth.user.id, &random_code)
                .await
                .is_err()
            {
                let email = verification_email(
                    &state.base_url.to_string(),
                    &smtp.1,
                    &form.email,
                    &random_code,
                );
                let mailer = smtp.clone().0;
                tokio::spawn(async move {
                    let _ = mailer.send(email).await;
                });
            }

            MessageResponse::new(
                StatusCode::OK,
                &format!(
                    "User email was changed, verification email was sent to {}",
                    &auth.user.email
                ),
            )
        }
        None => MessageResponse::new(
            StatusCode::OK,
            &format!("User email was changed to {}", &form.email),
        ),
    }
}

#[post("/verify/{code}")]
async fn verify(state: web::Data<State>, code: web::Path<String>) -> impl Responder {
    match state.database.get_user_from_verification(&code).await {
        Ok(user_data) => {
            if state
                .database
                .delete_verification(&user_data.id)
                .await
                .is_err()
            {
                return MessageResponse::internal_server_error();
            }

            // This case can ONLY happen if SMTP verification is disabled, the user tries to access their account, and THEN re-enables
            if user_data.verified {
                return MessageResponse::new(StatusCode::CONFLICT, "User was already verified");
            }

            if state
                .database
                .verify_user(&user_data.id, true)
                .await
                .is_err()
            {
                return MessageResponse::internal_server_error();
            }

            MessageResponse::new(StatusCode::OK, "User has been verified")
        }
        Err(_) => return MessageResponse::bad_request(),
    }
}

// This needs to delete every file owned by the user
// #[post("delete")]
// async fn delete(state: web::Data<State>, auth: auth::middleware::User, form: web::Json<UserDeleteForm>) -> impl Responder {
//     let matches = match argon2::verify_encoded(&auth.0.password, form.current_password.as_bytes()) {
//         Ok(matches) => matches,
//         Err(_) => return MessageResponse::internal_server_error()
//     };
// }
