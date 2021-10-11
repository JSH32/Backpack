use std::{
    collections::hash_map::DefaultHasher,
    ffi::OsStr,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
};

use actix_multipart::Multipart;
use actix_web::{delete, get, http::StatusCode, post, web, HttpResponse, Responder, Scope};
use nanoid::nanoid;
use serde_json::json;

use crate::{
    models::{file::FilePage, FileData, MessageResponse},
    state::State,
    util::{
        auth::{auth_role, Auth},
        file::{get_file_from_payload, MultipartError},
    },
};

pub fn get_routes() -> Scope {
    web::scope("/file/")
        .service(info)
        .service(upload)
        .service(list)
        .service(delete_file)
}

#[post("/upload")]
async fn upload(
    state: web::Data<State>,
    auth: Auth<auth_role::User, false, true>,
    mut payload: Multipart,
) -> Result<impl Responder, MessageResponse> {
    match get_file_from_payload(&mut payload, state.file_size_limit, "uploadFile").await {
        Ok(file) => {
            let extension = Path::new(&file.filename)
                .extension()
                .and_then(OsStr::to_str)
                .unwrap_or("");

            // New filename, collision not likely with NanoID
            let filename = nanoid!(10) + "." + extension;

            let mut hasher = DefaultHasher::new();
            file.bytes.hash(&mut hasher);
            let hash = &hasher.finish().to_string();

            let file_exists = state
                .database
                .exist_file_hash(&auth.user.id, hash)
                .await
                .map_err(|_| MessageResponse::internal_server_error())?;

            let mut file_url = PathBuf::from(&state.storage_url);

            if let Some(name) = file_exists {
                // Push the existing file name for the matching hash
                file_url.push(name);

                let mut object = serde_json::Map::new();
                object.insert(
                    "url".to_string(),
                    json!(&file_url.as_path().display().to_string()),
                );

                return Err(MessageResponse::new_with_data(
                    StatusCode::CONFLICT,
                    "You have already uploaded this file",
                    serde_json::Value::Object(object),
                ));
            }

            let mut file_data = state
                .database
                .create_file(
                    &auth.user.id,
                    &filename,
                    hash,
                    file.size as i32,
                    chrono::offset::Utc::now(),
                )
                .await
                .map_err(|_| MessageResponse::internal_server_error())?;

            // Upload file to storage provider
            // If this fails attempt to delete the file from database
            if let Err(_) = state.storage.put_object(&filename, &file.bytes).await {
                let _ = state.database.delete_file(&file_data.id).await;
                return Err(MessageResponse::internal_server_error());
            }

            file_url.push(&filename);
            file_data.url = Some(file_url.as_path().display().to_string());

            Ok(HttpResponse::Ok().json(file_data))
        }
        Err(err) => Err(match err {
            MultipartError::FieldNotFound(_) => MessageResponse::bad_request(),
            MultipartError::PayloadTooLarge(_) => MessageResponse::new(
                StatusCode::PAYLOAD_TOO_LARGE,
                &format!(
                    "File was larger than the size limit of {}mb",
                    state.file_size_limit / 1000 / 1000
                ),
            ),
            MultipartError::WriteError => MessageResponse::internal_server_error(),
        }),
    }
}

#[get("/list/{page_number}")]
async fn list(
    state: web::Data<State>,
    page_number: web::Path<u32>,
    auth: Auth<auth_role::User, false, true>,
) -> Result<impl Responder, MessageResponse> {
    const PAGE_SIZE: u32 = 25;

    if *page_number < 1 {
        return Err(MessageResponse::new(
            StatusCode::BAD_REQUEST,
            "Pages start at 1",
        ));
    }

    match state
        .database
        .get_total_file_pages(&auth.user.id, PAGE_SIZE)
        .await
    {
        Ok(total_pages) => {
            let storage_url = PathBuf::from(&state.storage_url);

            let file_list: Vec<FileData> = state
                .database
                .get_files(&auth.user.id, PAGE_SIZE, *page_number)
                .await
                .map_err(|_| MessageResponse::internal_server_error())?
                .into_iter()
                // Attach the URL to each file
                .map(|mut file| {
                    let mut file_url = storage_url.clone();
                    file_url.push(&file.name);
                    file.url = Some(file_url.as_path().display().to_string());
                    file
                })
                .collect();

            if file_list.len() < 1 {
                return Err(MessageResponse::new(
                    StatusCode::NOT_FOUND,
                    &format!("There are only {} pages", total_pages),
                ));
            }

            Ok(HttpResponse::Ok().json(FilePage {
                page: *page_number,
                pages: total_pages,
                files: file_list,
            }))
        }
        Err(_) => Err(MessageResponse::internal_server_error()),
    }
}

#[get("/info/{file_id}")]
async fn info(
    state: web::Data<State>,
    file_id: web::Path<String>,
    auth: Auth<auth_role::User, true, true>,
) -> impl Responder {
    match state.database.get_file(&file_id).await {
        Ok(mut v) => {
            if v.uploader != auth.user.id {
                MessageResponse::new(
                    StatusCode::FORBIDDEN,
                    "You are not allowed to access this file",
                )
                .http_response()
            } else {
                let mut file_url = PathBuf::from(&state.storage_url);
                file_url.push(&v.name);
                v.url = Some(file_url.as_path().display().to_string());
                HttpResponse::Ok().json(v)
            }
        }
        Err(_) => {
            MessageResponse::new(StatusCode::NOT_FOUND, "That file was not found").http_response()
        }
    }
}

#[delete("/delete/{file_id}")]
async fn delete_file(
    state: web::Data<State>,
    file_id: web::Path<String>,
    auth: Auth<auth_role::User, true, true>,
) -> Result<impl Responder, MessageResponse> {
    match state.database.get_file(&file_id).await {
        Ok(v) => {
            if v.uploader != auth.user.id {
                Err(MessageResponse::new(
                    StatusCode::FORBIDDEN,
                    "You are not allowed to access this file",
                ))
            } else {
                state
                    .database
                    .delete_file(&file_id)
                    .await
                    .map_err(|_| MessageResponse::internal_server_error())?;

                // We dont care about the result of this because of discrepancies
                let _ = state.storage.delete_object(&v.name).await;

                Ok(MessageResponse::new(
                    StatusCode::OK,
                    &format!("File {} was deleted", v.name),
                ))
            }
        }
        Err(_) => Err(MessageResponse::new(
            StatusCode::NOT_FOUND,
            "That file was not found",
        )),
    }
}
