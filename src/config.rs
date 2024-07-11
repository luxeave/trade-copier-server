use dotenv::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub server_addr: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv().ok();

        Ok(Config {
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| "trade_copier.db".to_string()),
            server_addr: env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string()),
        })
    }
}