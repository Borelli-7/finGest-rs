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

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::BigDecimal;
    
    #[test]
    fn test_summary_new() {
        let summary = Summary::new(
            "My Wallet".to_string(),
            Money::new(BigDecimal::from(1000), None),
        );
        assert_eq!(summary.wallet_name, "My Wallet");
        assert_eq!(summary.balance.amount, BigDecimal::from(1000));
        assert!(summary.expense_categories.is_empty());
        assert!(summary.income_categories.is_empty());
        assert_eq!(summary.total_expense.amount, BigDecimal::from(0));
        assert_eq!(summary.total_income.amount, BigDecimal::from(0));
    }
    
    #[test]
    fn test_summary_with_categories() {
        let mut summary = Summary::new(
            "Test Wallet".to_string(),
            Money::new(BigDecimal::from(500), None),
        );
        
        summary.expense_categories.insert(
            "Food".to_string(),
            Money::new(BigDecimal::from(200), None),
        );
        summary.income_categories.insert(
            "Salary".to_string(),
            Money::new(BigDecimal::from(3000), None),
        );
        
        assert_eq!(summary.expense_categories.len(), 1);
        assert_eq!(summary.income_categories.len(), 1);
        assert_eq!(
            summary.expense_categories.get("Food").unwrap().amount,
            BigDecimal::from(200)
        );
    }
}
