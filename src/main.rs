#![feature(macro_attributes_in_derive_output)]
#![feature(panic_info_message)]

use std::panic;
use std::panic::PanicInfo;
use std::path::Path;

use actix_web::{*, middleware::Logger};
use config::StorageConfig;
use storage::{StorageProvider, local::LocalProvider, s3::S3Provider};

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
    // Prettier panic message
    panic::set_hook(Box::new(panic_handle));    

    panic!("bruh");

    // Setup actix log
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let config = config::Config::new();

    let database = database::Database::new(16, &config.database_url).await;
    if let Err(err) = database.run_migrations(Path::new("migrations")).await {
        panic!("{}", err.to_string());
    }

    let storage: Box<dyn StorageProvider> = match config.storage_provider {
        StorageConfig::Local(v) => Box::new(LocalProvider::new(v.path)),
        StorageConfig::S3(v) => Box::new(S3Provider::new(&v.bucket, &v.access_key, &v.secret_key, v.region))
    };
    
    let api_state = web::Data::new(state::State {
        database: database,
        storage: storage,
        jwt_key: config.jwt_key
    });

    HttpServer::new(move || {
        App::new() 
            .wrap(Logger::default())
            .app_data(api_state.clone())
            .service(
                web::scope("/api/")
                    .service(routes::user::get_routes())
                    .service(routes::auth::get_routes())
                    .service(routes::application::get_routes())
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

fn panic_handle<'a>(info: &'a PanicInfo) {
    use colored::Colorize;

    let gray = colored::Color::TrueColor {
        r: (180),
        g: (180),
        b: (180),
    };

    let mut message = String::new();
    if let Some(s) = info.payload().downcast_ref::<&str>() {
        message = (*s).to_string();
    }

    println!("{}", "\nUh oh! A crash was encountered!\n".bright_red().bold());
    println!("{} {}", "Message:".color(gray), message.bright_yellow());
    println!("{} {}", "Location:".color(gray), info.location().unwrap().to_string().bright_yellow());
    println!("\n{}\n{}\n", "If you believe this is a bug please report it at:".color(gray), "https://github.com/Riku32/Backpack/issues".bright_green())
}