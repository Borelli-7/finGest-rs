use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

use crate::models::{DateRange, Money};

#[derive(Debug, Clone, Serialize, Deserialize, Validate, FromRow)]
pub struct Saving {
    pub id: Option<i32>,
    
    #[validate(length(min = 1, message = "Saving name cannot be empty"))]
    pub name: String,
    
    #[validate(nested)]
    pub goal: Money,
    
    #[validate(nested)]
    pub current: Money,
    
    #[validate(nested)]
    pub date_range: DateRange,
}

// DTO for saving input
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct SavingInputDto {
    #[validate(length(min = 1, message = "Saving name cannot be empty"))]
    pub name: String,
    
    #[validate(nested)]
    pub goal: Money,
    
    #[validate(nested)]
    pub current: Money,
    
    #[serde(rename = "dateRange")]
    #[validate(nested)]
    pub date_range: DateRange,
}

impl From<SavingInputDto> for Saving {
    fn from(dto: SavingInputDto) -> Self {
        Self {
            id: None,
            name: dto.name,
            goal: dto.goal,
            current: dto.current,
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
    fn test_saving_validation_success() {
        let saving = Saving {
            id: Some(1),
            name: "Vacation Fund".to_string(),
            goal: Money::new(BigDecimal::from(5000), None),
            current: Money::new(BigDecimal::from(1500), None),
            date_range: DateRange::new(
                Some(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()),
                Some(NaiveDate::from_ymd_opt(2023, 12, 31).unwrap()),
            ),
        };
        assert!(saving.validate().is_ok());
    }
    
    #[test]
    fn test_saving_validation_empty_name() {
        let saving = Saving {
            id: None,
            name: "".to_string(),
            goal: Money::zero(),
            current: Money::zero(),
            date_range: DateRange::default(),
        };
        assert!(saving.validate().is_err());
    }
    
    #[test]
    fn test_saving_input_dto_to_saving_conversion() {
        let dto = SavingInputDto {
            name: "Emergency Fund".to_string(),
            goal: Money::new(BigDecimal::from(10000), Some("EUR".to_string())),
            current: Money::new(BigDecimal::from(3000), Some("EUR".to_string())),
            date_range: DateRange::default(),
        };
        let saving: Saving = dto.into();
        assert_eq!(saving.id, None);
        assert_eq!(saving.name, "Emergency Fund");
        assert_eq!(saving.goal.amount, BigDecimal::from(10000));
        assert_eq!(saving.current.amount, BigDecimal::from(3000));
    }
}
