use std::ops::Deref;

use actix_multipart_extract::Multipart;
use actix_web::{delete, get, http::StatusCode, post, web, HttpResponse, Responder, Scope};

use crate::services::ToPageResponse;
use crate::{
    internal::auth::{auth_role, AllowApplication, Auth, DenyUnverified},
    models::{
        BatchDeleteRequest, BatchDeleteResponse, FileData, FileQuery, FileStats, UploadConflict,
        UploadFile,
    },
    services::{
        file::{FileService, UploadResult},
        ToMessageResponse, ToResponse,
    },
};

pub fn get_routes() -> Scope {
    web::scope("/file")
        .service(stats)
        .service(list)
        .service(info)
        .service(upload)
        .service(delete_files)
        .service(delete_file)
}

/// Upload a file
/// - Minimum required role: `user`
/// - Allow unverified users: `false`
/// - Application token allowed: `true`
#[utoipa::path(
    context_path = "/api/file",
    tag = "file",
    responses(
        (status = 200, body = FileData),
        (status = 409, body = MessageResponse, description = "File already uploaded"),
        (status = 413, body = MessageResponse, description = "File too large")
    ),
    security(("apiKey" = [])),
    request_body(content = UploadFile, content_type = "multipart/form-data")
)]
#[post("")]
async fn upload(
    service: web::Data<FileService>,
    user: Auth<auth_role::User, DenyUnverified, AllowApplication>,
    file: Multipart<UploadFile>,
) -> impl Responder {
    match service
        .upload_file(&user.id, &file.upload_file.name, &file.upload_file.bytes)
        .await
    {
        Ok(v) => match v {
            UploadResult::Success(file) => HttpResponse::Ok().json(file),
            UploadResult::Conflict(file) => HttpResponse::Conflict().json(UploadConflict {
                message: "File was already uploaded".into(),
                file,
            }),
        },
        Err(e) => e.to_response(),
    }
}

/// Get file stats for user
/// - Minimum required role: `user`
/// - Allow unverified users: `false`
/// - Application token allowed: `true`
#[utoipa::path(
    context_path = "/api/file",
    tag = "file",
    responses((status = 200, body = FileStats)),
    security(("apiKey" = [])),
)]
#[get("/stats")]
async fn stats(
    service: web::Data<FileService>,
    user: Auth<auth_role::User, DenyUnverified, AllowApplication>,
) -> impl Responder {
    service
        .user_stats(&user.id)
        .await
        .to_response::<FileStats>(StatusCode::OK)
}

/// Get a paginated list of files
/// - Minimum required role: `user`
/// - Allow unverified users: `false`
/// - Application token allowed: `true`
#[utoipa::path(
    context_path = "/api/file",
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
    user: Auth<auth_role::User, DenyUnverified, AllowApplication>,
    query: web::Query<FileQuery>,
) -> impl Responder {
    service
        .get_file_page(
            *page_number,
            25,
            Some(&user.id),
            query.query.as_ref().map(Deref::deref),
        )
        .await
        .to_page_response::<FileData>(StatusCode::OK)
}

/// Get file data by ID
/// - Minimum required role: `user`
/// - Allow unverified users: `false`
/// - Application token allowed: `true`
#[utoipa::path(
    context_path = "/api/file", 
    tag = "file",
    responses(
        (status = 200, body = FileData),
        (status = 403, body = MessageResponse, description = "Access denied"),
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
    user: Auth<auth_role::User, DenyUnverified, AllowApplication>,
) -> impl Responder {
    service
        .get_file(&file_id, Some(&user.id))
        .await
        .to_response::<FileData>(StatusCode::OK)
}

/// Delete file data by ID.
/// - Minimum required role: `user`
/// - Allow unverified users: `false`
/// - Application token allowed: `true`
#[utoipa::path(
    context_path = "/api/file",
    tag = "file",
    responses(
        (status = 200, body = MessageResponse, description = "File deleted"),
        (status = 403, body = MessageResponse, description = "Access denied"),
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
    user: Auth<auth_role::User, DenyUnverified, AllowApplication>,
) -> impl Responder {
    service
        .delete_file(&file_id, Some(&user.id))
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
    user: Auth<auth_role::User, DenyUnverified, AllowApplication>,
) -> impl Responder {
    service
        .delete_batch(&body.ids, Some(&user.id))
        .await
        .to_response::<BatchDeleteResponse>(StatusCode::OK)
}
