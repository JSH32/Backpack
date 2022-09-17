use actix_http::StatusCode;
use actix_web::{delete, get, post, web, Responder, Scope};

use crate::{
    internal::auth::{auth_role, Auth},
    models::admin::registration_key::{RegistrationKeyData, RegistrationKeyParams},
    services::{prelude::*, registration_key::RegistrationKeyService},
};

pub fn get_routes() -> Scope {
    web::scope("/registrationKey")
        .service(get_one)
        .service(list)
        .service(delete)
        .service(create)
}

/// Create a registration key
/// - Minimum required role: `admin`
/// - Allow unverified users: `false`
/// - Application token allowed: `false`
#[utoipa::path(
    context_path = "/api/admin/registrationKey",
    tag = "admin",
    responses((status = 200, body = RegistrationKeyData)),
    security(("apiKey" = [])),
    params(RegistrationKeyParams)
)]
#[post("")]
async fn create(
    service: web::Data<RegistrationKeyService>,
    user: Auth<auth_role::Admin>,
    query: web::Query<RegistrationKeyParams>,
) -> impl Responder {
    service
        .create_registration_key(&user.id, query.uses, query.expiration)
        .await
        .to_response::<RegistrationKeyData>(StatusCode::OK)
}

/// Get registration keys
/// - Minimum required role: `admin`
/// - Allow unverified users: `false`
/// - Application token allowed: `false`
#[utoipa::path(
    context_path = "/api/admin/registrationKey",
    tag = "admin",
    responses((status = 200, body = RegistrationKeyPage)),
    params(
        ("page_number" = usize, Path, description = "Page to get")
    ),
    security(("apiKey" = [])),
)]
#[get("/list/{page_number}")]
async fn list(
    service: web::Data<RegistrationKeyService>,
    page_number: web::Path<usize>,
    _user: Auth<auth_role::Admin>,
) -> impl Responder {
    service
        .get_page(*page_number, 25, None)
        .await
        .to_page_response::<RegistrationKeyData>(StatusCode::OK)
}

/// Get a single registration key
/// - Minimum required role: `admin`
/// - Allow unverified users: `false`
/// - Application token allowed: `false`
#[utoipa::path(
    context_path = "/api/admin/registrationKey",
    tag = "admin",
    responses(
        (status = 200, body = RegistrationKeyData),
        (status = 404, body = MessageResponse, description = "Registration key was not found"),
    ),
    params(
        ("registration_id" = usize, Path, description = "Registration key to get")
    ),
    security(("apiKey" = [])),
)]
#[get("/{registration_id}")]
async fn get_one(
    service: web::Data<RegistrationKeyService>,
    registration_id: web::Path<String>,
    _user: Auth<auth_role::Admin>,
) -> impl Responder {
    service
        .by_id(registration_id.to_string())
        .await
        .to_response::<RegistrationKeyData>(StatusCode::OK)
}

/// Delete a registration key
/// - Minimum required role: `admin`
/// - Allow unverified users: `false`
/// - Application token allowed: `false`
#[utoipa::path(
    context_path = "/api/admin/registrationKey",
    tag = "admin",
    responses(
        (status = 200, body = MessageResponse, description = "Registration key was deleted"),
        (status = 404, body = MessageResponse, description = "Registration key was not found"),
    ),
    params(
        ("registration_id" = usize, Path, description = "Registration key to delete")
    ),
    security(("apiKey" = [])),
)]
#[delete("/{registration_id}")]
async fn delete(
    service: web::Data<RegistrationKeyService>,
    registration_id: web::Path<String>,
    _user: Auth<auth_role::Admin>,
) -> impl Responder {
    service
        .delete(registration_id.to_string(), true, None)
        .await
        .to_message_response(StatusCode::OK)
}
