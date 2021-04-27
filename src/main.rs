use actix_web::*;
use config::StorageConfig;
use hmac::{Hmac, NewMac};
use rand::Rng;
use storage::{StorageProvider, local::LocalProvider, s3::S3Provider};

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
    let storage: Box<dyn StorageProvider> = match config.storage_provider {
        StorageConfig::Local(v) => Box::new(LocalProvider::new(v.path)),
        StorageConfig::S3(v) => Box::new(S3Provider::new(&v.bucket, &v.access_key, &v.secret_key, v.region))
    };
    
    let api_state = web::Data::new(state::State {
        database: database,
        storage: storage,
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