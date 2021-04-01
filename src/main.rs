use actix_web::*;
use hmac::{Hmac, NewMac};
use rand::{Rng, rngs::OsRng, distributions::Alphanumeric};
use storage::{StorageProvider, s3::S3Provider};

extern crate dotenv;
extern crate argon2;

mod database;
mod models;
mod config;
mod state;
mod routes;
mod storage;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::new();

    let database = database::Database::new(16, &config.database_url).await;
    let storage = S3Provider::new(&config.s3_bucket, &config.s3_access_key, &config.s3_secret_key, config.s3_region);

    let api_state = web::Data::new(state::State {
        database: database,
        storage: Box::new(storage),
        jwt_key: Hmac::new_varkey(&rand::thread_rng().gen::<[u8; 32]>()).expect("Could not generate JWT key")
    });

    HttpServer::new(move || {
        App::new() 
            .app_data(api_state.clone())
            .service(
                web::scope("/api/v1/")
                    .service(routes::user::get_routes())
                    .service(routes::auth::get_routes())
            )
            // Error handler when json body deserialization failed
            .app_data(web::JsonConfig::default().error_handler(|_, _| {
                Error::from(models::MessageResponse::bad_request())
            }))
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await
}