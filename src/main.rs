use actix_web::*;
use http::StatusCode;
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

    let api_state = web::Data::new(state::State {
        database: database,
        storage: storage
    });

    HttpServer::new(move || {
        App::new() 
            .app_data(api_state.clone())
            .service(
                web::scope("/api/v1/")
                    .service(routes::user::get_routes())
            )
            // Error handler when json body deserialization failed
            .app_data(web::JsonConfig::default().error_handler(|_, _| {
                HttpResponse::BadRequest()
                    .json(models::new_error(StatusCode::BAD_REQUEST, "Invalid data provided!"))
                    .into()
            }))
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await
}