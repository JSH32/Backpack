use actix_multipart_extract::Multipart;
use actix_web::{delete, get, http::StatusCode, post, web, HttpResponse, Responder, Scope};

use crate::{
    internal::auth::{auth_role, AllowApplication, Auth, AuthOptional, DenyUnverified},
    models::{BatchDeleteRequest, BatchDeleteResponse, UploadConflict, UploadData, UploadFile},
    services::{
        upload::{UploadResult, UploadService},
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
    service: web::Data<UploadService>,
    file_id: web::Path<String>,
    user: Auth<auth_role::User, DenyUnverified, AllowApplication>,
) -> impl Responder {
    service
        .delete_file(&file_id, Some(&user))
        .await
        .to_message_response(StatusCode::OK)
}

/// Delete multiple uploads by ID.
/// This will ignore any invalid IDs.
/// - Allow unverified users: `false`
/// - Application token allowed: `true`
#[utoipa::path(
    context_path = "/api/upload",
    tag = "upload",
    responses(
        (status = 200, body = BatchDeleteResponse, description = "Information about the batch operation result."),
    ),
    request_body(content = BatchDeleteRequest, description = "IDs to delete."),
    security(("apiKey" = [])),
)]
#[delete("/batch")]
async fn delete_files(
    service: web::Data<UploadService>,
    body: web::Json<BatchDeleteRequest>,
    user: Auth<auth_role::User, DenyUnverified, AllowApplication>,
) -> impl Responder {
    service
        .delete_batch(&body.ids, Some(&user))
        .await
        .to_response::<BatchDeleteResponse>(StatusCode::OK)
}

/// Get file data by ID
/// - Allow unverified users: `false`
/// - Application token allowed: `true`
#[utoipa::path(
    context_path = "/api/upload", 
    tag = "upload",
    responses(
        (status = 200, body = FileData),
        (status = 403, body = MessageResponse, description = "Access denied"),
        (status = 404, body = MessageResponse, description = "File not found")
    ),
    params(
        ("upload_id" = u64, Path, description = "Upload ID"),
    ),
    security(("apiKey" = [])),
)]
#[get("/{upload_id}")]
async fn info(
    service: web::Data<UploadService>,
    upload_id: web::Path<String>,
    user: AuthOptional<auth_role::User, DenyUnverified, AllowApplication>,
) -> impl Responder {
    service
        .get_file(&upload_id, user.as_ref())
        .await
        .to_response::<UploadData>(StatusCode::OK)
}

/// Upload a file.
/// You can only upload a file for yourself regardless of admin status.
/// - Allow unverified users: `false`
/// - Application token allowed: `true`
#[utoipa::path(
    context_path = "/api/upload",
    tag = "upload",
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
    service: web::Data<UploadService>,
    user: Auth<auth_role::User, DenyUnverified, AllowApplication>,
    file: Multipart<UploadFile>,
) -> impl Responder {
    match service
        .upload_file(&user.id, &file.upload_file.name, &file.upload_file.bytes)
        .await
    {
        Ok(v) => match v {
            UploadResult::Success(upload) => HttpResponse::Ok().json(upload),
            UploadResult::Conflict(upload) => HttpResponse::Conflict().json(UploadConflict {
                message: "File was already uploaded".into(),
                upload,
            }),
        },
        Err(e) => e.to_response(),
    }
}
