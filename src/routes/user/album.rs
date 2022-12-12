use actix_http::StatusCode;
use actix_web::{get, web, Responder, Scope};

use crate::{
    internal::auth::{auth_role, AuthOptional},
    models::album::AlbumData,
    services::{album::AlbumService, ToPageResponse},
};

pub fn get_routes() -> Scope {
    web::scope("/album").service(list)
}

/// Get all albums owned by a user.
/// - Allow unverified users: `true`
/// - Application token allowed: `true`
#[utoipa::path(
	context_path = "/api/user/{user_id}/album",
	tag = "album",
	responses((status = 200, body = ApplicationPage)),
	params(
		("page_number" = u64, Path, description = "Page to get albums by (starts at 1)"),
		("user_id" = str, Path)
	),
	security(("apiKey" = [])),
)]
#[get("/{page_number}")]
async fn list(
    service: web::Data<AlbumService>,
    params: web::Path<(String, usize)>,
    user: AuthOptional<auth_role::User>,
) -> impl Responder {
    let (user_id, page_number) = params.to_owned();

    service
        .get_album_page(page_number, 10, &user_id, user.as_ref())
        .await
        .to_page_response::<AlbumData>(StatusCode::OK)
}
