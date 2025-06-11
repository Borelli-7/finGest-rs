use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::cmp::Ordering;
use std::str::FromStr;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, FromRow)]
pub struct Money { 
    #[validate(custom = "validate_non_negative")]
    pub amount: BigDecimal,
    pub currency: String,
}

impl Money {
    const DEFAULT_CURRENCY: &'static str = "PLN";

    pub fn new(amount: BigDecimal, currency: Option<String>) -> Self {
        Self {
            amount,
            currency: currency.unwrap_or_else(|| Self::DEFAULT_CURRENCY.to_string()),
        }
    }

    pub fn zero() -> Self {
        Self {
            amount: BigDecimal::from(0),
            currency: Self::DEFAULT_CURRENCY.to_string(),
        }
    }

    pub fn from_str(amount: &str, currency: Option<&str>) -> Result<Self, bigdecimal::ParseBigDecimalError> {
        let amount = BigDecimal::from_str(amount)?;
        Ok(Self {
            amount,
            currency: currency
                .map(|c| c.to_string())
                .unwrap_or_else(|| Self::DEFAULT_CURRENCY.to_string()),
        })
    }
}

impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        self.currency == other.currency && self.amount == other.amount
    }
}

impl PartialOrd for Money {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.currency != other.currency {
            None
        } else {
            self.amount.partial_cmp(&other.amount)
        }
    }
}

fn validate_non_negative(amount: &BigDecimal) -> Result<(), validator::ValidationError> {
    if amount < &BigDecimal::from(0) {
        return Err(validator::ValidationError::new("negative_amount"));
    }
    Ok(())
}
