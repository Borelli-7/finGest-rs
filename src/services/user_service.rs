use std::collections::HashMap;
use bigdecimal::BigDecimal;
use sqlx::{PgPool, Row};
use async_trait::async_trait;

use crate::models::{Category, Money};

use crate::{
    errors::AppError,
    models::{
        Budget, BudgetOutputDto, DateRange, Expense, ExpenseInputDto,
        Summary, User, UserDto, Wallet, WalletDto,
    },
};

#[async_trait]
pub trait UserServiceTrait: Send + Sync {
    async fn get_users(&self) -> Result<Vec<UserDto>, AppError>;
    async fn update_user<T>(&self, login: &str, field: &str, value: HashMap<String, T>) -> Result<(), AppError>
    where
        T: serde::Serialize + std::fmt::Debug + Send + Sync + 'static;
    async fn delete_user(&self, login: &str) -> Result<(), AppError>;
    async fn get_wallets(&self, login: &str) -> Result<Vec<WalletDto>, AppError>;
    async fn add_wallet(&self, login: &str, wallet: WalletDto) -> Result<WalletDto, AppError>;
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
    ) -> Result<Expense, AppError>;
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
    async fn add_budget(&self, login: &str, budget: Budget) -> Result<Budget, AppError>;
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
        let rows_affected = match field {
            "firstName" => {
                if let Some(first_name) = value_json.get("firstName").and_then(|v| v.as_str()) {
                    let result = sqlx::query("UPDATE account SET first_name = $1 WHERE login = $2")
                        .bind(first_name)
                        .bind(login)
                        .execute(&self.pool)
                        .await
                        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
                    result.rows_affected()
                } else {
                    return Err(AppError::BadRequestError("firstName value is required".to_string()));
                }
            }
            "lastName" => {
                if let Some(last_name) = value_json.get("lastName").and_then(|v| v.as_str()) {
                    let result = sqlx::query("UPDATE account SET last_name = $1 WHERE login = $2")
                        .bind(last_name)
                        .bind(login)
                        .execute(&self.pool)
                        .await
                        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
                    result.rows_affected()
                } else {
                    return Err(AppError::BadRequestError("lastName value is required".to_string()));
                }
            }
            // Add other fields as needed
            _ => return Err(AppError::BadRequestError(format!("Invalid field: {}", field))),
        };

        // Check if any rows were affected by the update
        if rows_affected == 0 {
            return Err(AppError::NotFoundError(format!("user does not exist: {}", login)));
        }

        Ok(())
    }

    async fn delete_user(&self, login: &str) -> Result<(), AppError> {
        // Check if user exists first
        let user_exists = sqlx::query("SELECT 1 FROM account WHERE login = $1")
            .bind(login)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .is_some();

        if !user_exists {
            return Err(AppError::NotFoundError(format!("User `{}` does not exist", login)));
        }

        // Delete user (cascading deletes should handle related records)
        let result = sqlx::query("DELETE FROM account WHERE login = $1")
            .bind(login)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Double-check that the deletion was successful
        if result.rows_affected() == 0 {
            return Err(AppError::DatabaseError("Failed to delete user".to_string()));
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
    
    async fn add_wallet(&self, login: &str, wallet_dto: WalletDto) -> Result<WalletDto, AppError> {
        // First check if the user exists
        let user_exists = sqlx::query(
            "SELECT 1 FROM account WHERE login = $1"
        )
        .bind(login)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .is_some();

        if !user_exists {
            return Err(AppError::NotFoundError(format!("User with login '{}' not found", login)));
        }

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
        
        // Return the complete wallet data
        Ok(WalletDto {
            id: Some(wallet_id),
            name: wallet_dto.name,
            amount: wallet_dto.amount,
        })
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
    
    async fn get_expenses(&self, login: &str, wallet_id: i32, date_range: DateRange) -> Result<Vec<Expense>, AppError> {
        // First check if the wallet belongs to the user
        let belongs_to_user = sqlx::query(
            "SELECT 1 FROM account_wallet 
            WHERE account_login = $1 AND wallet_id = $2"
        )
        .bind(login)
        .bind(wallet_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .is_some();

        if !belongs_to_user {
            return Err(AppError::NotFoundError(format!("Wallet with id {} not found for user {}", wallet_id, login)));
        }

        // Fetch expenses for the wallet within the date range
        let expenses = sqlx::query(
            "SELECT 
                id, 
                amount_amount, 
                amount_currency, 
                date, 
                description, 
                category_name, 
                category_profit 
            FROM expense 
            WHERE wallet_id = $1 
            AND date >= TO_DATE($2, 'YYYY-MM-DD') 
            AND date <= TO_DATE($3, 'YYYY-MM-DD')"
        )
        .bind(wallet_id)
        .bind(date_range.start.to_string())
        .bind(date_range.end.to_string())
        .map(|row: sqlx::postgres::PgRow| {
            Expense {
                id: row.get("id"),
                amount: Money {
                    amount: row.get("amount_amount"),
                    currency: row.get("amount_currency"),
                },
                date: row.get("date"),
                description: row.get("description"),
                category: Category {
                    name: row.get("category_name"),
                    profit: row.get("category_profit"),
                },
            }
        })
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(expenses)
    }

    async fn get_highest_expense(&self, login: &str, wallet_id: i32, date_range: DateRange) -> Result<Option<Expense>, AppError> {
        // First check if the wallet belongs to the user
        let belongs_to_user = sqlx::query(
            "SELECT 1 FROM account_wallet 
            WHERE account_login = $1 AND wallet_id = $2"
        )
        .bind(login)
        .bind(wallet_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .is_some();

        if !belongs_to_user {
            return Err(AppError::NotFoundError(format!("Wallet with id {} not found for user {}", wallet_id, login)));
        }

        // Fetch the expense with the highest amount for the wallet within the date range
        // Note: This assumes non-profit expenses (i.e., actual expenses not income)
        let highest_expense_row = sqlx::query(
            "SELECT 
                id, 
                amount_amount, 
                amount_currency, 
                date, 
                description, 
                category_name, 
                category_profit 
            FROM expense 
            WHERE wallet_id = $1 
            AND date >= TO_DATE($2, 'YYYY-MM-DD') 
            AND date <= TO_DATE($3, 'YYYY-MM-DD') 
            AND category_profit = false 
            ORDER BY amount_amount DESC 
            LIMIT 1"
        )
        .bind(wallet_id)
        .bind(date_range.start.to_string())
        .bind(date_range.end.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        let highest_expense = highest_expense_row.map(|row| {
            Expense {
                id: row.get("id"),
                amount: Money {
                    amount: row.get("amount_amount"),
                    currency: row.get("amount_currency"),
                },
                date: row.get("date"),
                description: row.get("description"),
                category: Category {
                    name: row.get("category_name"),
                    profit: row.get("category_profit"),
                },
            }
        });

        Ok(highest_expense)
    }

    async fn add_expense(&self, login: &str, wallet_id: i32, expense: ExpenseInputDto) -> Result<Expense, AppError> {
        // First check if the wallet belongs to the user
        let belongs_to_user = sqlx::query(
            "SELECT 1 FROM account_wallet 
            WHERE account_login = $1 AND wallet_id = $2"
        )
        .bind(login)
        .bind(wallet_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .is_some();

        if !belongs_to_user {
            return Err(AppError::NotFoundError(format!("Wallet with id {} not found for user {}", wallet_id, login)));
        }

        // Validate that the category exists
        let category_exists = sqlx::query(
            "SELECT 1 FROM category 
            WHERE name = $1 AND profit = $2"
        )
        .bind(&expense.category.name)
        .bind(expense.category.profit)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .is_some();

        if !category_exists {
            return Err(AppError::BadRequestError(format!("Category {} with profit={} does not exist", expense.category.name, expense.category.profit)));
        }

        // Insert the expense and fetch the complete created expense
        let created_expense = sqlx::query(
            "INSERT INTO expense 
            (wallet_id, amount_amount, amount_currency, date, description, category_name, category_profit) 
            VALUES ($1, $2, $3, TO_DATE($4, 'YYYY-MM-DD'), $5, $6, $7) 
            RETURNING id, amount_amount, amount_currency, date, description, category_name, category_profit"
        )
        .bind(wallet_id)
        .bind(&expense.amount.amount)
        .bind(&expense.amount.currency)
        .bind(expense.date.to_string())
        .bind(&expense.description)
        .bind(&expense.category.name)
        .bind(expense.category.profit)
        .map(|row: sqlx::postgres::PgRow| {
            Expense {
                id: row.get("id"),
                amount: Money {
                    amount: row.get("amount_amount"),
                    currency: row.get("amount_currency"),
                },
                date: row.get("date"),
                description: row.get("description"),
                category: Category {
                    name: row.get("category_name"),
                    profit: row.get("category_profit"),
                },
            }
        })
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Update the wallet amount if needed
        // Note: This is a simplification. In a real app, you might want to update the wallet amount based on the expense type (profit or not)
        if !created_expense.category.profit {
            // Expense reduces the wallet amount
            sqlx::query(
                "UPDATE wallet 
                SET amount_amount = amount_amount - $1 
                WHERE id = $2 AND amount_currency = $3"
            )
            .bind(&created_expense.amount.amount)
            .bind(wallet_id)
            .bind(&created_expense.amount.currency)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        } else {
            // Income increases the wallet amount
            sqlx::query(
                "UPDATE wallet 
                SET amount_amount = amount_amount + $1 
                WHERE id = $2 AND amount_currency = $3"
            )
            .bind(&created_expense.amount.amount)
            .bind(wallet_id)
            .bind(&created_expense.amount.currency)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        Ok(created_expense)
    }

    async fn delete_expense(&self, login: &str, wallet_id: i32, expense_id: i32) -> Result<(), AppError> {
        // First verify the wallet belongs to the user
        let belongs_to_user = sqlx::query(
            "SELECT 1 FROM account_wallet 
            WHERE account_login = $1 AND wallet_id = $2"
        )
        .bind(login)
        .bind(wallet_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .is_some();

        if !belongs_to_user {
            return Err(AppError::NotFoundError(format!("Wallet with id {} not found for user {}", wallet_id, login)));
        }

        // Attempt to delete the expense and check if any rows were affected
        let result = sqlx::query(
            "DELETE FROM expense 
            WHERE id = $1 AND wallet_id = $2"
        )
        .bind(expense_id)
        .bind(wallet_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // If no rows were affected, the expense doesn't exist
        if result.rows_affected() == 0 {
            return Err(AppError::NotFoundError(format!("The expense with id {} does not exist", expense_id)));
        }

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

    async fn add_budget(&self, _login: &str, budget: Budget) -> Result<Budget, AppError> {
        // Implementation placeholder - in production, this would insert into database
        // and return the budget with the generated ID
        let created_budget = Budget {
            id: Some(1),
            ..budget
        };
        Ok(created_budget)
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
            async fn delete_user(&self, login: &str) -> Result<(), AppError>;
            async fn get_wallets(&self, login: &str) -> Result<Vec<WalletDto>, AppError>;
            async fn add_wallet(&self, login: &str, wallet: WalletDto) -> Result<WalletDto, AppError>;
            async fn get_summary(&self, login: &str, wallet_id: i32, date_range: DateRange) -> Result<Summary, AppError>;
            async fn get_expenses(&self, login: &str, wallet_id: i32, date_range: DateRange) -> Result<Vec<Expense>, AppError>;
            async fn get_highest_expense(&self, login: &str, wallet_id: i32, date_range: DateRange) -> Result<Option<Expense>, AppError>;
            async fn add_expense(&self, login: &str, wallet_id: i32, expense: ExpenseInputDto) -> Result<Expense, AppError>;
            async fn delete_expense(&self, login: &str, wallet_id: i32, expense_id: i32) -> Result<(), AppError>;
            async fn get_counted_categories(&self, login: &str, wallet_id: i32, date_range: DateRange) -> Result<HashMap<String, BigDecimal>, AppError>;
            async fn get_budgets(&self, login: &str, start: DateRange, end: DateRange) -> Result<Vec<BudgetOutputDto>, AppError>;
            async fn add_budget(&self, login: &str, budget: Budget) -> Result<Budget, AppError>;
        }
    }
}
