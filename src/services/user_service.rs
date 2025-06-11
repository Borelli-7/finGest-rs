use std::collections::HashMap;
use bigdecimal::BigDecimal;
use sqlx::{PgPool, Row};
use async_trait::async_trait;

use crate::{
    errors::AppError,
    models::{
        Budget, BudgetOutputDto, DateRange, Expense, ExpenseInputDto,
        Money, Summary, User, UserDto, Wallet, WalletDto,
    },
};

#[async_trait]
pub trait UserServiceTrait: Send + Sync {
    async fn get_users(&self) -> Result<Vec<UserDto>, AppError>;
    async fn update_user<T>(&self, login: &str, field: &str, value: HashMap<String, T>) -> Result<(), AppError>
    where
        T: serde::Serialize + std::fmt::Debug + Send + Sync + 'static;
    async fn get_wallets(&self, login: &str) -> Result<Vec<WalletDto>, AppError>;
    async fn add_wallet(&self, login: &str, wallet: WalletDto) -> Result<i32, AppError>;
    async fn get_summary(&self, login: &str, wallet_id: i32, date_range: DateRange) -> Result<Summary, AppError>;
    async fn get_expenses(
        &self,
        login: &str,
        wallet_id: i32,
        date_range: DateRange,
    ) -> Result<Vec<Expense>, AppError>;
    async fn get_highest_expense(
        &self,
        login: &str,
        wallet_id: i32,
        date_range: DateRange,
    ) -> Result<Option<Expense>, AppError>;
    async fn add_expense(
        &self,
        login: &str,
        wallet_id: i32,
        expense: ExpenseInputDto,
    ) -> Result<i32, AppError>;
    async fn delete_expense(&self, login: &str, wallet_id: i32, expense_id: i32) -> Result<(), AppError>;
    async fn get_counted_categories(
        &self,
        login: &str,
        wallet_id: i32,
        date_range: DateRange,
    ) -> Result<HashMap<String, BigDecimal>, AppError>;
    async fn get_budgets(
        &self,
        login: &str,
        start: DateRange,
        end: DateRange,
    ) -> Result<Vec<BudgetOutputDto>, AppError>;
    async fn add_budget(&self, login: &str, budget: Budget) -> Result<i32, AppError>;
}

pub struct UserService {
    pool: PgPool,
}

impl UserService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserServiceTrait for UserService {
    async fn get_users(&self) -> Result<Vec<UserDto>, AppError> {
        let users = sqlx::query("SELECT login, first_name, last_name, password, admin FROM account")
            .map(|row: sqlx::postgres::PgRow| {
                User {
                    login: row.get("login"),
                    first_name: row.get("first_name"),
                    last_name: row.get("last_name"),
                    password: row.get("password"),
                    admin: row.get("admin"),
                }
            })
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(users.into_iter().map(UserDto::from).collect())
    }

    async fn update_user<T>(&self, login: &str, field: &str, value: HashMap<String, T>) -> Result<(), AppError>
    where
        T: serde::Serialize + std::fmt::Debug + Send + Sync,
    {
        // This is a simplified implementation - in a real application, 
        // you would validate the field name and handle different types accordingly
        let value_json = serde_json::to_value(&value)
            .map_err(|e| AppError::BadRequestError(format!("Invalid value format: {}", e)))?;
        
        // For demonstration purposes, handle only simple fields
        match field {
            "firstName" => {
                if let Some(first_name) = value_json.get("firstName").and_then(|v| v.as_str()) {
                    sqlx::query("UPDATE account SET first_name = $1 WHERE login = $2")
                        .bind(first_name)
                        .bind(login)
                        .execute(&self.pool)
                        .await
                        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
                }
            }
            "lastName" => {
                if let Some(last_name) = value_json.get("lastName").and_then(|v| v.as_str()) {
                    sqlx::query("UPDATE account SET last_name = $1 WHERE login = $2")
                        .bind(last_name)
                        .bind(login)
                        .execute(&self.pool)
                        .await
                        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
                }
            }
            // Add other fields as needed
            _ => return Err(AppError::BadRequestError(format!("Invalid field: {}", field))),
        }

        Ok(())
    }

    async fn get_wallets(&self, login: &str) -> Result<Vec<WalletDto>, AppError> {
        // Implementation would use a join to fetch wallets for a specific user
        // For demonstration:
        let wallets = sqlx::query(
            "SELECT w.id, w.name, w.amount_amount as amount_amount, w.amount_currency as amount_currency
            FROM wallet w
            JOIN account_wallet aw ON w.id = aw.wallet_id
            WHERE aw.account_login = $1"
        )
        .bind(login)
        .map(|row: sqlx::postgres::PgRow| {
            Wallet {
                id: row.get("id"),
                name: row.get("name"),
                amount: Money {
                    amount: row.get("amount_amount"),
                    currency: row.get("amount_currency"),
                },
            }
        })
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(wallets.into_iter().map(WalletDto::from).collect())
    }

    // Additional method implementations would follow the same pattern
    // For brevity, I'm showing just a few of the key methods
    
