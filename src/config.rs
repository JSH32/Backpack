use dotenv::dotenv;
use std::env;

pub struct Config {
    pub database_url: String
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        Config {
            database_url: env::var("DATABASE_URL").unwrap()
        }
    }    
}