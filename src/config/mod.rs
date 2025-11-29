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
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
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
        
        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| ConfigError::MissingEnv("JWT_SECRET".to_string()))?;
        
        let jwt_expiration_hours = env::var("JWT_EXPIRATION_HOURS")
            .unwrap_or_else(|_| "24".to_string())
            .parse()
            .map_err(|_| ConfigError::InvalidEnv("JWT_EXPIRATION_HOURS".to_string()))?;

        Ok(Self {
            host,
            port,
            db_url,
            db_max_connections,
            log_level,
            jwt_secret,
            jwt_expiration_hours,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    
    #[test]
    fn test_config_from_env_with_all_vars() {
        unsafe {
            env::set_var("HOST", "0.0.0.0");
            env::set_var("PORT", "3000");
            env::set_var("DATABASE_URL", "postgres://localhost/testdb");
            env::set_var("DB_MAX_CONNECTIONS", "5");
            env::set_var("RUST_LOG", "debug");
            env::set_var("JWT_SECRET", "test_secret_key");
            env::set_var("JWT_EXPIRATION_HOURS", "48");
        }
        
        let config = Config::from_env().unwrap();
        
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 3000);
        assert_eq!(config.db_url, "postgres://localhost/testdb");
        assert_eq!(config.db_max_connections, 5);
        assert_eq!(config.log_level, "debug");
        assert_eq!(config.jwt_secret, "test_secret_key");
        assert_eq!(config.jwt_expiration_hours, 48);
        
        // Cleanup
        unsafe {
            env::remove_var("HOST");
            env::remove_var("PORT");
            env::remove_var("DATABASE_URL");
            env::remove_var("DB_MAX_CONNECTIONS");
            env::remove_var("RUST_LOG");
            env::remove_var("JWT_SECRET");
            env::remove_var("JWT_EXPIRATION_HOURS");
        }
    }
    
    #[test]
    fn test_config_from_env_with_defaults() {
        unsafe {
            env::remove_var("HOST");
            env::remove_var("PORT");
            env::set_var("DATABASE_URL", "postgres://localhost/testdb");
            env::remove_var("DB_MAX_CONNECTIONS");
            env::remove_var("RUST_LOG");
            env::set_var("JWT_SECRET", "test_secret");
            env::remove_var("JWT_EXPIRATION_HOURS");
        }
        
        let config = Config::from_env().unwrap();
        
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8080);
        assert_eq!(config.db_max_connections, 10);
        assert_eq!(config.log_level, "info");
        assert_eq!(config.jwt_expiration_hours, 24);
        
        // Cleanup
        unsafe {
            env::remove_var("DATABASE_URL");
            env::remove_var("JWT_SECRET");
        }
    }
    
    #[test]
    fn test_config_from_env_missing_database_url() {
        // Save current env state
        let saved_jwt = env::var("JWT_SECRET").ok();
        
        unsafe {
            env::remove_var("DATABASE_URL");
            env::set_var("JWT_SECRET", "test_secret");
        }
        
        let result = Config::from_env();
        
        // Restore env
        unsafe {
            if let Some(jwt) = saved_jwt {
                env::set_var("JWT_SECRET", jwt);
            } else {
                env::remove_var("JWT_SECRET");
            }
        }
        
        assert!(result.is_err());
        
        match result {
            Err(ConfigError::MissingEnv(var)) => {
                assert_eq!(var, "DATABASE_URL");
            }
            _ => panic!("Expected MissingEnv error for DATABASE_URL"),
        }
    }
    
    #[test]
    fn test_config_from_env_missing_jwt_secret() {
        // Save current env state
        let saved_db = env::var("DATABASE_URL").ok();
        
        unsafe {
            env::set_var("DATABASE_URL", "postgres://localhost/testdb");
            env::remove_var("JWT_SECRET");
        }
        
        let result = Config::from_env();
        
        // Restore env
        unsafe {
            env::remove_var("DATABASE_URL");
            if let Some(db) = saved_db {
                env::set_var("DATABASE_URL", db);
            }
        }
        
        assert!(result.is_err());
        
        match result {
            Err(ConfigError::MissingEnv(var)) => {
                assert_eq!(var, "JWT_SECRET");
            }
            _ => panic!("Expected MissingEnv error for JWT_SECRET"),
        }
    }
    
    #[test]
    fn test_config_from_env_invalid_port() {
        unsafe {
            env::set_var("PORT", "invalid");
            env::set_var("DATABASE_URL", "postgres://localhost/testdb");
            env::set_var("JWT_SECRET", "test_secret");
        }
        
        let result = Config::from_env();
        assert!(result.is_err());
        
        if let Err(ConfigError::InvalidEnv(var)) = result {
            assert_eq!(var, "PORT");
        } else {
            panic!("Expected InvalidEnv error");
        }
        
        unsafe {
            env::remove_var("PORT");
            env::remove_var("DATABASE_URL");
            env::remove_var("JWT_SECRET");
        }
    }
}