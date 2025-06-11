use sqlx::postgres::PgPool;
use crate::errors::AppError;

mod migrations;
pub mod schema;

pub async fn init_db(pool: &PgPool) -> Result<(), AppError> {
    // Run migrations
    migrations::run(pool).await?;
    
    // Initialize any seed data if needed
    
    Ok(())
}

pub trait Repository<T, ID> {
    async fn find_all(&self) -> Result<Vec<T>, AppError>;
    async fn find_by_id(&self, id: ID) -> Result<Option<T>, AppError>;
    async fn create(&self, item: T) -> Result<T, AppError>;
    async fn update(&self, id: ID, item: T) -> Result<T, AppError>;
    async fn delete(&self, id: ID) -> Result<(), AppError>;
}