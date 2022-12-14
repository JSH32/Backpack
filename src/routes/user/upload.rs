use actix_http::StatusCode;
use actix_web::{get, web, Responder, Scope};

use crate::internal::auth::AuthOptional;
use crate::services::ToPageResponse;
use crate::{
    internal::auth::{auth_role, AllowApplication, Auth, DenyUnverified},
    models::{UploadData, UploadQuery, UploadStats},
    services::{upload::UploadService, ToResponse},
};

pub fn get_routes() -> Scope {
    web::scope("/upload").service(stats).service(list)
}

/// Get a paginated list of files
/// - Allow unverified users: `false`
/// - Application token allowed: `true`
#[utoipa::path(
	context_path = "/api/user/{user_id}/upload",
	tag = "upload",
	responses(
		(status = 200, body = UploadPage),
		(status = 400, body = MessageResponse, description = "Invalid page number"),
		(status = 404, body = MessageResponse, description = "Page not found")
	),
	params(
		("page_number" = u64, Path, description = "Page to get files by (starts at 1)"),
		("user_id" = str, Path),
		UploadQuery
	),
	security(("apiKey" = [])),
)]
#[get("/list/{page_number}")]
async fn list(
    service: web::Data<UploadService>,
    params: web::Path<(String, usize)>,
    user: AuthOptional<auth_role::User, DenyUnverified, AllowApplication>,
    query: web::Query<UploadQuery>,
) -> impl Responder {
    let (user_id, page_number) = params.to_owned();

    service
        .get_upload_page(
            page_number,
            25,
            Some(user_id.to_string()),
            query.query.to_owned(),
            query.album_id.to_owned(),
            query.public.to_owned(),
            user.user.as_ref(),
        )
        .await
        .to_page_response::<UploadData>(StatusCode::OK)
}

/// Get file stats for user
/// - Allow unverified users: `false`
/// - Application token allowed: `true`
#[utoipa::path(
	context_path = "/api/user/{user_id}/upload",
	tag = "upload",
	responses((status = 200, body = UploadStats)),
	security(("apiKey" = [])),
	params(("user_id" = str, Path))
)]
#[get("/stats")]
async fn stats(
    service: web::Data<UploadService>,
    user_id: web::Path<String>,
    user: Auth<auth_role::User, DenyUnverified, AllowApplication>,
) -> impl Responder {
    service
        .user_stats(user_id.as_str(), Some(&user))
        .await
        .to_response::<UploadStats>(StatusCode::OK)
}
