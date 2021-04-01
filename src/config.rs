use dotenv::dotenv;
use rusoto_core::Region;
use std::env;

pub struct Config {
    pub port: u16,
    pub storage_url: String,
    pub database_url: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,
    pub s3_bucket: String,
    pub s3_region: Region,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        Config {
            port: env::var("PORT").unwrap().parse::<u16>().unwrap(),
            storage_url: env::var("STORAGE_BASEURL").unwrap(),
            database_url: env::var("DATABASE_URL").unwrap(),
            s3_access_key: env::var("S3_ACCESS_KEY").unwrap(),
            s3_secret_key: env::var("S3_SECRET_KEY").unwrap(),
            s3_bucket: env::var("S3_BUCKET").unwrap(),
            s3_region: Region::Custom {
                name: env::var("S3_REGION").unwrap(),
                endpoint: env::var("S3_ENDPOINT").unwrap(),
            }
        }
    }
}