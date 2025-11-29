use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

use crate::models::{Expense, Money};

#[derive(Debug, Clone, Serialize, Deserialize, Validate, FromRow)]
pub struct Wallet {
    pub id: Option<i32>,
    
    #[validate(length(min = 1, message = "Wallet name cannot be empty"))]
    pub name: String,
    
    #[validate(nested)]
    pub amount: Money,
}

// Wallet with expenses included
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletWithExpenses {
    pub id: Option<i32>,
    pub name: String,
    pub amount: Money,
    pub expenses: Vec<Expense>,
}

// DTO for Wallet responses and requests
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WalletDto {
    pub id: Option<i32>,
    
    #[validate(length(min = 1, message = "Wallet name cannot be empty"))]
    pub name: String,
    
    #[validate(nested)]
    pub amount: Money,
}

impl From<Wallet> for WalletDto {
    fn from(wallet: Wallet) -> Self {
        Self {
            id: wallet.id,
            name: wallet.name,
            amount: wallet.amount,
        }
    }
}

impl From<WalletDto> for Wallet {
    fn from(dto: WalletDto) -> Self {
        Self {
            id: dto.id,
            name: dto.name,
            amount: dto.amount,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::BigDecimal;
    use validator::Validate;
    
    #[test]
    fn test_wallet_validation_success() {
        let wallet = Wallet {
            id: Some(1),
            name: "My Wallet".to_string(),
            amount: Money::new(BigDecimal::from(100), None),
        };
        assert!(wallet.validate().is_ok());
    }
    
    #[test]
    fn test_wallet_validation_empty_name() {
        let wallet = Wallet {
            id: None,
            name: "".to_string(),
            amount: Money::zero(),
        };
        assert!(wallet.validate().is_err());
    }
    
    #[test]
    fn test_wallet_dto_to_wallet_conversion() {
        let dto = WalletDto {
            id: Some(5),
            name: "Test Wallet".to_string(),
            amount: Money::new(BigDecimal::from(250), Some("USD".to_string())),
        };
        let wallet: Wallet = dto.into();
        assert_eq!(wallet.id, Some(5));
        assert_eq!(wallet.name, "Test Wallet");
        assert_eq!(wallet.amount.currency, "USD");
    }
    
    #[test]
    fn test_wallet_to_dto_conversion() {
        let wallet = Wallet {
            id: Some(10),
            name: "Savings".to_string(),
            amount: Money::new(BigDecimal::from(1000), None),
        };
        let dto: WalletDto = wallet.into();
        assert_eq!(dto.id, Some(10));
        assert_eq!(dto.name, "Savings");
        assert_eq!(dto.amount.currency, "PLN");
    }
}
