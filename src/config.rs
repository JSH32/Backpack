pub struct Config {
    pub database_url: String
}

impl Config {
    pub fn new() -> Self {
        Config {
            database_url: dotenv!("DATABASE_URL").to_string()
        }
    }    
}