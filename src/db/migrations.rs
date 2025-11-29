use sqlx::{postgres::PgPool, migrate::MigrateDatabase, Postgres};
use crate::errors::AppError;

pub async fn run(pool: &PgPool) -> Result<(), AppError> {
    tracing::info!("Running database migrations");
    
    // Use SQLx migrations from the migrations directory
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    tracing::info!("Database migrations completed successfully");
    
    Ok(())
}

#[allow(dead_code)]
pub async fn create_database_if_not_exists(url: &str) -> Result<(), AppError> {
    let database_url_parts: Vec<&str> = url.split('/').collect();
    
    if let Some(database_name) = database_url_parts.last() {
        if !database_name.contains('?') {
            let _base_url = database_url_parts[..database_url_parts.len() - 1].join("/");
            
            if !Postgres::database_exists(&url).await.unwrap_or(false) {
                tracing::info!("Creating database: {}", database_name);
                Postgres::create_database(&url).await
                    .map_err(|e| AppError::DatabaseError(format!("Failed to create database: {}", e)))?;
            }
        } else {
            // Handle URL with query parameters
            let db_with_params: Vec<&str> = database_name.split('?').collect();
            let db_name = db_with_params[0];
            
            let base_url = format!("{}/{}", 
                database_url_parts[..database_url_parts.len() - 1].join("/"), 
                db_name
            );
            
            if !Postgres::database_exists(&base_url).await.unwrap_or(false) {
                tracing::info!("Creating database: {}", db_name);
                Postgres::create_database(&base_url).await
                    .map_err(|e| AppError::DatabaseError(format!("Failed to create database: {}", e)))?;
            }
        }
    }
    
    Ok(())
}