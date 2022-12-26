use git_version::git_version;
use rand::Rng;

use crate::services::{ServiceError, ServiceResult};

pub mod auth;
pub mod file;
pub mod lateinit;

pub const GIT_VERSION: &str = git_version!();

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

/// Validate length of resource.
pub fn validate_length(
    resource: &str,
    min_length: usize,
    max_length: usize,
    string: &str,
) -> ServiceResult<()> {
    if string.len() > 16 {
        Err(ServiceError::InvalidData(
            format!("{resource} too long (maximum {max_length} characters)").into(),
        ))
    } else if string.len() < 4 {
        Err(ServiceError::InvalidData(
            format!("{resource} too short (minimum {min_length} characters)").into(),
        ))
    } else {
        Ok(())
    }
}
