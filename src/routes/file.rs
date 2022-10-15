use actix_multipart_extract::Multipart;
use actix_web::{delete, get, http::StatusCode, post, web, HttpResponse, Responder, Scope};

use crate::{
    internal::auth::{auth_role, AllowApplication, Auth, DenyUnverified},
    models::{BatchDeleteRequest, BatchDeleteResponse, FileData, UploadConflict, UploadFile},
    services::{
        file::{FileService, UploadResult},
        ToMessageResponse, ToResponse,
    },
};

pub fn get_routes() -> Scope {
    web::scope("/file")
        .service(info)
        .service(upload)
        .service(delete_files)
        .service(delete_file)
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
        .delete_file(&file_id, Some(&user))
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
        .delete_batch(&body.ids, Some(&user))
        .await
        .to_response::<BatchDeleteResponse>(StatusCode::OK)
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
        .get_file(&file_id, Some(&user))
        .await
        .to_response::<FileData>(StatusCode::OK)
}

/// Upload a file.
/// You can only upload a file for yourself regardless of admin status.
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
