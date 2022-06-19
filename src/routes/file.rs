use std::{
    collections::HashMap,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use actix_web::{delete, get, http::StatusCode, post, web, HttpResponse, Responder, Scope};
use nanoid::nanoid;
use sea_orm::{
    sea_query::SimpleExpr, ActiveModelTrait, ColumnTrait, ConnectionTrait, DbBackend, EntityTrait,
    ModelTrait, PaginatorTrait, QueryFilter, QueryOrder, Set, Statement,
};
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::{
    database::entity::files,
    internal::{
        auth::{auth_role, Auth},
        file::{get_thumbnail_image, IMAGE_EXTS},
        multipart::Multipart,
        response::{self, Response},
        validate_paginate,
    },
    models::{FileData, FileStats, MessageResponse, Page, UploadFile},
    state::State,
};

pub fn get_routes() -> Scope {
    web::scope("/file")
        .service(stats)
        .service(list)
        .service(info)
        .service(upload)
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
    state: web::Data<State>,
    auth: Auth<auth_role::User, false, true>,
    file: Multipart<UploadFile>,
) -> Response<impl Responder> {
    if file.upload_file.bytes.len() > state.file_size_limit {
        return MessageResponse::ok(
            StatusCode::PAYLOAD_TOO_LARGE,
            &format!(
                "File was larger than the size limit of {}mb",
                state.file_size_limit / 1000 / 1000
            ),
        );
    }

    let extension = Path::new(&file.upload_file.name)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("");

    // New filename, collision not likely with NanoID
    let filename = nanoid!(10) + "." + extension;

    let hash = &format!("{:x}", Sha256::digest(&file.upload_file.bytes));

    let file_exists = files::Entity::find()
        .filter(files::Column::Hash.eq(hash.to_owned()))
        .one(&state.database)
        .await?;

    if let Some(file) = file_exists {
        // Push the existing file name for the matching hash
        let mut file_url = PathBuf::from(&state.storage_url);
        file_url.push(file.name);

        // let mut object = serde_json::Map::new();
        let mut object = HashMap::new();
        object.insert(
            "url".to_string(),
            json!(&file_url.as_path().display().to_string().replace("\\", "/")),
        );

        return MessageResponse::ok_with_data(
            StatusCode::CONFLICT,
            "You have already uploaded this file",
            object,
        );
    }

    let file_model = files::ActiveModel {
        uploader: Set(auth.user.id.to_owned()),
        name: Set(filename.to_owned()),
        original_name: Set(file.upload_file.name.to_owned()),
        hash: Set(hash.to_owned()),
        size: Set(file.upload_file.bytes.len() as i64),
        ..Default::default()
    }
    .insert(&state.database)
    .await?;

    // Upload file to storage provider
    // If this fails attempt to delete the file from database
    if let Err(err) = state
        .storage
        .put_object(&filename, &file.upload_file.bytes)
        .await
    {
        let _ = file_model.delete(&state.database).await;
        return Err(response::Error(err));
    }

    let root_path = PathBuf::from(&state.storage_url);

    let mut file_data = FileData::from(file_model);

    // Create thumbnail
    if IMAGE_EXTS
        .into_iter()
        .any(|ext| ext.eq(&extension.to_uppercase()))
    {
        // We don't care if this fails. Thumbnail can fail for whatever reason due to image encoding
        // User/API caller should not expect thumbnail to ALWAYS exist
        if let Ok(image) = &get_thumbnail_image(&file.upload_file.bytes) {
            // TODO: Store thumbnail status as a boolean in database just in case
            let _ = state
                .storage
                .put_object(&format!("./thumb/{}", &filename), image)
                .await;

            file_data.set_thumbnail_url(root_path.clone());
        }
    }

    file_data.set_url(root_path.clone());
    Ok(HttpResponse::Ok().json(file_data))
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
    state: web::Data<State>,
    auth: Auth<auth_role::User, false, true>,
) -> Response<impl Responder> {
    // Im not using an ORM for this query
    let usage = state
        .database
        .query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"SELECT COALESCE(CAST(SUM(size) AS BIGINT), 0) FROM files WHERE uploader = $1"#,
            vec![auth.user.id.into()],
        ))
        .await?;

    Ok(HttpResponse::Ok().json(FileStats {
        usage: match usage {
            Some(v) => v.try_get("", "coalesce")?,
            None => 0,
        },
    }))
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
        ("page_number" = u64, path, description = "Page to get files by (starts at 1)"),
        ("query" = Option<str>, query, description = "Query by file name similarity")
    ),
    security(("apiKey" = [])),
)]
#[get("/list/{page_number}")]
async fn list(
    state: web::Data<State>,
    page_number: web::Path<usize>,
    auth: Auth<auth_role::User, false, true>,
    query_params: web::Query<HashMap<String, String>>,
) -> Response<impl Responder> {
    let query = match query_params.get("query") {
        Some(str) => files::Column::Name.like(&format!("%{}%", str)),
        None => SimpleExpr::Custom("true".to_string()),
    };

    let paginator = files::Entity::find()
        .filter(files::Column::Uploader.eq(auth.user.id.to_owned()))
        .filter(query)
        .order_by_desc(files::Column::Uploaded)
        .paginate(&state.database, 25);

    let pages = paginator.num_pages().await?;
    if let Some(err) = validate_paginate(*page_number, pages) {
        return Ok(err.http_response());
    }

    let storage_url = PathBuf::from(state.storage_url.clone());

    Ok(HttpResponse::Ok().json(Page {
        page: *page_number,
        pages,
        list: paginator
            .fetch_page(*page_number - 1)
            .await?
            .iter()
            .map(|model| {
                let mut file_data = FileData::from(model.to_owned());
                file_data.set_url(storage_url.clone());
                file_data.set_thumbnail_url(storage_url.clone());
                file_data
            })
            .collect(),
    }))
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
        ("file_id" = u64, path, description = "File ID"),
    ),
    security(("apiKey" = [])),
)]
#[get("/{file_id}")]
async fn info(
    state: web::Data<State>,
    file_id: web::Path<String>,
    auth: Auth<auth_role::User, true, true>,
) -> Response<impl Responder> {
    Ok(
        match files::Entity::find_by_id(file_id.to_string())
            .one(&state.database)
            .await?
        {
            Some(v) => {
                if v.uploader != auth.user.id {
                    MessageResponse::new(
                        StatusCode::FORBIDDEN,
                        "You are not allowed to access this file",
                    )
                    .http_response()
                } else {
                    let storage_url = PathBuf::from(&state.storage_url);

                    let mut file = FileData::from(v);
                    file.set_url(storage_url.clone());
                    file.set_thumbnail_url(storage_url);

                    HttpResponse::Ok().json(file)
                }
            }
            None => MessageResponse::new(StatusCode::NOT_FOUND, "That file was not found")
                .http_response(),
        },
    )
}

/// Delete file data by ID
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
        ("file_id" = u64, path, description = "File ID"),
    ),
    security(("apiKey" = [])),
)]
#[delete("/{file_id}")]
async fn delete_file(
    state: web::Data<State>,
    file_id: web::Path<String>,
    auth: Auth<auth_role::User, true, true>,
) -> Response<impl Responder> {
    Ok(
        match files::Entity::find_by_id(file_id.to_string())
            .one(&state.database)
            .await?
        {
            Some(v) => {
                if v.uploader != auth.user.id {
                    MessageResponse::new(
                        StatusCode::FORBIDDEN,
                        "You are not allowed to access this file",
                    )
                } else {
                    v.clone().delete(&state.database).await?;

                    // We dont care about the result of this because of discrepancies
                    let _ = state.storage.delete_object(&v.name).await;
                    let _ = state
                        .storage
                        .delete_object(&format!("thumb/{}", &v.name))
                        .await;

                    MessageResponse::new(StatusCode::OK, &format!("File {} was deleted", v.name))
                }
            }
            None => MessageResponse::new(StatusCode::NOT_FOUND, "That file was not found"),
        },
    )
}
