use sqlx::{PgPool, Row};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;

use crate::{
    errors::AppError,
    models::{Category, CreateCategoryDto, UpdateCategoryDto},
};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CategoryServiceTrait: Send + Sync {
    async fn get_categories(&self) -> Result<Vec<Category>, AppError>;
    async fn create_category(&self, dto: CreateCategoryDto) -> Result<Category, AppError>;
    async fn update_category(&self, name: String, profit: bool, dto: UpdateCategoryDto) -> Result<Category, AppError>;
    async fn delete_category(&self, name: String, profit: bool) -> Result<(), AppError>;
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

    async fn update_category(&self, name: String, profit: bool, dto: UpdateCategoryDto) -> Result<Category, AppError> {
        // First, check if the category exists
        let existing = sqlx::query(
            "SELECT name, profit FROM category WHERE name = $1 AND profit = $2"
        )
        .bind(&name)
        .bind(profit)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if existing.is_none() {
            return Err(AppError::NotFoundError(
                format!("Category '{}' with profit={} not found", name, profit)
            ));
        }

        // Check if the new name already exists with the same profit type (conflict check)
        let conflict = sqlx::query(
            "SELECT name, profit FROM category WHERE LOWER(name) = LOWER($1) AND profit = $2 AND name != $3"
        )
        .bind(&dto.new_name)
        .bind(profit)
        .bind(&name)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if conflict.is_some() {
            return Err(AppError::ConflictError(
                format!("Category '{}' with profit={} already exists", dto.new_name, profit)
            ));
        }

        // Update the category name
        let updated_category = sqlx::query(
            "UPDATE category SET name = $1 WHERE name = $2 AND profit = $3 RETURNING name, profit"
        )
        .bind(&dto.new_name)
        .bind(&name)
        .bind(profit)
        .map(|row: sqlx::postgres::PgRow| {
            Category {
                name: row.get("name"),
                profit: row.get("profit"),
            }
        })
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(updated_category)
    }

    async fn delete_category(&self, name: String, profit: bool) -> Result<(), AppError> {
        // First, check if the category exists
        let existing = sqlx::query(
            "SELECT name, profit FROM category WHERE name = $1 AND profit = $2"
        )
        .bind(&name)
        .bind(profit)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if existing.is_none() {
            return Err(AppError::NotFoundError(
                format!("Category '{}' with profit={} not found", name, profit)
            ));
        }

        // Delete the category
        let result = sqlx::query(
            "DELETE FROM category WHERE name = $1 AND profit = $2"
        )
        .bind(&name)
        .bind(profit)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Verify that the deletion occurred
        if result.rows_affected() == 0 {
            return Err(AppError::DatabaseError(
                "Failed to delete category".to_string()
            ));
        }

        Ok(())
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
            async fn update_category(&self, name: String, profit: bool, dto: UpdateCategoryDto) -> Result<Category, AppError>;
            async fn delete_category(&self, name: String, profit: bool) -> Result<(), AppError>;
        }
    }

    #[test]
    fn test_category_service_new() {
        // This is just a simple smoke test
        // Real database tests would require a test database setup
    }
}
