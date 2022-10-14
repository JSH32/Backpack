use git_version::git_version;
use rand::Rng;

use crate::database::entity::users;

pub mod auth;
pub mod file;

pub const GIT_VERSION: &str = git_version!();

/// Get user ID being accessed as a string.
pub fn user_id(user_id: &str, accessing_useer: &users::Model) -> String {
    if user_id == "@me" {
        accessing_useer.id
    } else {
        user_id.into()
    }
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
