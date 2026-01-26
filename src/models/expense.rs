use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

use crate::models::{Category, Money};

#[derive(Debug, Clone, Serialize, Deserialize, Validate, FromRow)]
pub struct Expense {
    pub id: Option<i32>,
    
    #[validate(nested)]
    pub amount: Money,
    
    pub date: NaiveDate,
    
    #[validate(length(min = 1, max = 255, message = "Description must be between 1 and 255 characters"))]
    pub description: String,
    
    #[validate(nested)]
    pub category: Category,
}

// DTO for expense input
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ExpenseInputDto {
    #[validate(nested)]
    pub amount: Money,
    
    pub date: NaiveDate,
    
    #[validate(length(min = 1, max = 255, message = "Description must be between 1 and 255 characters"))]
    pub description: String,
    
    #[validate(nested)]
    pub category: Category,
}

// DTO for updating existing expenses
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateExpenseDto {
    #[validate(nested)]
    pub amount: Option<Money>,
    
    pub date: Option<NaiveDate>,
    
    #[validate(length(min = 1, max = 255, message = "Description must be between 1 and 255 characters"))]
    pub description: Option<String>,
    
    #[validate(nested)]
    pub category: Option<Category>,
}

impl From<ExpenseInputDto> for Expense {
    fn from(dto: ExpenseInputDto) -> Self {
        Self {
            id: None,
            amount: dto.amount,
            date: dto.date,
            description: dto.description,
            category: dto.category,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::BigDecimal;
    use validator::Validate;
    
    #[test]
    fn test_expense_validation_success() {
        let expense = Expense {
            id: Some(1),
            amount: Money::new(BigDecimal::from(50), None),
            date: NaiveDate::from_ymd_opt(2023, 6, 15).unwrap(),
            description: "Grocery shopping".to_string(),
            category: Category::new("Food".to_string(), false),
        };
        assert!(expense.validate().is_ok());
    }
    
    #[test]
    fn test_expense_validation_empty_description() {
        let expense = Expense {
            id: None,
            amount: Money::zero(),
            date: NaiveDate::from_ymd_opt(2023, 6, 15).unwrap(),
            description: "".to_string(),
            category: Category::new("Food".to_string(), false),
        };
        assert!(expense.validate().is_err());
    }
    
    #[test]
    fn test_expense_validation_description_too_long() {
        let long_description = "a".repeat(256);
        let expense = Expense {
            id: None,
            amount: Money::zero(),
            date: NaiveDate::from_ymd_opt(2023, 6, 15).unwrap(),
            description: long_description,
            category: Category::new("Food".to_string(), false),
        };
        assert!(expense.validate().is_err());
    }
    
    #[test]
    fn test_expense_input_dto_to_expense_conversion() {
        let dto = ExpenseInputDto {
            amount: Money::new(BigDecimal::from(75), Some("EUR".to_string())),
            date: NaiveDate::from_ymd_opt(2023, 7, 20).unwrap(),
            description: "Restaurant".to_string(),
            category: Category::new("Food".to_string(), false),
        };
        let expense: Expense = dto.into();
        assert_eq!(expense.id, None);
        assert_eq!(expense.amount.currency, "EUR");
        assert_eq!(expense.description, "Restaurant");
    }
    
    #[test]
    fn test_update_expense_dto_validation_success() {
        let update_dto = UpdateExpenseDto {
            amount: Some(Money::new(BigDecimal::from(150), Some("EUR".to_string()))),
            date: Some(NaiveDate::from_ymd_opt(2023, 8, 15).unwrap()),
            description: Some("Updated grocery shopping".to_string()),
            category: Some(Category::new("Food".to_string(), false)),
        };
        assert!(update_dto.validate().is_ok());
    }
    
    #[test]
    fn test_update_expense_dto_partial_update() {
        let update_dto = UpdateExpenseDto {
            amount: None,
            date: None,
            description: Some("Only description updated".to_string()),
            category: None,
        };
        assert!(update_dto.validate().is_ok());
    }
    
    #[test]
    fn test_update_expense_dto_empty_description_validation_error() {
        let update_dto = UpdateExpenseDto {
            amount: None,
            date: None,
            description: Some("".to_string()),
            category: None,
        };
        assert!(update_dto.validate().is_err());
    }
}
