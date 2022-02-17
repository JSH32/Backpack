use std::{
    collections::HashMap,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use actix_multipart::Multipart;
use actix_web::{delete, get, http::StatusCode, post, web, HttpResponse, Responder, Scope};
use nanoid::nanoid;
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::{
    models::{file::FilePage, Error, FileData, FileStats, MessageResponse, Response},
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

            let file_exists = state.database.exist_file_hash(&auth.user.id, &hash).await?;

            if let Some(name) = file_exists {
                // Push the existing file name for the matching hash
                let mut file_url = PathBuf::from(&state.storage_url);
                file_url.push(name);

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

            let mut file_data = state
                .database
                .create_file(
                    &auth.user.id,
                    &filename,
                    &file.filename,
                    &hash,
                    file.size as i32,
                    chrono::offset::Utc::now(),
                )
                .await?;

            // Upload file to storage provider
            // If this fails attempt to delete the file from database
            if let Err(_) = state.storage.put_object(&filename, &file.bytes).await {
                let _ = state.database.delete_file(&file_data.id).await;
                return Ok(MessageResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Unable to upload file",
                )
                .http_response());
            }

            let root_path = PathBuf::from(&state.storage_url);

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

                    file_data.set_thumbnail_url(root_path.clone());
                }
            }

            file_data.set_url(root_path);

            Ok(HttpResponse::Ok().json(file_data))
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
    Ok(HttpResponse::Ok().json(FileStats {
        usage: state.database.get_user_usage(&auth.user.id).await?,
    }))
}

#[get("/list/{page_number}")]
async fn list(
    state: web::Data<State>,
    page_number: web::Path<u32>,
    auth: Auth<auth_role::User, false, true>,
    query_params: web::Query<HashMap<String, String>>,
) -> Response<impl Responder> {
    let query = match query_params.get("query") {
        Some(str) => Some(str.clone()),
        None => None,
    };

    const PAGE_SIZE: u32 = 25;

    if *page_number < 1 {
        return Ok(
            MessageResponse::new(StatusCode::BAD_REQUEST, "Pages start at 1").http_response(),
        );
    }

    let total_pages = state
        .database
        .get_total_file_pages(&auth.user.id, PAGE_SIZE, &query)
        .await?;

    let file_list: Vec<FileData> = state
        .database
        .get_files(&auth.user.id, PAGE_SIZE, *page_number, &query)
        .await?
        .into_iter()
        // Attach the URL to each file
        .map(|mut file| {
            let storage_url = PathBuf::from(&state.storage_url);
            file.set_url(storage_url.clone());
            file.set_thumbnail_url(storage_url);

            file
        })
        .collect();

    if file_list.len() < 1 {
        return Ok(MessageResponse::new(
            StatusCode::NOT_FOUND,
            &format!("There are only {} pages", total_pages),
        )
        .http_response());
    }

    Ok(HttpResponse::Ok().json(FilePage {
        page: *page_number,
        pages: total_pages,
        files: file_list,
    }))
}

#[get("/{file_id}")]
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
                let storage_url = PathBuf::from(&state.storage_url);

                v.set_url(storage_url.clone());
                v.set_thumbnail_url(storage_url);

                HttpResponse::Ok().json(v)
            }
        }
        Err(_) => {
            MessageResponse::new(StatusCode::NOT_FOUND, "That file was not found").http_response()
        }
    }
}

#[delete("/{file_id}")]
async fn delete_file(
    state: web::Data<State>,
    file_id: web::Path<String>,
    auth: Auth<auth_role::User, true, true>,
) -> Response<impl Responder> {
    match state.database.get_file(&file_id).await {
        Ok(v) => {
            if v.uploader != auth.user.id {
                Ok(MessageResponse::new(
                    StatusCode::FORBIDDEN,
                    "You are not allowed to access this file",
                ))
            } else {
                state.database.delete_file(&file_id).await?;

                // We dont care about the result of this because of discrepancies
                let _ = state.storage.delete_object(&v.name).await;
                let _ = state
                    .storage
                    .delete_object(&format!("thumb/{}", &v.name))
                    .await;

                Ok(MessageResponse::new(
                    StatusCode::OK,
                    &format!("File {} was deleted", v.name),
                ))
            }
        }
        Err(_) => Ok(MessageResponse::new(
            StatusCode::NOT_FOUND,
            "That file was not found",
        )),
    }
}
