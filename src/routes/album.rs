use actix_http::StatusCode;
use actix_web::{delete, get, patch, post, web, Responder, Scope};

use crate::{
    internal::auth::{auth_role, AllowApplication, Auth, AuthOptional, DenyUnverified},
    models::{
        album::{AlbumCreate, AlbumData, AlbumDelete, AlbumUpdate},
        MessageResponse,
    },
    services::{album::AlbumService, ToResponse},
};

pub fn get_routes() -> Scope {
    web::scope("/album")
        .service(info)
        .service(delete)
        .service(create)
        .service(update)
}

/// Get album info.
///
/// This wont work if you don't have access to the album and the album is privated.
///
/// **For private albums:**
/// - Allow unverified users: `false`
/// - Application token allowed: `true`
#[utoipa::path(
    context_path = "/api/album",
    tag = "album",
    responses(
        (status = 200, body = AlbumData),
        (status = 401, body = MessageResponse)
    ),
    security(("apiKey" = [])),
)]
#[get("/{album_id}")]
async fn info(
    service: web::Data<AlbumService>,
    album_id: web::Path<String>,
    user: AuthOptional<auth_role::User, DenyUnverified, AllowApplication>,
) -> impl Responder {
    service
        .get_album(&album_id, user.as_ref())
        .await
        .to_response::<AlbumData>(StatusCode::OK)
}

/// Delete an album.
///
/// - Allow unverified users: `false`
/// - Application token allowed: `false`
#[utoipa::path(
    context_path = "/api/album",
    tag = "album",
    responses(
        (status = 200, body = MessageResponse),
        (status = 401, body = MessageResponse)
    ),
    security(("apiKey" = [])),
)]
#[delete("/{album_id}")]
async fn delete(
    service: web::Data<AlbumService>,
    album_id: web::Path<String>,
    delete_album: web::Query<AlbumDelete>,
    user: Auth<auth_role::User, DenyUnverified, AllowApplication>,
) -> impl Responder {
    match service
        .delete(
            &album_id,
            match delete_album.delete_files {
                Some(v) => v,
                None => false,
            },
            Some(&user),
        )
        .await
    {
        Ok(v) => MessageResponse::new(
            StatusCode::OK,
            &format!("Successfully deleted album: {}", v.name),
        )
        .http_response(),
        Err(e) => e.to_response(),
    }
}

/// Create an album
/// - Allow unverified users: `false`
/// - Application token allowed: `true`
#[utoipa::path(
    context_path = "/api/album",
    tag = "album",
    responses((status = 200, body = AlbumData)),
    request_body = AlbumCreate,
    security(("apiKey" = [])),
)]
#[post("")]
async fn create(
    service: web::Data<AlbumService>,
    user: Auth<auth_role::User, DenyUnverified, AllowApplication>,
    form: web::Json<AlbumCreate>,
) -> impl Responder {
    service
        .create_album(&user.id, &form.name, form.description.clone(), form.public)
        .await
        .to_response::<AlbumData>(StatusCode::OK)
}

/// Update album settings
/// - Allow unverified users: `false`
/// - Application token allowed: `true`
#[utoipa::path(
    context_path = "/api/album",
    tag = "album",
    responses((status = 200, body = AlbumData)),
    request_body = AlbumUpdate,
    security(("apiKey" = [])),
)]
#[patch("/{album_id}")]
async fn update(
    service: web::Data<AlbumService>,
    user: Auth<auth_role::User, DenyUnverified, AllowApplication>,
    album_id: web::Path<String>,
    form: web::Json<AlbumUpdate>,
) -> impl Responder {
    service
        .update(
            &album_id,
            form.name.to_owned(),
            form.description.to_owned(),
            form.public,
            Some(&user),
        )
        .await
        .to_response::<AlbumData>(StatusCode::OK)
}
