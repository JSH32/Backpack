use actix_web::{delete, get, http::StatusCode, patch, post, put, web, Responder, Scope};

use crate::{
    database::entity::sea_orm_active_enums::AuthMethod,
    internal::auth::{
        auth_role, AllowApplication, AllowUnregistered, AllowUnverified, Auth, DenyApplication,
    },
    models::{
        MessageResponse, RegistrationParams, UpdateUserSettings, UserCreateForm, UserData,
        UserDeleteForm,
    },
    services::{user::UserService, ToResponse},
};

pub mod album;
pub mod application;
pub mod upload;

pub fn get_routes() -> Scope {
    web::scope("/user")
        .service(
            web::scope("/{user_id}")
                .service(upload::get_routes())
                .service(application::get_routes())
                .service(album::get_routes())
                .service(settings)
                .service(info)
                .service(resend_verify)
                .service(register_key)
                .service(delete),
        )
        .service(verify)
        .service(create)
}

/// Get private user information. This is not the same thing as a user profile.
/// - Allow unverified users: `true`
/// - Application token allowed: `true`
#[utoipa::path(
    context_path = "/api/user/{user_id}",
    tag = "user",
    responses(
        (status = 200, body = UserData)
    ),
    params(("user_id" = str, Path)),
    security(("apiKey" = []))
)]
#[get("")]
async fn info(
    service: web::Data<UserService>,
    user_id: web::Path<String>,
    user: Auth<auth_role::User, AllowUnverified, AllowApplication, AllowUnregistered>,
) -> impl Responder {
    service
        .by_id_authorized(&user_id, Some(&user))
        .await
        .to_response::<UserData>(StatusCode::OK)
}

/// Change user settings
/// - Allow unverified users: `true`
/// - Application token allowed: `false`
#[utoipa::path(
    context_path = "/api/user/{user_id}",
    tag = "user",
    responses(
        (status = 200, body = UserData),
        (status = 400, body = MessageResponse),
        (status = 409, body = MessageResponse)
    ),
    security(("apiKey" = [])),
    params(("user_id" = str, Path)),
    request_body = UpdateUserSettings
)]
#[put("/settings")]
async fn settings(
    service: web::Data<UserService>,
    form: web::Json<UpdateUserSettings>,
    user_id: web::Path<String>,
    user: Auth<auth_role::User, AllowUnverified, DenyApplication, AllowUnregistered>,
) -> impl Responder {
    service
        .update_settings(
            &user_id,
            form.0.email,
            form.0.username,
            form.0.new_password,
            form.0.current_password,
            Some(&user),
        )
        .await
        .to_response::<UserData>(StatusCode::OK)
}

/// Register account using a registration key.
/// This is only required on services with `invite_only` enabled.
/// Admins can register a user without a key.
#[utoipa::path(
    context_path = "/api/user/{user_id}",
    tag = "user",
    responses(
        (status = 200, body = UserData),
        (status = 400, body = MessageResponse),
    ),
    params(
        RegistrationParams,
        ("user_id" = str, Path)
    ),
)]
#[get("/register")]
async fn register_key(
    service: web::Data<UserService>,
    user: Auth<auth_role::User, AllowUnverified, DenyApplication, AllowUnregistered>,
    user_id: web::Path<String>,
    params: web::Query<RegistrationParams>,
) -> impl Responder {
    service
        .register_user(&user_id, params.key.clone(), Some(&user))
        .await
        .to_response::<UserData>(StatusCode::OK)
}

/// Create a new user
#[utoipa::path(
    context_path = "/api/user",
    tag = "user",
    responses(
        (status = 200, body = UserData),
        (status = 400, body = MessageResponse),
        (status = 409, body = MessageResponse)
    ),
    request_body = UserCreateForm,
)]
#[post("")]
async fn create(
    service: web::Data<UserService>,
    form: web::Json<UserCreateForm>,
) -> impl Responder {
    match service
        .create_user(
            form.0.username,
            form.0.email,
            (AuthMethod::Password, form.0.password, None),
            form.0.registration_key,
        )
        .await
    {
        Ok(_) => MessageResponse::new(StatusCode::OK, "User has successfully been created")
            .http_response(),
        Err(e) => e.to_response(),
    }
}

/// Resend a verification code to the email
/// - Allow unverified users: `true`
/// - Application token allowed: `false`
///
/// This will be disabled if `smtp` is disabled in server settings
#[utoipa::path(
    context_path = "/api/user/{user_id}",
    tag = "user",
    responses(
        (status = 200, body = MessageResponse),
        (status = 409, body = MessageResponse, description = "Already verified"),
        (status = 410, body = MessageResponse, description = "SMTP is disabled")
    ),
    security(("apiKey" = [])),
    params(("user_id" = str, Path))
)]
#[patch("/verify/resend")]
async fn resend_verify(
    service: web::Data<UserService>,
    user_id: web::Path<String>,
    user: Auth<auth_role::User, AllowUnverified, DenyApplication, AllowUnregistered>,
) -> impl Responder {
    match service.resend_verification(&user_id, Some(&user)).await {
        Ok(v) => MessageResponse::new(
            StatusCode::OK,
            &format!("Verification email resent to {}", v),
        )
        .http_response(),
        Err(e) => e.to_response(),
    }
}

/// Verify using a verification code.
///
/// This will verify whatever user the code was created for.
/// This will be disabled if `smtp` is disabled in server settings
#[utoipa::path(
    context_path = "/api/user",
    tag = "user",
    responses(
        (status = 200, body = MessageResponse),
        (status = 400, body = MessageResponse, description = "Invalid verification code"),
        (status = 410, body = MessageResponse, description = "SMTP is disabled")
    ),
    params(
        ("code" = str, Path, description = "Verification code to verify"),
    )
)]
#[patch("/verify/{code}")]
async fn verify(service: web::Data<UserService>, code: web::Path<String>) -> impl Responder {
    match service.verify_by_code(&code).await {
        Ok(_) => MessageResponse::new(StatusCode::OK, "User has been verified").http_response(),
        Err(e) => e.to_response(),
    }
}

/// Delete a user and all files owned by the user
/// - Allow unverified users: `true`
/// - Application token allowed: `false`
#[utoipa::path(
    context_path = "/api/user/{user_id}",
    tag = "user",
    responses(
        (status = 200, body = MessageResponse, description = "User was deleted"),
        (status = 400, body = MessageResponse, description = "Incorrect password")
    ),
    request_body(content = UserDeleteForm, description = "Verify your password"),
    security(("apiKey" = [])),
    params(("user_id" = str, Path))
)]
#[delete("")]
async fn delete(
    service: web::Data<UserService>,
    user_id: web::Path<String>,
    user: Auth<auth_role::User, AllowUnverified, DenyApplication, AllowUnregistered>,
    form: web::Json<UserDeleteForm>,
) -> impl Responder {
    match service
        .delete(&user_id, form.password.clone(), Some(&user))
        .await
    {
        Ok(_) => MessageResponse::new(StatusCode::OK, "User has been deleted").http_response(),
        Err(e) => e.to_response(),
    }
}
