use argon2::{self, Argon2, PasswordHash, PasswordVerifier};
use lettre::AsyncTransport;

use crate::{
    models::{MessageResponse, UpdateUserSettings, UserCreateForm},
    state::State,
    util::{
        self,
        auth::{auth_role, Auth},
        random_string,
        user::{new_password, validate_username, verification_email},
        EMAIL_REGEX,
    },
};

use actix_web::{get, http::StatusCode, patch, post, put, web, HttpResponse, Responder, Scope};

pub fn get_routes(smtp_verification: bool) -> Scope {
    let scope = web::scope("/user")
        .service(create)
        .service(settings)
        .service(info);

    if smtp_verification {
        scope.service(resend_verify).service(verify)
    } else {
        scope
    }
}

#[get("")]
async fn info(auth: Auth<auth_role::User, true, true>) -> impl Responder {
    HttpResponse::Ok().json(auth.user)
}

#[put("/settings")]
async fn settings(
    auth: Auth<auth_role::User, true, true>,
    state: web::Data<State>,
    form: web::Json<UpdateUserSettings>,
) -> Result<impl Responder, MessageResponse> {
    // Check if the users password is correct
    if Argon2::default()
        .verify_password(
            form.current_password.as_bytes(),
            &PasswordHash::new(&auth.user.password)
                .map_err(|_| MessageResponse::internal_server_error())?,
        )
        .is_ok()
    {
        return Err(MessageResponse::new(
            StatusCode::BAD_REQUEST,
            "Incorrect current password",
        ));
    }

    // We set the properties in here (except current_password) from the if blocks.
    // So that if we get one error afterwards it does not change partial data
    let mut to_change = UpdateUserSettings {
        current_password: "".to_string(), // Don't change this

        email: None,
        username: None,
        new_password: None,
    };

    if let Some(new_password) = &form.new_password {
        to_change.new_password = Some(util::user::new_password(&new_password)?);
    }

    if let Some(new_email) = &form.email {
        if !EMAIL_REGEX.is_match(&new_email) {
            return Err(MessageResponse::new(
                StatusCode::BAD_REQUEST,
                "Invalid email was provided",
            ));
        }

        if state.database.get_user_by_email(&new_email).await.is_ok() {
            return Err(MessageResponse::new(
                StatusCode::CONFLICT,
                "An account with that email already exists!",
            ));
        }

        to_change.email = Some(new_email.to_string());
    }

    if let Some(new_username) = &form.username {
        validate_username(&new_username)?;

        if state
            .database
            .get_user_by_username(&new_username)
            .await
            .is_ok()
        {
            return Err(MessageResponse::new(
                StatusCode::CONFLICT,
                "An account with that username already exists!",
            ));
        }

        to_change.username = Some(new_username.to_string());
    }

    // Update email if change validated
    if let Some(email) = to_change.email {
        state
            .database
            .change_email(&auth.user.id, &email)
            .await
            .map_err(|_| MessageResponse::internal_server_error())?;

        // If email validation is on we need to resend the email and unverify the user
        if let Some(smtp) = &state.smtp_client {
            state
                .database
                .verify_user(&auth.user.id, false)
                .await
                .map_err(|_| MessageResponse::internal_server_error())?;

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
                    &email,
                    &random_code,
                    state.with_client,
                );
                let mailer = smtp.clone().0;
                tokio::spawn(async move {
                    let _ = mailer.send(email).await;
                });
            }
        }
    }

    // Update password if change validated
    if let Some(new_password) = to_change.new_password {
        state
            .database
            .change_password(&auth.user.id, &util::user::new_password(&new_password)?)
            .await
            .map_err(|_| MessageResponse::internal_server_error())?;
    }

    // Update username if change validated
    if let Some(new_username) = to_change.username {
        state
            .database
            .change_username(&auth.user.id, &new_username)
            .await
            .map_err(|_| MessageResponse::internal_server_error())?;
    }

    // Send updated user data in case of data change
    Ok(HttpResponse::Ok().json(
        state
            .database
            .get_user_by_id(&auth.user.id)
            .await
            .map_err(|_| MessageResponse::internal_server_error())?,
    ))
}

#[post("")]
async fn create(
    state: web::Data<State>,
    mut form: web::Json<UserCreateForm>,
) -> Result<impl Responder, MessageResponse> {
    // Check if username length is within bounds
    validate_username(&form.username)?;

    if !EMAIL_REGEX.is_match(&form.email) {
        return Err(MessageResponse::new(
            StatusCode::BAD_REQUEST,
            "Invalid email was provided",
        ));
    }

    // Check if user with same email was found
    if state.database.get_user_by_email(&form.email).await.is_ok() {
        return Err(MessageResponse::new(
            StatusCode::CONFLICT,
            "An account with that email already exists!",
        ));
    }

    // Check if user with same username was found
    if state
        .database
        .get_user_by_username(&form.username)
        .await
        .is_ok()
    {
        return Err(MessageResponse::new(
            StatusCode::CONFLICT,
            "An account with that username already exists!",
        ));
    }

    form.password = match new_password(&form.password) {
        Ok(password_hashed) => password_hashed,
        Err(err) => return Err(err),
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
                        state.with_client,
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
        Err(_) => return Err(MessageResponse::internal_server_error()),
    }

    Ok(MessageResponse::new(
        StatusCode::OK,
        "User has successfully been created",
    ))
}

#[patch("/verify/resend")]
async fn resend_verify(
    state: web::Data<State>,
    auth: Auth<auth_role::User, true, false>,
) -> Result<impl Responder, MessageResponse> {
    if auth.user.verified {
        return Err(MessageResponse::new(
            StatusCode::CONFLICT,
            "You are already verified",
        ));
    }

    state
        .database
        .delete_verification(&auth.user.id)
        .await
        .map_err(|_| MessageResponse::internal_server_error())?;

    let random_code = random_string(72);
    state
        .database
        .create_verification(&auth.user.id, &random_code)
        .await
        .map_err(|_| MessageResponse::internal_server_error())?;

    let smtp = state.smtp_client.as_ref().unwrap();
    let email = verification_email(
        &state.base_url.to_string(),
        &smtp.1,
        &auth.user.email,
        &random_code,
        state.with_client,
    );

    let mailer = smtp.clone().0;
    tokio::spawn(async move {
        let _ = mailer.send(email).await;
    });

    Ok(MessageResponse::new(
        StatusCode::OK,
        &format!("Verification email resent to {}", auth.user.email),
    ))
}

#[patch("/verify/{code}")]
async fn verify(
    state: web::Data<State>,
    code: web::Path<String>,
) -> Result<impl Responder, MessageResponse> {
    match state.database.get_user_from_verification(&code).await {
        Ok(user_data) => {
            state
                .database
                .delete_verification(&user_data.id)
                .await
                .map_err(|_| MessageResponse::internal_server_error())?;

            // This case can ONLY happen if SMTP verification is disabled, the user tries to access their account, and THEN re-enables
            if user_data.verified {
                return Err(MessageResponse::new(
                    StatusCode::CONFLICT,
                    "User was already verified",
                ));
            }

            state
                .database
                .verify_user(&user_data.id, true)
                .await
                .map_err(|_| MessageResponse::internal_server_error())?;

            Ok(MessageResponse::new(
                StatusCode::OK,
                "User has been verified",
            ))
        }
        Err(_) => return Err(MessageResponse::bad_request()),
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
