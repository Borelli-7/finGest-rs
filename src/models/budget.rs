use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

use crate::models::{Category, DateRange, Money};

#[derive(Debug, Clone, Serialize, Deserialize, Validate, FromRow)]
pub struct Budget {
    pub id: Option<i32>,
    
    #[validate]
    pub category: Category,
    
    #[validate]
    pub total: Money,
    
    #[validate]
    pub date_range: DateRange,
}

// DTO for budget output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetOutputDto {
    pub id: Option<i32>,
    pub category: Category,
    pub total: Money,
    pub date_range: DateRange,
    pub spent: Money,
    pub left: Money,
}

// DTO for budget input
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct BudgetInputDto {
    #[validate]
    pub category: Category,
    
    #[validate]
    pub total: Money,
    
    #[serde(rename = "dateRange")]
    #[validate]
    pub date_range: DateRange,
}

impl From<Budget> for BudgetOutputDto {
    fn from(budget: Budget) -> Self {
        Self {
            id: budget.id,
            category: budget.category,
            total: budget.total.clone(),
            date_range: budget.date_range,
            spent: Money::zero(), // This would be calculated in the service layer
            left: budget.total,   // This would be calculated in the service layer
        }
    }
}

impl From<BudgetInputDto> for Budget {
    fn from(dto: BudgetInputDto) -> Self {
        Self {
            id: None,
            category: dto.category,
            total: dto.total,
            date_range: dto.date_range,
        }
    }
}
