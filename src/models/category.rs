use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, FromRow, PartialEq, Eq, Hash)]
pub struct Category {
    #[validate(length(min = 1, message = "Category name cannot be empty"))]
    pub name: String,
    pub profit: bool,
}

impl Category {
    pub fn new(name: String, profit: bool) -> Self {
        Self { name, profit }
    }
}

// This struct represents the composite primary key for Category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryPK {
    pub name: String,
    pub profit: bool,
}