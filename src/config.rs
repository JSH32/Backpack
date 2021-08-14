use dotenv::dotenv;
use rusoto_core::Region;
use std::{env, fmt::Debug, path::{Path, PathBuf}, str::FromStr};

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub storage_url: String,
    pub database_url: String,
    pub base_url: String,
    pub jwt_key: String,
    pub serve_frontend: bool,
    pub storage_provider: StorageConfig,
    pub smtp_config: Option<SMTPConfig>
}

#[derive(Clone)]
pub struct S3Config {
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
    pub region: Region
}

#[derive(Clone)]
pub struct LocalConfig {
    pub path: PathBuf,

    // Should local provider directory be served by the application
    // This can be disable if someone wants to serve using some other webserver
    pub serve: bool
}

#[derive(Clone)]
pub struct SMTPConfig {
    pub username: String,
    pub password: String,
    pub server: String
}

#[derive(Clone)]
pub enum StorageConfig {
    Local(LocalConfig),
    S3(S3Config)
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        Config {
            port: get_env("PORT"),
            storage_url: get_env("STORAGE_BASEURL"),
            database_url: get_env("DATABASE_URL"),
            jwt_key: get_env("JWT_KEY"),
            base_url: get_env("BASEURL"),
            serve_frontend: get_env_or("SERVE_FRONTEND", !cfg!(debug_assertions)),
            storage_provider: {
                match get_env::<String>("STORAGE_PROVIDER").as_str() {
                    "local" => StorageConfig::Local(LocalConfig {
                        path: Path::new(get_env::<String>("LOCAL_PATH").as_str()).to_path_buf(),
                        serve: get_env_or("LOCAL_SERVE", true)
                    }),
                    "s3" => StorageConfig::S3(S3Config {
                        bucket: get_env("S3_BUCKET"),
                        access_key: get_env("S3_ACCESS_KEY"),
                        secret_key: get_env("S3_SECRET_KEY"),
                        region: Region::Custom {
                            name: get_env("S3_REGION"),
                            endpoint: get_env("S3_ENDPOINT")
                        }
                    }),
                    _ => panic!("Invalid storage provider for environment variable STORAGE_PROVIDER")
                }
            },
            smtp_config: {
                match get_env_or("SMTP_ENABLED", false) {
                    true => Some(SMTPConfig {
                        username: get_env("SMTP_USERNAME"),
                        password: get_env("SMTP_PASSWORD"),
                        server: get_env("SMTP_SERVER")
                    }),
                    false => None,
                }
            }
        }
    }
}

fn get_env_or<T>(var: &str, default: T) -> T where T: FromStr, <T as FromStr>::Err: Debug {
    match env::var(var) {
        Ok(v) => v.parse::<T>()
            .expect(&format!("Unable to parse {} as {}", var, std::any::type_name::<T>())),
        Err(_) => default,
    }
}

fn get_env<T>(var: &str) -> T where T: FromStr, <T as FromStr>::Err: Debug {
    env::var(var)
        .expect(&format!("Missing environment variable {}", var))
        .parse::<T>()
        .expect(&format!("Unable to parse {} as {}", var, std::any::type_name::<T>()))
}