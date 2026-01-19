use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

use crate::models::{Category, DateRange, Money};

#[derive(Debug, Clone, Serialize, Deserialize, Validate, FromRow)]
pub struct Budget {
    pub id: Option<i32>,
    
    #[validate(nested)]
    pub category: Category,
    
    #[validate(nested)]
    pub total: Money,
    
    #[validate(nested)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct BudgetInputDto {
    #[validate(nested)]
    pub category: Category,
    
    #[validate(nested)]
    pub total: Money,
    
    #[serde(rename = "dateRange")]
    #[validate(nested)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::BigDecimal;
    use chrono::NaiveDate;
    use validator::Validate;
    
    #[test]
    fn test_budget_validation_success() {
        let budget = Budget {
            id: Some(1),
            category: Category::new("Food".to_string(), false),
            total: Money::new(BigDecimal::from(500), None),
            date_range: DateRange::new(
                Some(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()),
                Some(NaiveDate::from_ymd_opt(2023, 12, 31).unwrap()),
            ),
        };
        assert!(budget.validate().is_ok());
    }
    
    #[test]
    fn test_budget_to_output_dto_conversion() {
        let budget = Budget {
            id: Some(10),
            category: Category::new("Transport".to_string(), false),
            total: Money::new(BigDecimal::from(300), Some("USD".to_string())),
            date_range: DateRange::new(
                Some(NaiveDate::from_ymd_opt(2023, 6, 1).unwrap()),
                Some(NaiveDate::from_ymd_opt(2023, 6, 30).unwrap()),
            ),
        };
        let dto: BudgetOutputDto = budget.into();
        assert_eq!(dto.id, Some(10));
        assert_eq!(dto.category.name, "Transport");
        assert_eq!(dto.total.currency, "USD");
        assert_eq!(dto.spent.amount, BigDecimal::from(0));
        assert_eq!(dto.left.amount, BigDecimal::from(300));
    }
    
    #[test]
    fn test_budget_input_dto_to_budget_conversion() {
        let dto = BudgetInputDto {
            category: Category::new("Entertainment".to_string(), false),
            total: Money::new(BigDecimal::from(200), None),
            date_range: DateRange::default(),
        };
        let budget: Budget = dto.into();
        assert_eq!(budget.id, None);
        assert_eq!(budget.category.name, "Entertainment");
        assert_eq!(budget.total.amount, BigDecimal::from(200));
    }
}
