use std::panic;
use std::path::Path;
use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::{*, middleware::Logger};
use actix_files::Files;
use config::StorageConfig;
use lettre::AsyncSmtpTransport;
use lettre::Tokio1Executor;
use lettre::transport::smtp::authentication::Credentials;
use storage::{StorageProvider, local::LocalProvider, s3::S3Provider};
use tokio::fs;

extern crate env_logger;
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
            },
            false => None,
        },
        None => None,
    };

    let database = database::Database::new(16, &config.database_url).await;
    if let Err(err) = database.run_migrations(Path::new("migrations")).await {
        panic!("{}", err.to_string());
    }

    let storage: Box<dyn StorageProvider> = match config.storage_provider {
        StorageConfig::Local(v) => {
            if !v.path.exists() {
                fs::create_dir(&v.path).await.expect(&format!("Unable to create {} directory", 
                    v.path.to_str().unwrap_or("storage")));
            }

            Box::new(LocalProvider::new(v.path))
        },
        StorageConfig::S3(v) => Box::new(S3Provider::new(&v.bucket, &v.access_key, &v.secret_key, v.region))
    };

    let smtp_client = match config.smtp_config {
        Some(smtp_config) => {
            let creds = Credentials::new(smtp_config.username.clone(), smtp_config.password);

            Some((AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_config.server)
                .unwrap()
                .credentials(creds)
                .build(), smtp_config.username))
        },
        None => None
    };

    // Get setting as single boolean before client gets moved
    let smtp_enabled = smtp_client.is_some();

    let api_state = web::Data::new(state::State {
        database: database,
        storage: storage,
        jwt_key: config.jwt_key,
        smtp_client: smtp_client,
        base_url: config.base_url
    });

    HttpServer::new(move || {
        let mut app = App::new() 
            .wrap(Logger::default())
            .app_data(api_state.clone())
            .service(
                web::scope("/api/")
                    .service(routes::user::get_routes(smtp_enabled))
                    .service(routes::auth::get_routes())
                    .service(routes::application::get_routes())
            )
            // Error handler when json body deserialization failed
            .app_data(web::JsonConfig::default().error_handler(|_, _| Error::from(models::MessageResponse::bad_request())));


        if client_path.is_some() {
            let mut index_path = client_path.as_ref().unwrap().clone();
            index_path.push("index.html");

            app = app.default_service(
                Files::new("", &client_path.as_ref().unwrap())
                    // Redirect every 404 to index for react
                    .default_handler(move |req: ServiceRequest| {
                        let (req, _) = req.into_parts();

                        let response = NamedFile::open(&index_path)
                            .expect("Index file not found")
                            .into_response(&req);

                        async {
                            Ok(ServiceResponse::new(req, response))
                        }
                    })
                    .index_file("index.html") // Set defailt index file
                    .show_files_listing() // Show index file
            )
        }
        
        app
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await
}