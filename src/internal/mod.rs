use actix_http::StatusCode;
use git_version::git_version;
use rand::Rng;
use regex::Regex;

use crate::models::MessageResponse;

pub mod auth;
pub mod file;
pub mod response;
pub mod user;

pub const GIT_VERSION: &str = git_version!();

lazy_static! {
    pub static ref EMAIL_REGEX: regex::Regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})"
    )
    .unwrap();
}

pub fn random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    return password;
}

/// Return proper error if page numbers don't match up
pub fn validate_paginate(page_number: usize, total_pages: usize) -> Option<MessageResponse> {
    if page_number < 1 {
        return Some(MessageResponse::new(
            StatusCode::BAD_REQUEST,
            "Pages start at 1",
        ));
    }

    if total_pages < page_number {
        return Some(MessageResponse::new(
            StatusCode::NOT_FOUND,
            &format!("There are only {} pages", total_pages),
        ));
    }

    None
}
