use actix_web::{HttpResponse, Responder, get, web};

use crate::state::State;

pub mod user;
pub mod auth;
pub mod application;