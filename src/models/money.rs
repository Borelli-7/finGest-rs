use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::cmp::Ordering;
use std::str::FromStr;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, FromRow)]
pub struct Money { 
    #[validate(custom(function = "validate_non_negative"))]
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_money_new_with_currency() {
        let money = Money::new(BigDecimal::from(100), Some("USD".to_string()));
        assert_eq!(money.amount, BigDecimal::from(100));
        assert_eq!(money.currency, "USD");
    }
    
    #[test]
    fn test_money_new_with_default_currency() {
        let money = Money::new(BigDecimal::from(50), None);
        assert_eq!(money.amount, BigDecimal::from(50));
        assert_eq!(money.currency, "PLN");
    }
    
    #[test]
    fn test_money_zero() {
        let money = Money::zero();
        assert_eq!(money.amount, BigDecimal::from(0));
        assert_eq!(money.currency, "PLN");
    }
    
    #[test]
    fn test_money_from_str_success() {
        let money = Money::from_str("123.45", Some("EUR")).unwrap();
        assert_eq!(money.amount.to_string(), "123.45");
        assert_eq!(money.currency, "EUR");
    }
    
    #[test]
    fn test_money_from_str_default_currency() {
        let money = Money::from_str("99.99", None).unwrap();
        assert_eq!(money.amount.to_string(), "99.99");
        assert_eq!(money.currency, "PLN");
    }
    
    #[test]
    fn test_money_from_str_invalid() {
        let result = Money::from_str("invalid", None);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_money_equality_same_currency() {
        let money1 = Money::new(BigDecimal::from(100), Some("USD".to_string()));
        let money2 = Money::new(BigDecimal::from(100), Some("USD".to_string()));
        assert_eq!(money1, money2);
    }
    
    #[test]
    fn test_money_inequality_different_amount() {
        let money1 = Money::new(BigDecimal::from(100), Some("USD".to_string()));
        let money2 = Money::new(BigDecimal::from(200), Some("USD".to_string()));
        assert_ne!(money1, money2);
    }
    
    #[test]
    fn test_money_inequality_different_currency() {
        let money1 = Money::new(BigDecimal::from(100), Some("USD".to_string()));
        let money2 = Money::new(BigDecimal::from(100), Some("EUR".to_string()));
        assert_ne!(money1, money2);
    }
    
    #[test]
    fn test_money_partial_ord_same_currency() {
        let money1 = Money::new(BigDecimal::from(50), Some("USD".to_string()));
        let money2 = Money::new(BigDecimal::from(100), Some("USD".to_string()));
        assert!(money1 < money2);
        assert!(money2 > money1);
    }
    
    #[test]
    fn test_money_partial_ord_different_currency() {
        let money1 = Money::new(BigDecimal::from(50), Some("USD".to_string()));
        let money2 = Money::new(BigDecimal::from(100), Some("EUR".to_string()));
        assert_eq!(money1.partial_cmp(&money2), None);
    }
    
    #[test]
    fn test_validate_non_negative_valid() {
        let result = validate_non_negative(&BigDecimal::from(100));
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validate_non_negative_zero() {
        let result = validate_non_negative(&BigDecimal::from(0));
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validate_non_negative_invalid() {
        let result = validate_non_negative(&BigDecimal::from(-50));
        assert!(result.is_err());
    }
}
