use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::models::Money;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    pub wallet_name: String,
    pub balance: Money,
    pub expense_categories: HashMap<String, Money>,
    pub income_categories: HashMap<String, Money>,
    pub total_expense: Money,
    pub total_income: Money,
}

impl Summary {
    pub fn new(wallet_name: String, balance: Money) -> Self {
        Self {
            wallet_name,
            balance,
            expense_categories: HashMap::new(),
            income_categories: HashMap::new(),
            total_expense: Money::zero(),
            total_income: Money::zero(),
        }
    }
}
