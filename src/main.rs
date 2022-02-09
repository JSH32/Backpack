use actix_http::Uri;
use config::StorageConfig;
use models::MessageResponse;
use tokio::fs;

use std::{
    panic,
    path::{Path, PathBuf},
};

use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    http::StatusCode,
    middleware::Logger,
    web::{self, Data},
    App, HttpRequest, HttpServer,
};

use actix_files::{Files, NamedFile};

use lettre::{transport::smtp::authentication::Credentials, AsyncSmtpTransport, Tokio1Executor};

use storage::{local::LocalProvider, s3::S3Provider, StorageProvider};

#[macro_use]
extern crate lazy_static;
extern crate argon2;
extern crate dotenv;
extern crate env_logger;

mod config;
mod database;
mod models;
mod routes;
mod state;
mod storage;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Setup actix log
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let config = config::Config::new();

    // Check if client directory provided has requirements to be served
    let client_path = match std::env::args().nth(1) {
        Some(v) => match config.serve_frontend {
            true => {
                let path = PathBuf::from(v);

                if !path.is_dir() {
                    panic!("Invalid client provided");
                }

                Some(path)
            }
            false => None,
        },
        None => None,
    };

    let sonyflake_worker = database::sonyflake::Sonyflake::new(config.worker_id, None)
        .expect("There was a problem creating the Sonyflake worker");

    let database = database::Database::new(16, &config.database_url, sonyflake_worker).await;
    if let Err(err) = database.run_migrations(Path::new("migrations")).await {
        panic!("{}", err);
    }

    let storage: Box<dyn StorageProvider> = match &config.storage_provider {
        StorageConfig::Local(v) => {
            if !v.path.exists() {
                fs::create_dir(&v.path).await.expect(&format!(
                    "Unable to create {} directory",
                    v.path.to_str().unwrap_or("storage")
                ));
            }

            Box::new(LocalProvider::new(v.path.clone()))
        }
        StorageConfig::S3(v) => Box::new(S3Provider::new(
            &v.bucket,
            &v.access_key,
            &v.secret_key,
            v.region.clone(),
        )),
    };

    let smtp_client = match config.smtp_config {
        Some(smtp_config) => {
            let creds = Credentials::new(smtp_config.username.clone(), smtp_config.password);

            Some((
                AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_config.server)
                    .unwrap()
                    .credentials(creds)
                    .build(),
                smtp_config.username,
            ))
        }
        None => None,
    };

    // Get setting as single boolean before client gets moved
    let smtp_enabled = smtp_client.is_some();

    let api_state = Data::new(state::State {
        database: database,
        storage: storage,
        jwt_key: config.jwt_key,
        smtp_client: smtp_client,
        base_url: config.base_url.parse::<Uri>().unwrap(),
        storage_url: config.storage_url,
        with_client: config.serve_frontend,
        // Convert MB to bytes
        file_size_limit: config.file_size_limit * 1000 * 1000,
    });

    let storage_path = match &config.storage_provider {
        StorageConfig::Local(v) => {
            if v.serve {
                Some(v.path.clone())
            } else {
                None
            }
        }
        _ => None,
    };

    HttpServer::new(move || {
        let mut app = App::new()
            .wrap(Logger::default())
            .app_data(api_state.clone())
            .service(
                web::scope("/api/")
                    .service(routes::user::get_routes(smtp_enabled))
                    .service(routes::auth::get_routes())
                    .service(routes::application::get_routes())
                    .service(routes::file::get_routes()),
            )
            // Error handler when json body deserialization failed
            .app_data(web::JsonConfig::default().error_handler(|_, _| {
                actix_web::Error::from(models::MessageResponse::bad_request())
            }));

        let base_storage_path = storage_path.clone();

        if client_path.is_some() {
            let mut index_path = client_path.as_ref().unwrap().clone();
            index_path.push("index.html");

            app = app.default_service(
                Files::new("", &client_path.as_ref().unwrap())
                    // Redirect every 404 to index for react
                    .default_handler(move |req: ServiceRequest| {
                        let (req, _) = req.into_parts();

                        let response = match &base_storage_path {
                            Some(v) => {
                                let mut file_path = v.clone();
                                file_path
                                    .push(req.path().trim_start_matches('/').replace("..", ""));
                                match NamedFile::open(&file_path) {
                                    Ok(v) => v.into_response(&req),
                                    Err(_) => NamedFile::open(&index_path)
                                        .expect("Index file not found")
                                        .into_response(&req),
                                }
                            }
                            None => NamedFile::open(&index_path)
                                .expect("Index file not found")
                                .into_response(&req),
                        };

                        async { Ok(ServiceResponse::new(req, response)) }
                    })
                    .index_file("index.html") // Set defailt index file
                    .show_files_listing(), // Show index file
            );
        } else {
            app = app.default_service(web::route().to(move |req: HttpRequest| {
                if let Some(v) = &base_storage_path {
                    let mut file_path = v.clone();

                    // Request path after the root
                    let path_end = req.path().trim_start_matches('/');

                    // Make sure request path isn't empty
                    // This would attempt to send the directory (and fail) otherwise
                    if !path_end.eq("") {
                        // Sanitize the path to prevent walking to another directory
                        file_path.push(path_end.replace("..", ""));
                        if let Ok(v) = NamedFile::open(&file_path) {
                            return v.into_response(&req);
                        }
                    }
                };

                MessageResponse::new(StatusCode::NOT_FOUND, "Resource was not found!")
                    .http_response()
            }))
        };

        app
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await
}
