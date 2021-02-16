pub mod user;

use actix_web::{HttpResponse, http::StatusCode};
use serde_json::{Map, Value, json};

/// Create new success data response
pub fn new_data(code: StatusCode, data: Value) -> Value {
    let mut success = Map::new();
    success.insert("code".to_string(), json!(code.as_u16()));
    success.insert("error".to_string(), json!(false));
    success.insert("data".to_string(), data);
    serde_json::to_value(&success).unwrap()
}

/// Create new success message response
pub fn new_message(code: StatusCode, message: &str) -> Value {
    let mut success = Map::new();
    success.insert("code".to_string(), json!(code.as_u16()));
    success.insert("error".to_string(), json!(false));
    success.insert("message".to_string(), json!(message));
    serde_json::to_value(&success).unwrap()
}

/// Create new error message response
pub fn new_error(code: StatusCode, message: &str) -> Value {
    let mut error = Map::new();
    error.insert("code".to_string(), json!(code.as_u16()));
    error.insert("error".to_string(), json!(true));
    error.insert("message".to_string(), json!(message));
    serde_json::to_value(&error).unwrap()
}

/// Create new internal server error response
pub fn new_internal_server_error() -> HttpResponse {
    HttpResponse::InternalServerError()
        .json(new_error(StatusCode::INTERNAL_SERVER_ERROR, "There was a problem processing your request"))
}