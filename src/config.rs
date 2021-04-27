use dotenv::dotenv;
use rusoto_core::Region;
use std::{env, path::{Path, PathBuf}};

pub struct Config {
    pub port: u16,
    pub storage_url: String,
    pub database_url: String,
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
            port: env::var("PORT").unwrap().parse::<u16>().unwrap(),
            storage_url: env::var("STORAGE_BASEURL").unwrap(),
            database_url: env::var("DATABASE_URL").unwrap(),
            storage_provider: {
                match env::var("STORAGE_PROVIDER").unwrap().as_str() {
                    "local" => StorageConfig::Local(LocalConfig {
                        path: Path::new(env::var("LOCAL_PATH").unwrap().as_str()).to_path_buf(),
                        serve: match env::var("LOCAL_SERVE").ok() {
                            Some(v) => v.parse().unwrap_or(false),
                            None => false
                        }
                    }),
                    "s3" => StorageConfig::S3(S3Config {
                        bucket: env::var("S3_BUCKET").unwrap(),
                        access_key: env::var("S3_ACCESS_KEY").unwrap(),
                        secret_key: env::var("S3_SECRET_KEY").unwrap(),
                        region: Region::Custom {
                            name: env::var("S3_REGION").unwrap(),
                            endpoint: env::var("S3_ENDPOINT").unwrap()
                        }
                    }),
                    _ => panic!("Invalid storage provider for environment variable STORAGE_PROVIDER")
                }
            }
        }
    }
}