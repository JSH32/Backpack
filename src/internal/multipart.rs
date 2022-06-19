use actix_http::Payload;
use actix_web::{Error, FromRequest, HttpRequest};
use futures::{Future, StreamExt, TryStreamExt};
use serde::Deserialize;
use serde_aux::prelude::serde_introspect;
use serde_json::{Map, Number, Value};
use std::{ops::Deref, pin::Pin};

/// Representing a file in a multipart form.
/// This must be used with [`Multipart`]
#[derive(Debug, Deserialize)]
pub struct File {
    pub content_type: String,
    pub name: String,
    pub bytes: Vec<u8>,
}

/// Extractor to extract multipart forms from the request
pub struct Multipart<T> {
    data: T,
}

impl<T> Multipart<T> {
    fn new(data: T) -> Self {
        Multipart::<T> { data }
    }
}

impl<T> Deref for Multipart<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data
    }
}

impl<T: serde::de::DeserializeOwned> FromRequest for Multipart<T> {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let mut multipart = actix_multipart::Multipart::new(req.headers(), payload.take());
        let req_owned = req.to_owned();

        Box::pin(async move {
            match serde_json::from_value::<T>(
                multipart_to_json(serde_introspect::<T>(), &mut multipart).await,
            ) {
                Ok(parsed) => Ok(Multipart::<T>::new(parsed)),
                Err(err) => Err(match req_owned.app_data::<MultipartConfig>() {
                    Some(config) => match &config.error_handler {
                        Some(error_handler) => error_handler(err),
                        None => actix_web::error::ErrorBadRequest(""),
                    },
                    None => actix_web::error::ErrorBadRequest(""),
                }),
            }
        })
    }
}

async fn multipart_to_json(
    valid_fields: &[&str],
    multipart: &mut actix_multipart::Multipart,
) -> Value {
    let mut map = Map::new();

    while let Ok(Some(mut field)) = multipart.try_next().await {
        let disposition = field.content_disposition().clone();

        let field_name = match disposition.get_name() {
            Some(v) => v,
            None => continue,
        };

        let field_name_formatted = field_name.replace("[]", "");

        // Make sure the field actually exists on the form
        if !valid_fields.contains(&field_name) {
            continue;
        }

        if field.content_disposition().get_filename().is_some() {
            // Is a file
            let mut data: Vec<Value> = Vec::new();

            while let Some(chunk) = field.next().await {
                match chunk {
                    Ok(bytes) => {
                        data.reserve_exact(bytes.len());
                        for byte in bytes {
                            data.push(Value::Number(Number::from(byte)));
                        }
                    }
                    Err(_) => {
                        map.insert(field_name_formatted.to_owned(), Value::Null);
                        continue;
                    }
                }
            }

            let mut field_map = Map::new();
            field_map.insert(
                "content_type".to_owned(),
                Value::String(field.content_type().to_string()),
            );

            field_map.insert(
                "name".to_owned(),
                Value::String(
                    field
                        .content_disposition()
                        .get_filename()
                        .unwrap()
                        .to_string(),
                ),
            );

            field_map.insert("bytes".to_owned(), Value::Array(data));

            params_insert(
                &mut map,
                field_name,
                &field_name_formatted,
                Value::Object(field_map),
            );
        } else if let Some(Ok(value)) = field.next().await {
            // Not a file, parse as other JSON types
            if let Ok(str) = std::str::from_utf8(&value) {
                // Attempt to convert into a number
                match str.parse::<isize>() {
                    Ok(number) => params_insert(
                        &mut map,
                        field_name,
                        &field_name_formatted,
                        Value::Number(Number::from(number)),
                    ),
                    Err(_) => match str {
                        "true" => params_insert(
                            &mut map,
                            field_name,
                            &field_name_formatted,
                            Value::Bool(true),
                        ),
                        "false" => params_insert(
                            &mut map,
                            field_name,
                            &field_name_formatted,
                            Value::Bool(false),
                        ),
                        _ => params_insert(
                            &mut map,
                            field_name,
                            &field_name_formatted,
                            Value::String(str.to_owned()),
                        ),
                    },
                }
            }
        } else {
            // Nothing
            params_insert(&mut map, field_name, &field_name_formatted, Value::Null)
        }
    }

    Value::Object(map)
}

fn params_insert(
    params: &mut Map<String, Value>,
    field_name: &str,
    field_name_formatted: &String,
    element: Value,
) {
    if field_name.ends_with("[]") {
        if params.contains_key(field_name_formatted) {
            if let Value::Array(val) = params.get_mut(field_name_formatted).unwrap() {
                val.push(element);
            }
        } else {
            params.insert(field_name_formatted.to_owned(), Value::Array(vec![element]));
        }
    } else {
        params.insert(field_name.to_owned(), element);
    }
}

type MultipartErrorHandler =
    Box<dyn Fn(serde_json::Error) -> actix_web::Error + Send + Sync + 'static>;

/// Config for Multipart data, insert as AppData to actix
pub struct MultipartConfig {
    pub error_handler: Option<MultipartErrorHandler>,
}

impl MultipartConfig {
    pub fn set_error_handler<F>(mut self, error_handler: F) -> Self
    where
        F: Fn(serde_json::Error) -> actix_web::Error + Send + Sync + 'static,
    {
        self.error_handler = Some(Box::new(error_handler));
        self
    }
}

impl Default for MultipartConfig {
    fn default() -> Self {
        Self {
            error_handler: None,
        }
    }
}
