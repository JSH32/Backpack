use std::{collections::hash_map::DefaultHasher, ffi::OsStr, hash::{Hash, Hasher}, path::{Path, PathBuf}};

use actix_multipart::Multipart;
use nanoid::nanoid;
use actix_web::{
    HttpResponse, 
    Responder, 
    Scope, 
    http::StatusCode, 
    get, 
    put, 
    web
};

use crate::{
    models::MessageResponse, 
    state::State,
    util::{
        auth::{
            Auth, 
            auth_role
        }, 
        file::{
            MultipartError,
            get_file_from_payload
        }
    }
};

pub fn get_routes() -> Scope {
    web::scope("/file/")
        .service(info)
        .service(upload)
}

#[put("/upload")]
async fn upload(state: web::Data<State>, auth: Auth<auth_role::User, false, true>, mut payload: Multipart) -> Result<impl Responder, MessageResponse> {
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

            let mut file_data = state.database.create_file(
                &auth.user.id,
                &filename, 
                &hasher.finish().to_string(),
                file.size as i32,
                chrono::offset::Utc::now()
            ).await.map_err(|_| MessageResponse::internal_server_error())?;

            // Upload file to storage provider
            // If this fails attempt to delete the file from database
            if let Err(_) = state.storage.put_object(&filename, &file.bytes).await {
                let _ = state.database.delete_file(&file_data.id).await;
                return Err(MessageResponse::internal_server_error());
            }

            let mut file_url = PathBuf::from(&state.storage_url);
            file_url.push(&filename);
            file_data.url = Some(file_url.as_path().display().to_string());

            Ok(HttpResponse::Ok()
                .json(file_data))
        },
        Err(err) => Err(match err {
            MultipartError::FieldNotFound(_) => MessageResponse::bad_request(),
            MultipartError::PayloadTooLarge(_) =>
                MessageResponse::new(StatusCode::PAYLOAD_TOO_LARGE,
                    &format!("File was larger than the size limit of {}mb", state.file_size_limit/1000/1000)),
            MultipartError::WriteError => MessageResponse::internal_server_error(),
        })
    }
}

#[get("/info/{file_id}")]
async fn info(state: web::Data<State>, file_id: web::Path<String>, auth: Auth<auth_role::User, true, true>) -> impl Responder {
    match state.database.get_file(&file_id).await {
        Ok(mut v) => {
            if v.uploader != auth.user.id {
                MessageResponse::new(StatusCode::FORBIDDEN, "You are not allowed to access this file").http_response()
            } else {
                let mut file_url = PathBuf::from(&state.storage_url);
                file_url.push(&v.name);
                v.url = Some(file_url.as_path().display().to_string());
                HttpResponse::Ok().json(v)
            }
        },
        Err(_) => {
            MessageResponse::new(StatusCode::NOT_FOUND, "That file was not found").http_response()
        }
    }
}