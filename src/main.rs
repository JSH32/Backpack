use crate::database::entity::files;
use actix_http::Uri;
use clap::Parser;
use colored::*;
use config::StorageConfig;
use figlet_rs::FIGfont;
use indicatif::{ProgressBar, ProgressStyle};
use models::MessageResponse;
use sea_orm::{ConnectOptions, Database, EntityTrait};
use sqlx::{migrate::Migrator, postgres::PgPoolOptions};
use state::State;
use tokio::fs;

use util::file::IMAGE_EXTS;

use std::{
    convert::TryInto,
    ffi::OsStr,
    panic,
    path::{Path, PathBuf},
    time::Duration,
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

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Client directory location to serve from `/` path
    #[clap(short, long)]
    client: Option<String>,

    /// Regenerate image thumbnails
    #[clap(short, long, takes_value = false)]
    generate_thumbnails: bool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Setup actix log
    std::env::set_var("RUST_LOG", "actix_web=info,backpack=info,sqlx=error");
    env_logger::init();

    let fig_font = FIGfont::from_content(include_str!("./resources/small.flf")).unwrap();
    let figure = fig_font.convert("Backpack").unwrap();
    println!("{}", figure.to_string().purple());

    let config = config::Config::new();
    let args = Args::parse();

    // Check if client directory provided has requirements to be served
    let client_path = match args.client {
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

    // Create a SQLx pool for running migrations
    let migrator_pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&config.database_url)
        .await
        .expect("Could not initialize migrator connection");

    let migrator = Migrator::new(Path::new("migrations")).await.unwrap();
    migrator.run(&migrator_pool).await.unwrap();
    migrator_pool.close().await;

    let mut opt = ConnectOptions::new(config.database_url.clone());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);

    let database = Database::connect(opt).await.unwrap();

    log::info!("Connected to the database");

    let storage: Box<dyn StorageProvider> = match &config.storage_provider {
        StorageConfig::Local(v) => {
            if !v.path.exists() {
                fs::create_dir(&v.path).await.expect(&format!(
                    "Unable to create {} directory",
                    v.path.to_str().unwrap_or("storage")
                ));
            }

            // Thumbnail directory
            let mut thumb_path = v.path.clone();
            thumb_path.push("thumb");

            if !thumb_path.exists() {
                fs::create_dir(&thumb_path)
                    .await
                    .expect("Unable to create thumbnail directory");
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

    // If the generate thumbnails flag is enabled
    if args.generate_thumbnails {
        generate_thumbnails(&api_state).await.unwrap();
        return Ok(());
    }

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

    log::info!(
        "Starting webserver on port {}",
        config.port.to_string().yellow()
    );

    HttpServer::new(move || {
        let mut app = App::new()
            .wrap(Logger::default())
            .app_data(api_state.clone())
            .service(
                web::scope("/api/")
                    .service(routes::user::get_routes(smtp_enabled))
                    .service(routes::auth::get_routes())
                    .service(routes::application::get_routes())
                    .service(routes::file::get_routes())
                    .service(routes::get_routes())
                    .service(routes::registration_key::get_routes()),
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

async fn generate_thumbnails(state: &Data<State>) -> anyhow::Result<()> {
    log::info!("Regenerating image thumbnails");

    let files = files::Entity::find().all(&state.database).await?;

    let image_files: Vec<files::Model> = files
        .iter()
        .filter(|file| {
            let extension = Path::new(&file.name)
                .extension()
                .and_then(OsStr::to_str)
                .unwrap_or("");

            IMAGE_EXTS
                .into_iter()
                .any(|ext| ext.eq(&extension.to_uppercase()))
        })
        .map(|v| v.clone())
        .collect();

    log::info!(
        "{} files to generate",
        image_files.len().to_string().yellow()
    );

    let progress = ProgressBar::new(image_files.len().try_into().unwrap());
    progress.set_style(
        ProgressStyle::default_bar()
            .template(&format!(
                "{}{{elapsed_precise}}{} {{bar:40.cyan/blue}} {{pos:>2}}/{{len:2}} {{msg}}",
                "[".bright_black(),
                "]".bright_black()
            ))
            .progress_chars("##-"),
    );

    for file in image_files {
        let extension = Path::new(&file.name)
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or("");

        if IMAGE_EXTS
            .into_iter()
            .any(|ext| ext.eq(&extension.to_uppercase()))
        {
            progress.set_message(file.name.clone());
            progress.inc(1);

            match state.storage.get_object(&file.name).await {
                Ok(buf) => {
                    // log::info!("Generating thumbnail for {}", file.yellow());
                    if let Err(err) = state
                        .storage
                        .put_object(
                            &format!("thumb/{}", file.name),
                            &util::file::get_thumbnail_image(&buf)?,
                        )
                        .await
                    {
                        log::error!("Error putting {}: {}", file.name, err)
                    }
                }
                Err(err) => log::error!("Error getting {}: {}", file.name, err),
            }
        }
    }

    progress.finish_with_message("Finished generating thumbnails");

    Ok(())
}
