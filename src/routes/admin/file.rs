use actix_http::StatusCode;
use actix_web::{delete, get, web, Responder, Scope};

use crate::{
    internal::auth::{auth_role, Auth},
    models::{admin::file::FileQuery, BatchDeleteRequest, BatchDeleteResponse, FileData},
    services::{file::FileService, ToMessageResponse, ToPageResponse, ToResponse},
};

pub fn get_routes() -> Scope {
    web::scope("/file")
        .service(list)
        .service(info)
        .service(delete_file)
        .service(delete_files)
}

/// Get a paginated list of files
/// - Minimum required role: `admin`
/// - Allow unverified users: `false`
/// - Application token allowed: `false`
#[utoipa::path(
    context_path = "/api/file/list",
    tag = "admin",
    responses(
        (status = 200, body = FilePage),
        (status = 400, body = MessageResponse, description = "Invalid page number"),
        (status = 404, body = MessageResponse, description = "Page not found")
    ),
    params(
        ("page_number" = usize, Path, description = "Page to get"),
        FileQuery
    ),
    security(("apiKey" = [])),
)]
#[get("/list/{page_number}")]
async fn list(
    service: web::Data<FileService>,
    page_number: web::Path<usize>,
    query: web::Query<FileQuery>,
    _user: Auth<auth_role::Admin>,
) -> impl Responder {
    service
        .get_file_page(*page_number, 25, query.0.user, query.0.search)
        .await
        .to_page_response::<FileData>(StatusCode::OK)
}

/// Get file data by ID
/// - Minimum required role: `admin`
/// - Allow unverified users: `false`
/// - Application token allowed: `false`
#[utoipa::path(
    context_path = "/api/file", 
    tag = "file",
    responses(
        (status = 200, body = FileData),
        (status = 404, body = MessageResponse, description = "File not found")
    ),
    params(
        ("file_id" = u64, Path, description = "File ID"),
    ),
    security(("apiKey" = [])),
)]
#[get("/{file_id}")]
async fn info(
    service: web::Data<FileService>,
    file_id: web::Path<String>,
    _user: Auth<auth_role::Admin>,
) -> impl Responder {
    service
        .get_file(&file_id, None)
        .await
        .to_response::<FileData>(StatusCode::OK)
}

/// Delete file data by ID.
/// - Minimum required role: `admin`
/// - Allow unverified users: `false`
/// - Application token allowed: `false`
#[utoipa::path(
    context_path = "/api/file",
    tag = "file",
    responses(
        (status = 200, body = MessageResponse, description = "File deleted"),
        (status = 404, body = MessageResponse, description = "File not found")
    ),
    params(
        ("file_id" = u64, Path, description = "File ID"),
    ),
    security(("apiKey" = [])),
)]
#[delete("/{file_id}")]
async fn delete_file(
    service: web::Data<FileService>,
    file_id: web::Path<String>,
    _user: Auth<auth_role::Admin>,
) -> impl Responder {
    service
        .delete_file(&file_id, None)
        .await
        .to_message_response(StatusCode::OK)
}

/// Delete multiple files by ID.
/// This will ignore any invalid IDs.
/// - Minimum required role: `user`
/// - Allow unverified users: `false`
/// - Application token allowed: `true`
#[utoipa::path(
    context_path = "/api/file",
    tag = "file",
    responses(
        (status = 200, body = BatchDeleteResponse, description = "Information about the batch operation result."),
    ),
    request_body(content = BatchDeleteRequest, description = "IDs to delete."),
    security(("apiKey" = [])),
)]
#[delete("/batch")]
async fn delete_files(
    service: web::Data<FileService>,
    body: web::Json<BatchDeleteRequest>,
    _user: Auth<auth_role::Admin>,
) -> impl Responder {
    service
        .delete_batch(&body.ids, None)
        .await
        .to_response::<BatchDeleteResponse>(StatusCode::OK)
}
