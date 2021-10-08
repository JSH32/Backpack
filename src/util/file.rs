use actix_multipart::Multipart;
use thiserror::Error;

use futures::{
    AsyncWriteExt, 
    TryStreamExt
};

#[derive(Error, Debug)]
pub enum MultipartError {
    #[error("field `{0}` was not found")]
    FieldNotFound(String),
    #[error("payload was larger than `{0}`")]
    PayloadTooLarge(usize),
    #[error("there was a problem writing from the payload")]
    WriteError
}

pub struct File {
    pub filename: String,
    pub bytes: Vec<u8>,
    pub size: usize
}

pub async fn get_file_from_payload(payload: &mut Multipart, size_limit: usize, field_name: &str) -> Result<File, MultipartError> {
    while let Ok(Some(mut field)) = payload.try_next().await {
        if let Some(disposition) = field.content_disposition() {
            let filename_param = match disposition.get_filename() {
                Some(v) => v,
                None => continue
            };

            let name_param = match disposition.get_name() {
                Some(v) => v,
                None => continue
            };

            if name_param != field_name {
                continue;
            }

            let mut bytes = Vec::<u8>::new();
            let mut size = 0;

            while let Ok(Some(chunk)) = field.try_next().await {
                size += chunk.len();

                if size > size_limit {
                    return Err(MultipartError::PayloadTooLarge(size_limit))
                }

                if let Err(_) = bytes.write(&chunk).await {
                    return Err(MultipartError::WriteError)
                }
            }

            return Ok(File {
                filename: filename_param.to_string(),
                bytes: bytes,
                size: size
            });
        }
    };

    Err(MultipartError::FieldNotFound(field_name.to_string()))
}