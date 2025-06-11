use sqlx::{PgPool, Row};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;

use crate::{
    errors::AppError,
    models::Category,
};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CategoryServiceTrait: Send + Sync {
    async fn get_categories(&self) -> Result<Vec<Category>, AppError>;
}

pub struct CategoryService {
    pool: PgPool,
}

impl CategoryService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CategoryServiceTrait for CategoryService {
    async fn get_categories(&self) -> Result<Vec<Category>, AppError> {
        // Using a runtime query instead of compile-time macro for development
        let categories = sqlx::query("SELECT name, profit FROM category")
            .map(|row: sqlx::postgres::PgRow| {
                Category {
                    name: row.get("name"),
                    profit: row.get("profit"),
                }
            })
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(categories)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;

    mock! {
        pub CategoryService {}

        #[async_trait]
        impl CategoryServiceTrait for CategoryService {
            async fn get_categories(&self) -> Result<Vec<Category>, AppError>;
        }
    }
}