    async fn add_wallet(&self, login: &str, wallet_dto: WalletDto) -> Result<i32, AppError> {
        // This would be implemented as a transaction to ensure data consistency
        let mut tx = self.pool.begin().await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        // Create the wallet
        let wallet_id = sqlx::query(
            "INSERT INTO wallet (name, amount_amount, amount_currency) VALUES ($1, $2, $3) RETURNING id"
        )
        .bind(&wallet_dto.name)
        .bind(&wallet_dto.amount.amount)
        .bind(&wallet_dto.amount.currency)
        .map(|row: sqlx::postgres::PgRow| row.get::<i32, _>("id"))
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        // Associate wallet with user
        sqlx::query(
            "INSERT INTO account_wallet (account_login, wallet_id) VALUES ($1, $2)"
        )
        .bind(login)
        .bind(wallet_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        tx.commit().await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        Ok(wallet_id)
    }

    async fn get_summary(&self, login: &str, wallet_id: i32, _date_range: DateRange) -> Result<Summary, AppError> {
        // In a real implementation, this would calculate expenses and income within the date range
        // For demonstration, returning a simplified placeholder
        let wallet = sqlx::query(
            "SELECT w.id, w.name, w.amount_amount as amount_amount, w.amount_currency as amount_currency
            FROM wallet w
            JOIN account_wallet aw ON w.id = aw.wallet_id
            WHERE aw.account_login = $1 AND w.id = $2"
        )
        .bind(login)
        .bind(wallet_id)
        .map(|row: sqlx::postgres::PgRow| {
            Wallet {
                id: row.get("id"),
                name: row.get("name"),
                amount: Money {
                    amount: row.get("amount_amount"),
                    currency: row.get("amount_currency"),
                },
            }
        })
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFoundError(format!("Wallet with id {} not found", wallet_id)))?;

        let summary = Summary::new(wallet.name.clone(), wallet.amount);
        
        // This would be replaced with actual database queries to populate the summary
        
        Ok(summary)
    }

    // Additional methods would be implemented similarly
    
    async fn get_expenses(&self, _login: &str, _wallet_id: i32, _date_range: DateRange) -> Result<Vec<Expense>, AppError> {
        // Implementation placeholder
        Ok(Vec::new())
    }

    async fn get_highest_expense(&self, _login: &str, _wallet_id: i32, _date_range: DateRange) -> Result<Option<Expense>, AppError> {
        // Implementation placeholder
        Ok(None)
    }

    async fn add_expense(&self, _login: &str, _wallet_id: i32, _expense: ExpenseInputDto) -> Result<i32, AppError> {
        // Implementation placeholder
        Ok(1)
    }

    async fn delete_expense(&self, _login: &str, _wallet_id: i32, _expense_id: i32) -> Result<(), AppError> {
        // Implementation placeholder
        Ok(())
    }

    async fn get_counted_categories(&self, _login: &str, _wallet_id: i32, _date_range: DateRange) -> Result<HashMap<String, BigDecimal>, AppError> {
        // Implementation placeholder
        Ok(HashMap::new())
    }

    async fn get_budgets(&self, _login: &str, _start: DateRange, _end: DateRange) -> Result<Vec<BudgetOutputDto>, AppError> {
        // Implementation placeholder
        Ok(Vec::new())
    }

    async fn add_budget(&self, _login: &str, _budget: Budget) -> Result<i32, AppError> {
        // Implementation placeholder
        Ok(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;

    mock! {
        pub UserService {}

        #[async_trait]
        impl UserServiceTrait for UserService {
            async fn get_users(&self) -> Result<Vec<UserDto>, AppError>;
            async fn update_user<T>(&self, login: &str, field: &str, value: HashMap<String, T>) -> Result<(), AppError>
            where
                T: serde::Serialize + std::fmt::Debug + Send + Sync + 'static;
            async fn get_wallets(&self, login: &str) -> Result<Vec<WalletDto>, AppError>;
            async fn add_wallet(&self, login: &str, wallet: WalletDto) -> Result<i32, AppError>;
            async fn get_summary(&self, login: &str, wallet_id: i32, date_range: DateRange) -> Result<Summary, AppError>;
            async fn get_expenses(&self, login: &str, wallet_id: i32, date_range: DateRange) -> Result<Vec<Expense>, AppError>;
            async fn get_highest_expense(&self, login: &str, wallet_id: i32, date_range: DateRange) -> Result<Option<Expense>, AppError>;
            async fn add_expense(&self, login: &str, wallet_id: i32, expense: ExpenseInputDto) -> Result<i32, AppError>;
            async fn delete_expense(&self, login: &str, wallet_id: i32, expense_id: i32) -> Result<(), AppError>;
            async fn get_counted_categories(&self, login: &str, wallet_id: i32, date_range: DateRange) -> Result<HashMap<String, BigDecimal>, AppError>;
            async fn get_budgets(&self, login: &str, start: DateRange, end: DateRange) -> Result<Vec<BudgetOutputDto>, AppError>;
            async fn add_budget(&self, login: &str, budget: Budget) -> Result<i32, AppError>;
        }
    }
}
