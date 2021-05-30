use dotenv::dotenv;
use rusoto_core::Region;
use std::{env, fmt::Debug, path::{Path, PathBuf}, str::FromStr};

pub struct Config {
    pub port: u16,
    pub storage_url: String,
    pub database_url: String,
    pub jwt_key: String,
    pub storage_provider: StorageConfig,
}

pub struct S3Config {
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
    pub region: Region
}

pub struct LocalConfig {
    pub path: PathBuf,

    // Should local provider directory be served by the application
    // This can be disable if someone wants to serve using some other webserver
    pub serve: bool
}

pub enum StorageConfig {
    Local(LocalConfig),
    S3(S3Config)
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        Config {
            port: get_env::<u16>("PORT"),
            storage_url: get_env::<String>("STORAGE_BASEURL"),
            database_url: get_env::<String>("DATABASE_URL"),
            jwt_key: get_env::<String>("JWT_KEY"),
            storage_provider: {
                match get_env::<String>("STORAGE_PROVIDER").as_str() {
                    "local" => StorageConfig::Local(LocalConfig {
                        path: Path::new(get_env::<String>("LOCAL_PATH").as_str()).to_path_buf(),
                        serve: match env::var("LOCAL_SERVE").ok() {
                            Some(v) => v.parse().expect("LOCAL_SERVE must be true or false"),
                            None => false
                        }
                    }),
                    "s3" => StorageConfig::S3(S3Config {
                        bucket: get_env::<String>("S3_BUCKET"),
                        access_key: get_env::<String>("S3_ACCESS_KEY"),
                        secret_key: get_env::<String>("S3_SECRET_KEY"),
                        region: Region::Custom {
                            name: get_env::<String>("S3_REGION"),
                            endpoint: get_env::<String>("S3_ENDPOINT")
                        }
                    }),
                    _ => panic!("Invalid storage provider for environment variable STORAGE_PROVIDER")
                }
            }
        }
    }
}

fn get_env<T>(var: &str) -> T where T: FromStr, <T as FromStr>::Err: Debug {
    env::var(var)
        .expect(&format!("Missing environment variable {}", var))
        .parse::<T>()
        .expect(&format!("Unable to parse {} as {}", var, std::any::type_name::<T>()))
}