use actix_http::StatusCode;
use actix_web::{get, web, Responder, Scope};

use crate::internal;
use crate::services::ToPageResponse;
use crate::{
    internal::auth::{auth_role, AllowApplication, Auth, DenyUnverified},
    models::{FileData, FileQuery, FileStats},
    services::{file::FileService, ToResponse},
};

pub fn get_routes() -> Scope {
    web::scope("/file").service(stats).service(list)
}

/// Get a paginated list of files
/// - Minimum required role: `user`
/// - Allow unverified users: `false`
/// - Application token allowed: `true`
#[utoipa::path(
	context_path = "/api/user/{user_id}/file",
	tag = "file",
	responses(
		(status = 200, body = FilePage),
		(status = 400, body = MessageResponse, description = "Invalid page number"),
		(status = 404, body = MessageResponse, description = "Page not found")
	),
	params(
		("page_number" = u64, Path, description = "Page to get files by (starts at 1)"),
		FileQuery
	),
	security(("apiKey" = [])),
)]
#[get("/list/{page_number}")]
async fn list(
    service: web::Data<FileService>,
    page_number: web::Path<usize>,
    user_id: web::Path<String>,
    user: Auth<auth_role::User, DenyUnverified, AllowApplication>,
    query: web::Query<FileQuery>,
) -> impl Responder {
    service
        .get_file_page(
            *page_number,
            25,
            Some(internal::user_id(&user_id, &user)),
            query.query.to_owned(),
            query.album_id.to_owned(),
            query.public.to_owned(),
            Some(&user),
        )
        .await
        .to_page_response::<FileData>(StatusCode::OK)
}

/// Get file stats for user
/// - Minimum required role: `user`
/// - Allow unverified users: `false`
/// - Application token allowed: `true`
#[utoipa::path(
	context_path = "/api/user/{user_id}/file",
	tag = "file",
	responses((status = 200, body = FileStats)),
	security(("apiKey" = [])),
)]
#[get("/stats")]
async fn stats(
    service: web::Data<FileService>,
    user_id: web::Path<String>,
    user: Auth<auth_role::User, DenyUnverified, AllowApplication>,
) -> impl Responder {
    service
        .user_stats(&internal::user_id(&user_id, &user), Some(&user))
        .await
        .to_response::<FileStats>(StatusCode::OK)
}
