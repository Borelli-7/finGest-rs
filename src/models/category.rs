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

// DTO for creating a new category
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateCategoryDto {
    #[validate(length(min = 1, max = 255, message = "Category name must be between 1 and 255 characters"))]
    pub name: String,
    pub profit: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;
    
    #[test]
    fn test_category_new() {
        let category = Category::new("Food".to_string(), false);
        assert_eq!(category.name, "Food");
        assert_eq!(category.profit, false);
    }
    
    #[test]
    fn test_category_validation_success() {
        let category = Category::new("Transport".to_string(), false);
        assert!(category.validate().is_ok());
    }
    
    #[test]
    fn test_category_validation_empty_name() {
        let category = Category::new("".to_string(), true);
        assert!(category.validate().is_err());
    }
    
    #[test]
    fn test_category_equality() {
        let cat1 = Category::new("Food".to_string(), false);
        let cat2 = Category::new("Food".to_string(), false);
        assert_eq!(cat1, cat2);
    }
    
    #[test]
    fn test_category_inequality_name() {
        let cat1 = Category::new("Food".to_string(), false);
        let cat2 = Category::new("Transport".to_string(), false);
        assert_ne!(cat1, cat2);
    }
    
    #[test]
    fn test_category_inequality_profit() {
        let cat1 = Category::new("Salary".to_string(), true);
        let cat2 = Category::new("Salary".to_string(), false);
        assert_ne!(cat1, cat2);
    }

    #[test]
    fn test_create_category_dto_validation_success() {
        let dto = CreateCategoryDto {
            name: "Groceries".to_string(),
            profit: false,
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn test_create_category_dto_validation_empty_name() {
        let dto = CreateCategoryDto {
            name: "".to_string(),
            profit: false,
        };
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_create_category_dto_validation_too_long_name() {
        let dto = CreateCategoryDto {
            name: "a".repeat(256),
            profit: false,
        };
        assert!(dto.validate().is_err());
    }
}