#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate dotenv;

mod database;
mod models;
mod config;
mod api;
mod routes;

#[async_std::main]
async fn main() {
    let config = config::Config::new();
    let database = database::Database::new(16, &config.database_url).await;

    let api_state = api::API {
        database: database
    };

    rocket::ignite()
        .manage(api_state)
        .mount(
            "/api/v1",
            routes![
                routes::user::create
            ],
        )
        .launch();
}