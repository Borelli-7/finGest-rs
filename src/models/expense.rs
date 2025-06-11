use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

use crate::models::{Category, Money};

#[derive(Debug, Clone, Serialize, Deserialize, Validate, FromRow)]
pub struct Expense {
    pub id: Option<i32>,
    
    #[validate]
    pub amount: Money,
    
    pub date: NaiveDate,
    
    #[validate(length(min = 1, max = 255, message = "Description must be between 1 and 255 characters"))]
    pub description: String,
    
    #[validate]
    pub category: Category,
}

// DTO for expense input
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct ExpenseInputDto {
    #[validate]
    pub amount: Money,
    
    pub date: NaiveDate,
    
    #[validate(length(min = 1, max = 255, message = "Description must be between 1 and 255 characters"))]
    pub description: String,
    
    #[validate]
    pub category: Category,
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
