use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

use crate::models::{Expense, Money};

#[derive(Debug, Clone, Serialize, Deserialize, Validate, FromRow)]
pub struct Wallet {
    pub id: Option<i32>,
    
    #[validate(length(min = 1, message = "Wallet name cannot be empty"))]
    pub name: String,
    
    #[validate]
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
    
    #[validate]
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
