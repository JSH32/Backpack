use std::{
    collections::HashMap,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use actix_multipart::Multipart;
use actix_web::{delete, get, http::StatusCode, post, web, HttpResponse, Responder, Scope};
use nanoid::nanoid;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbBackend, EntityTrait,
    ModelTrait, Order, QueryFilter, Set, Statement,
};
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::util::use_paginate;
use crate::{
    database::{
        entity::files,
    },
    models::{Error, FileData, FileStats, MessageResponse, Response},
    state::State,
    util::{
        auth::{auth_role, Auth},
        file::{get_file_from_payload, get_thumbnail_image, MultipartError, IMAGE_EXTS},
    },
};

pub fn get_routes() -> Scope {
    web::scope("/file")
        .service(stats)
        .service(list)
        .service(info)
        .service(upload)
        .service(delete_file)
}

#[post("")]
async fn upload(
    state: web::Data<State>,
    auth: Auth<auth_role::User, false, true>,
    mut payload: Multipart,
) -> Response<impl Responder> {
    match get_file_from_payload(&mut payload, state.file_size_limit, "uploadFile").await {
        Ok(file) => {
            let extension = Path::new(&file.filename)
                .extension()
                .and_then(OsStr::to_str)
                .unwrap_or("");

            // New filename, collision not likely with NanoID
            let filename = nanoid!(10) + "." + extension;

            let hash = &format!("{:x}", Sha256::digest(&file.bytes));

            let file_exists = files::Entity::find()
                .filter(files::Column::Hash.eq(hash.to_owned()))
                .one(&state.database)
                .await?;

            if let Some(file) = file_exists {
                // Push the existing file name for the matching hash
                let mut file_url = PathBuf::from(&state.storage_url);
                file_url.push(file.name);

                let mut object = serde_json::Map::new();
                object.insert(
                    "url".to_string(),
                    json!(&file_url.as_path().display().to_string().replace("\\", "/")),
                );

                return Ok(MessageResponse::new_with_data(
                    StatusCode::CONFLICT,
                    "You have already uploaded this file",
                    serde_json::Value::Object(object),
                )
                .http_response());
            }

            let file_model = files::ActiveModel {
                uploader: Set(auth.user.id.to_owned()),
                name: Set(filename.to_owned()),
                original_name: Set(file.filename.to_owned()),
                hash: Set(hash.to_owned()),
                size: Set(file.size as i64),
                ..Default::default()
            }
            .insert(&state.database)
            .await?;

            // Upload file to storage provider
            // If this fails attempt to delete the file from database
            if let Err(_) = state.storage.put_object(&filename, &file.bytes).await {
                let _ = file_model.delete(&state.database).await;
                return Ok(MessageResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Unable to upload file",
                )
                .http_response());
            }

            let root_path = PathBuf::from(&state.storage_url);

            let mut file_api = FileData::from(file_model);

            // Create thumbnail
            if IMAGE_EXTS
                .into_iter()
                .any(|ext| ext.eq(&extension.to_uppercase()))
            {
                // We don't care if this fails. Thumbnail can fail for whatever reason due to image encoding
                // User/API caller should not expect thumbnail to ALWAYS exist
                if let Ok(image) = &get_thumbnail_image(&file.bytes) {
                    let _ = state
                        .storage
                        .put_object(&format!("./thumb/{}", &filename), image)
                        .await;

                    file_api.set_thumbnail_url(root_path.clone());
                }
            }

            file_api.set_url(root_path);

            Ok(HttpResponse::Ok().json(file_api))
        }
        Err(err) => match err {
            MultipartError::FieldNotFound(_) => Ok(MessageResponse::bad_request().http_response()),
            MultipartError::PayloadTooLarge(_) => Ok(MessageResponse::new(
                StatusCode::PAYLOAD_TOO_LARGE,
                &format!(
                    "File was larger than the size limit of {}mb",
                    state.file_size_limit / 1000 / 1000
                ),
            )
            .http_response()),
            MultipartError::WriteError(err) => Err(Error::from(err)),
        },
    }
}

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

#[get("/list/{page_number}")]
async fn list(
    state: web::Data<State>,
    page_number: web::Path<u64>,
    auth: Auth<auth_role::User, false, true>,
    query_params: web::Query<HashMap<String, String>>,
) -> Response<impl Responder> {
    let query = match query_params.get("query") {
        Some(str) => Some((files::Column::Uploaded, str.clone())),
        None => None,
    };

    let storage_url = PathBuf::from(state.storage_url.clone());

    Ok(use_paginate(
        files::Entity,
        &state.database,
        25,
        *page_number,
        Some(files::Column::Uploader.eq(auth.user.id.clone())),
        Some((files::Column::Uploaded, Order::Desc)),
        query,
        move |f: &files::Model| {
            let mut file_data = FileData::from(f.to_owned());

            file_data.set_url(storage_url.clone());
            file_data.set_thumbnail_url(storage_url.clone());

            json!(file_data)
        },
    )
    .await?)
}

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
