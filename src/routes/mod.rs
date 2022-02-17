use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use thiserror::Error;

use crate::models::MessageResponse;

pub mod application;
pub mod auth;
pub mod file;
pub mod user;
