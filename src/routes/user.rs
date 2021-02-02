use crate::api::{API};

#[get("/user/create")]
pub fn create(state: rocket::State<API>) -> &'static str {
    "Not implemented"
}