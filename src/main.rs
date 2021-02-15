use std::sync::{Arc, Mutex};

use actix_web::*;
use rusoto_s3::{S3, S3Client};
use storage::Storage;

extern crate dotenv;
extern crate argon2;

mod database;
mod models;
mod config;
mod state;
mod routes;
mod storage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::new();

    let database = database::Database::new(16, &config.database_url).await;
    let storage = Storage::new(&config.s3_bucket, &config.s3_access_key, &config.s3_secret_key, config.s3_region);

    let api_state = Arc::new(state::State {
        database: database,
        storage: storage
    });

    HttpServer::new(move || {
        App::new()
            .data(api_state.clone())
            .service(
                web::scope("/api/v1/")
                    .service(routes::user::get_routes())
            )
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await
}