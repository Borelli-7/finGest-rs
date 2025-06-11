use std::env;
use std::net::Ipv4Addr;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub db_url: String,
    pub db_max_connections: u32,
    pub log_level: String,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnv(String),

    #[error("Invalid environment variable: {0}")]
    InvalidEnv(String),
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let host = env::var("HOST").unwrap_or_else(|_| Ipv4Addr::LOCALHOST.to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .map_err(|_| ConfigError::InvalidEnv("PORT".to_string()))?;

        let db_url = env::var("DATABASE_URL").map_err(|_| ConfigError::MissingEnv("DATABASE_URL".to_string()))?;
        let db_max_connections = env::var("DB_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .map_err(|_| ConfigError::InvalidEnv("DB_MAX_CONNECTIONS".to_string()))?;
        
        let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

        Ok(Self {
            host,
            port,
            db_url,
            db_max_connections,
            log_level,
        })
    }
}