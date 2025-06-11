use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

use crate::models::{DateRange, Money};

#[derive(Debug, Clone, Serialize, Deserialize, Validate, FromRow)]
pub struct Saving {
    pub id: Option<i32>,
    
    #[validate(length(min = 1, message = "Saving name cannot be empty"))]
    pub name: String,
    
    #[validate]
    pub goal: Money,
    
    #[validate]
    pub current: Money,
    
    #[validate]
    pub date_range: DateRange,
}

// DTO for saving input
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct SavingInputDto {
    #[validate(length(min = 1, message = "Saving name cannot be empty"))]
    pub name: String,
    
    #[validate]
    pub goal: Money,
    
    #[validate]
    pub current: Money,
    
    #[serde(rename = "dateRange")]
    #[validate]
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
