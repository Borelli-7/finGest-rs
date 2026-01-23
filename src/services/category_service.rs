use sqlx::{PgPool, Row};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;

use crate::{
    errors::AppError,
    models::{Category, CreateCategoryDto},
};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CategoryServiceTrait: Send + Sync {
    async fn get_categories(&self) -> Result<Vec<Category>, AppError>;
    async fn create_category(&self, dto: CreateCategoryDto) -> Result<Category, AppError>;
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

    async fn create_category(&self, dto: CreateCategoryDto) -> Result<Category, AppError> {
        // Check if category already exists (case-insensitive for name)
        let existing = sqlx::query(
            "SELECT name, profit FROM category WHERE LOWER(name) = LOWER($1) AND profit = $2"
        )
        .bind(&dto.name)
        .bind(dto.profit)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if existing.is_some() {
            return Err(AppError::ConflictError(
                format!("Category '{}' with profit={} already exists", dto.name, dto.profit)
            ));
        }

        // Insert the new category
        let category = sqlx::query("INSERT INTO category (name, profit) VALUES ($1, $2) RETURNING name, profit")
            .bind(&dto.name)
            .bind(dto.profit)
            .map(|row: sqlx::postgres::PgRow| {
                Category {
                    name: row.get("name"),
                    profit: row.get("profit"),
                }
            })
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(category)
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
            async fn create_category(&self, dto: CreateCategoryDto) -> Result<Category, AppError>;
        }
    }

    #[test]
    fn test_category_service_new() {
        // This is just a simple smoke test
        // Real database tests would require a test database setup
    }
}
