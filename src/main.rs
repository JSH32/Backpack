use crate::{
    database::entity::files,
    docs::ApiDoc,
    internal::GIT_VERSION,
    services::{
        application::ApplicationService, auth::AuthService, file::FileService,
        registration_key::RegistrationKeyService, user::UserService,
    },
};
use actix_http::Uri;
use actix_multipart_extract::MultipartConfig;
use clap::Parser;
use colored::*;
use config::StorageConfig;
use figlet_rs::FIGfont;
use indicatif::{ProgressBar, ProgressStyle};
use models::MessageResponse;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, EntityTrait, Statement};
use std::sync::{Arc, RwLock};
use tokio::{
    runtime::Builder,
    sync::mpsc::{unbounded_channel, UnboundedSender},
};

use internal::file::IMAGE_EXTS;
use utoipa::OpenApi;

use migration::{Migrator, MigratorTrait};
use std::{convert::TryInto, ffi::OsStr, path::Path};

use actix_web::{
    http::StatusCode,
    middleware::Logger,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer,
};

use actix_files::NamedFile;

#[macro_use]
extern crate lazy_static;

extern crate argon2;
extern crate dotenv;
extern crate env_logger;

mod config;
mod database;
mod docs;
mod internal;
mod models;
mod routes;
mod services;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Regenerate image thumbnails
    #[clap(short, long, takes_value = false)]
    generate_thumbnails: bool,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Setup actix log
    std::env::set_var("RUST_LOG", "actix_web=info,backpack=info,sqlx=error");
    env_logger::init();

    let fig_font = FIGfont::from_content(include_str!("./resources/small.flf")).unwrap();
    let figure = fig_font.convert("Backpack").unwrap();
    println!("{}", figure.to_string().purple());
    println!(
        "Running Backpack on version: {}",
        GIT_VERSION.to_string().yellow()
    );

    let config = config::Config::new();
    let args = Args::parse();

    let database = Data::new(
        sea_orm::Database::connect(&config.database_url)
            .await
            .unwrap(),
    );

    log::info!(
        "Connected to the database ({})",
        get_db_version(&database).await.unwrap()
    );

    // Apply all pending migrations
    if config.run_migrations {
        Migrator::up(&database, None).await.unwrap();
    }

    // Get setting as single boolean before client gets moved
    let invite_only = config.invite_only;

    // Registration key service.
    let registration_key_service =
        Data::new(RegistrationKeyService::new(database.clone().into_inner()));

    // File service.
    let file_service = Data::new(
        FileService::new(
            database.clone().into_inner(),
            config.storage_provider.clone(),
            &config.storage_url,
            config.file_size_limit,
        )
        .await,
    );

    // User service.
    let user_service = Data::new(UserService::new(
        database.clone().into_inner(),
        registration_key_service.clone().into_inner(),
        file_service.clone().into_inner(),
        config.smtp_config,
        &config.client_url,
        config.invite_only,
    ));

    let application_service_container = Arc::new(RwLock::new(None));

    // Auth service.
    let auth_service = Data::new(AuthService::new(
        user_service.clone().into_inner(),
        application_service_container.clone(),
        config.api_url.parse::<Uri>().unwrap(),
        config.jwt_key,
    ));

    // Application service.
    let application_service = Data::new(ApplicationService::new(
        database.clone().into_inner(),
        auth_service.clone().into_inner(),
    ));

    application_service_container
        .write()
        .unwrap()
        .replace(application_service.clone().into_inner());

    // If the generate thumbnails flag is enabled
    if args.generate_thumbnails {
        generate_thumbnails(&database, &file_service).await.unwrap();
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
        let base_storage_path = storage_path.clone();
        App::new()
            .wrap(Logger::default())
            .app_data(database.clone())
            .app_data(registration_key_service.clone())
            .app_data(user_service.clone())
            .app_data(file_service.clone())
            .app_data(auth_service.clone())
            .app_data(application_service.clone())
            .route(
                "/api/docs/openapi.json",
                web::get().to(|| async { ApiDoc::openapi().to_pretty_json() }),
            )
            .route(
                "/api/docs",
                web::get()
                    .to(|| async { HttpResponse::Ok().body(include_str!("docs/rapidoc.html")) }),
            )
            .service(
                web::scope("/api")
                    .service(routes::user::get_routes())
                    .service(routes::auth::get_routes())
                    .service(routes::application::get_routes())
                    .service(routes::file::get_routes())
                    .service(routes::admin::get_routes(invite_only))
                    .service(routes::get_routes()),
            )
            // Error handler when json body deserialization failed
            .app_data(web::JsonConfig::default().error_handler(|_, _| {
                actix_web::Error::from(models::MessageResponse::bad_request())
            }))
            .app_data(
                MultipartConfig::default()
                    .set_error_handler(|_| models::MessageResponse::bad_request().http_response()),
            )
            .default_service(web::to(move |req: HttpRequest| {
                let storage_path = base_storage_path.clone();
                async move {
                    if let Some(v) = &storage_path {
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
                    }

                    MessageResponse::new(StatusCode::NOT_FOUND, "Resource was not found!")
                        .http_response()
                }
            }))
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await
}

