use actix_web::{delete, get, http::StatusCode, post, web, Responder, Scope};
use sea_orm::{prelude::*, Condition};

use crate::{
    database::entity::applications,
    internal::auth::{auth_role, Auth},
    models::application::*,
    services::{
        application::ApplicationService, prelude::UserOwnedService, ToMessageResponse, ToResponse,
    },
};

pub fn get_routes() -> Scope {
    web::scope("/application")
        .service(info)
        .service(create)
        .service(delete)
        .service(token)
}

/// Get token by application ID
/// - Allow unverified users: `false`
/// - Application token allowed: `false`
#[utoipa::path(
        context_path = "/api/application",
        tag = "application",
        responses(
            (status = 200, body = TokenResponse),
            (status = 404, body = MessageResponse, description = "Application not found")
        ),
        params(
            ("application_id" = str, Path, description = "Application ID to get token for"),
        ),
        security(("apiKey" = [])),
    )]
#[get("/{application_id}/token")]
async fn token(
    service: web::Data<ApplicationService>,
    application_id: web::Path<String>,
    user: Auth<auth_role::User>,
) -> impl Responder {
    service
        .generate_token(&application_id, Some(&user))
        .await
        .to_response::<TokenResponse>(StatusCode::OK)
}

/// Get token info
/// - Allow unverified users: `false`
/// - Application token allowed: `false`
#[utoipa::path(
    context_path = "/api/application",
    tag = "application",
    responses(
        (status = 200, body = ApplicationData),
        (status = 401, body = MessageResponse)
    ),
    security(("apiKey" = [])),
)]
#[get("/{application_id}")]
async fn info(
    service: web::Data<ApplicationService>,
    user: Auth<auth_role::User>,
    application_id: web::Path<String>,
) -> impl Responder {
    service
        .by_condition_authorized(
            Condition::all().add(applications::Column::Id.eq(application_id.to_owned())),
            Some(&user),
            true,
        )
        .await
        .to_response::<ApplicationData>(StatusCode::OK)
}

/// Create an application
/// - Allow unverified users: `false`
/// - Application token allowed: `false`
#[utoipa::path(
    context_path = "/api/application",
    tag = "application",
    responses(
        (status = 200, body = ApplicationData),
        (status = 400, body = MessageResponse, description = "Token limit reached or invalid name"),
    ),
    request_body = ApplicationCreate,
    security(("apiKey" = [])),
)]
#[post("")]
async fn create(
    service: web::Data<ApplicationService>,
    user: Auth<auth_role::User>,
    form: web::Json<ApplicationCreate>,
) -> impl Responder {
    service
        .create_application(&user.id, &form.name)
        .await
        .to_response::<ApplicationData>(StatusCode::OK)
}

/// Delete an application
/// - Allow unverified users: `false`
/// - Application token allowed: `false`
#[utoipa::path(
    context_path = "/api/application",
    tag = "application",
    responses(
        (status = 200, body = MessageResponse, description = "Application was deleted"),
        (status = 401, body = MessageResponse, description = "Unauthorized or token does not exist"),
    ),
    security(("apiKey" = [])),
)]
#[delete("/{application_id}")]
async fn delete(
    service: web::Data<ApplicationService>,
    user: Auth<auth_role::User>,
    application_id: web::Path<String>,
) -> impl Responder {
    service
        .delete_authorized(application_id.to_string(), None, Some(&user))
        .await
        .to_message_response(StatusCode::OK)
}