/// Get database version.
async fn get_db_version(database: &DatabaseConnection) -> Result<String, anyhow::Error> {
    let version: String = database
        .query_one(Statement::from_string(
            database.get_database_backend(),
            format!(
                "select {}() as version;",
                match database.get_database_backend() {
                    DbBackend::Sqlite => "sqlite_version",
                    _ => "version",
                }
            )
            .to_string(),
        ))
        .await?
        .unwrap()
        .try_get("", "version")?;

    // SQLite version function is just a version number.
    Ok(match database.get_database_backend() {
        DbBackend::Sqlite => format!("SQLite {}", version),
        _ => version,
    })
}

/// TODO: Move this to admin panel with a websocket.
/// Regenerate all thumbnails.
/// This is a multithreaded blocking operation used in the CLI.
async fn generate_thumbnails(
    database: &Arc<DatabaseConnection>,
    file_service: &Arc<FileService>,
) -> anyhow::Result<()> {
    log::info!("Regenerating image thumbnails");

    let files = files::Entity::find().all(database.as_ref()).await?;

    // Get every file which is an image or has an image extension.
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

    // Make a seperate runtime for spawning blocking operations so that hundreds of threads aren't created.
    let runtime = Builder::new_multi_thread()
        .worker_threads(1) // We only need blocking threads
        .max_blocking_threads(num_cpus::get() / 2)
        .thread_name("backpack-thumbnail-generator")
        .build()
        .unwrap();

    log::info!(
        "{} files to generate with {} threads",
        image_files.len().to_string().yellow(),
        (num_cpus::get() / 2).to_string().yellow()
    );

    let progress = ProgressBar::new(image_files.len().try_into().unwrap());
    progress.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>2}/{len:2} {msg}",
            )
            .unwrap()
            .progress_chars("##-"),
    );

    let (tx, mut rx) = unbounded_channel();

    for file in image_files {
        let extension = Path::new(&file.name)
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or("");

        if IMAGE_EXTS
            .into_iter()
            .any(|ext| ext.eq(&extension.to_uppercase()))
        {
            let file_service = file_service.clone();
            let task_tx: UnboundedSender<Result<String, (String, String)>> = tx.clone();
            let spawner = runtime.handle().clone();
            tokio::spawn(async move {
                match file_service.storage.get_object(&file.name).await {
                    Ok(buf) => {
                        // Open a new task on the custom tokio runtime.
                        let resized = spawner
                            .spawn_blocking(move || internal::file::get_thumbnail_image(&buf))
                            .await
                            .unwrap();

                        match resized {
                            Ok(v) => {
                                // Write thumbnail object to storage.
                                if let Err(err) = file_service
                                    .storage
                                    .put_object(&format!("thumb/{}", file.name), &v)
                                    .await
                                {
                                    task_tx
                                        .send(Err((
                                            file.name.to_owned(),
                                            format!("Error putting {}: {}", file.name, err),
                                        )))
                                        .unwrap();
                                }
                            }
                            Err(e) => {
                                task_tx.send(Err((file.name, e.to_string()))).unwrap();
                                return;
                            }
                        };
                    }
                    Err(err) => {
                        task_tx
                            .send(Err((
                                file.name.to_owned(),
                                format!("Error getting {}: {}", file.name, err),
                            )))
                            .unwrap();
                    }
                }

                // Send completion status of image.
                task_tx.send(Ok(file.name)).unwrap();
            });
        }
    }

    // All errors produced while generating images.
    let mut errors = vec![];

    // Wait for all tasks to finish and handle completions.
    while let Some(message) = rx.recv().await {
        match message {
            Ok(name) => progress.set_message(name),
            Err((name, error)) => {
                progress.set_message(name.to_owned());
                errors.push(format!("{}: {}", name, error))
            }
        };

        progress.inc(1);

        // Drop the channel receiver and exit once everything has been recieved.
        if progress.position() == progress.length().unwrap() {
            drop(tx);
            break;
        }
    }

    progress.finish_with_message("Finished generating thumbnails");

    // If there were any errors then we should log them.
    if errors.len() > 0 {
        log::warn!(
            "Completed with {} errors:\n{}",
            errors.len(),
            errors.join("\n")
        );
    }

    // Shutdown the temporary runtime for this operation.
    runtime.shutdown_background();

    Ok(())
}
