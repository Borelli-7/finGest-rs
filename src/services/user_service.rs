use std::collections::HashMap;
use bigdecimal::BigDecimal;
use sqlx::{PgPool, Row};
use async_trait::async_trait;

use crate::models::{Category, Money};

use crate::{
    errors::AppError,
    models::{
        Budget, BudgetOutputDto, UpdateBudgetDto, DateRange, Expense, ExpenseInputDto, UpdateExpenseDto,
        Summary, User, UserDto, Wallet, WalletDto, UpdateWalletDto,
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
    async fn update_wallet(&self, login: &str, wallet_id: i32, update_data: UpdateWalletDto) -> Result<WalletDto, AppError>;
    async fn delete_wallet(&self, login: &str, wallet_id: i32) -> Result<(), AppError>;
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
    async fn update_expense(
        &self,
        login: &str,
        wallet_id: i32,
        expense_id: i32,
        update_data: UpdateExpenseDto,
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
        authenticated_login: &str,
        start: DateRange,
        end: DateRange,
    ) -> Result<Vec<BudgetOutputDto>, AppError>;
    async fn add_budget(&self, login: &str, budget: Budget) -> Result<Budget, AppError>;
    async fn update_budget(
        &self,
        login: &str,
        budget_id: i32,
        update_data: UpdateBudgetDto,
    ) -> Result<Budget, AppError>;
    async fn delete_budget(&self, login: &str, budget_id: i32) -> Result<(), AppError>;
}

pub struct UserService {
    pool: PgPool,
}

impl UserService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Helper method to check if a user exists
    async fn check_user_exists(&self, login: &str) -> Result<(), AppError> {
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

        Ok(())
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
            return Err(AppError::NotFoundError(format!("User with login '{}' not found", login)));
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
        // First check if the user exists
        self.check_user_exists(login).await?;

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
        self.check_user_exists(login).await?;

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

    async fn update_wallet(&self, login: &str, wallet_id: i32, update_data: UpdateWalletDto) -> Result<WalletDto, AppError> {
        // First check if the user exists
        self.check_user_exists(login).await?;

        // Check if the wallet exists and belongs to the user
        let existing_wallet = sqlx::query(
            "SELECT w.id, w.name, w.amount_amount, w.amount_currency
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
        .ok_or_else(|| AppError::AuthorizationError(
            "Not authorized to update this wallet or wallet not found".to_string()
        ))?;

        // Prepare the updated values (use existing values if not provided)
        let new_name = update_data.name.as_ref().unwrap_or(&existing_wallet.name);
        let new_amount = update_data.amount.as_ref().unwrap_or(&existing_wallet.amount);

        // Update the wallet
        sqlx::query(
            "UPDATE wallet 
            SET name = $1, amount_amount = $2, amount_currency = $3
            WHERE id = $4"
        )
        .bind(new_name)
        .bind(&new_amount.amount)
        .bind(&new_amount.currency)
        .bind(wallet_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Return the updated wallet data
        Ok(WalletDto {
            id: Some(wallet_id),
            name: new_name.clone(),
            amount: new_amount.clone(),
        })
    }

    async fn delete_wallet(&self, login: &str, wallet_id: i32) -> Result<(), AppError> {
        // First check if the user exists
        self.check_user_exists(login).await?;

        // Check if the wallet exists and belongs to the user
        let wallet_belongs_to_user = sqlx::query(
            "SELECT 1 FROM account_wallet 
            WHERE account_login = $1 AND wallet_id = $2"
        )
        .bind(login)
        .bind(wallet_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if wallet_belongs_to_user.is_none() {
            // Check if wallet exists at all to provide appropriate error
            let wallet_exists = sqlx::query(
                "SELECT 1 FROM wallet WHERE id = $1"
            )
            .bind(wallet_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

            if wallet_exists.is_some() {
                // Wallet exists but doesn't belong to this user
                return Err(AppError::AuthorizationError(
                    "Not authorized to delete this wallet".to_string()
                ));
            } else {
                // Wallet doesn't exist
                return Err(AppError::NotFoundError(
                    format!("Wallet with id {} not found", wallet_id)
                ));
            }
        }

        // Start a transaction to ensure data consistency
        let mut tx = self.pool.begin().await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Delete all expenses associated with the wallet first
        sqlx::query(
            "DELETE FROM expense WHERE wallet_id = $1"
        )
        .bind(wallet_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Delete the account_wallet association
        sqlx::query(
            "DELETE FROM account_wallet WHERE wallet_id = $1"
        )
        .bind(wallet_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Delete the wallet itself
        let result = sqlx::query(
            "DELETE FROM wallet WHERE id = $1"
        )
        .bind(wallet_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Verify deletion was successful
        if result.rows_affected() == 0 {
            return Err(AppError::DatabaseError(
                "Failed to delete wallet".to_string()
            ));
        }

        // Commit the transaction
        tx.commit().await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn get_summary(&self, login: &str, wallet_id: i32, _date_range: DateRange) -> Result<Summary, AppError> {
        // First check if the user exists
        self.check_user_exists(login).await?;

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
        // First check if the user exists
        self.check_user_exists(login).await?;

        // Then check if the wallet belongs to the user
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
        // First check if the user exists
        self.check_user_exists(login).await?;

        // Then check if the wallet belongs to the user
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

    async fn update_expense(&self, login: &str, wallet_id: i32, expense_id: i32, update_data: UpdateExpenseDto) -> Result<Expense, AppError> {
        // First check if the user exists
        self.check_user_exists(login).await?;

        // Check if the wallet exists and belongs to the user
        let wallet_belongs_to_user = sqlx::query(
            "SELECT 1 FROM account_wallet 
            WHERE account_login = $1 AND wallet_id = $2"
        )
        .bind(login)
        .bind(wallet_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if wallet_belongs_to_user.is_none() {
            // Check if wallet exists at all to provide appropriate error
            let wallet_exists = sqlx::query(
                "SELECT 1 FROM wallet WHERE id = $1"
            )
            .bind(wallet_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

            if wallet_exists.is_some() {
                // Wallet exists but doesn't belong to this user
                return Err(AppError::AuthorizationError(
                    "Not authorized to update expenses in this wallet".to_string()
                ));
            } else {
                // Wallet doesn't exist
                return Err(AppError::NotFoundError(
                    format!("Wallet with id {} not found", wallet_id)
                ));
            }
        }

        // Check if the expense exists and belongs to the wallet
        let existing_expense = sqlx::query(
            "SELECT 
                id, 
                amount_amount, 
                amount_currency, 
                date, 
                description, 
                category_name, 
                category_profit 
            FROM expense 
            WHERE id = $1 AND wallet_id = $2"
        )
        .bind(expense_id)
        .bind(wallet_id)
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
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let existing_expense = match existing_expense {
            Some(expense) => expense,
            None => {
                // Check if expense exists at all to provide appropriate error
                let expense_exists = sqlx::query(
                    "SELECT 1 FROM expense WHERE id = $1"
                )
                .bind(expense_id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;

                if expense_exists.is_some() {
                    // Expense exists but doesn't belong to this wallet (hence not to this user)
                    return Err(AppError::AuthorizationError(
                        "Not authorized to update this expense".to_string()
                    ));
                } else {
                    // Expense doesn't exist
                    return Err(AppError::NotFoundError(
                        format!("Expense with id {} not found", expense_id)
                    ));
                }
            }
        };

        // Prepare the updated values (use existing values if not provided)
        let new_amount = update_data.amount.as_ref().unwrap_or(&existing_expense.amount);
        let new_date = update_data.date.unwrap_or(existing_expense.date);
        let new_description = update_data.description.as_ref().unwrap_or(&existing_expense.description);
        let new_category = update_data.category.as_ref().unwrap_or(&existing_expense.category);

        // If category is being updated, validate it exists
        if update_data.category.is_some() {
            let category_exists = sqlx::query(
                "SELECT 1 FROM category 
                WHERE name = $1 AND profit = $2"
            )
            .bind(&new_category.name)
            .bind(new_category.profit)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .is_some();

            if !category_exists {
                return Err(AppError::BadRequestError(
                    format!("Category {} with profit={} does not exist", new_category.name, new_category.profit)
                ));
            }
        }

        // Calculate the difference in amount for wallet balance update
        let old_impact = if existing_expense.category.profit {
            existing_expense.amount.amount.clone()
        } else {
            -existing_expense.amount.amount.clone()
        };

        let new_impact = if new_category.profit {
            new_amount.amount.clone()
        } else {
            -new_amount.amount.clone()
        };

        // Update the expense
        let updated_expense = sqlx::query(
            "UPDATE expense 
            SET amount_amount = $1, 
                amount_currency = $2, 
                date = TO_DATE($3, 'YYYY-MM-DD'), 
                description = $4, 
                category_name = $5, 
                category_profit = $6 
            WHERE id = $7 
            RETURNING id, amount_amount, amount_currency, date, description, category_name, category_profit"
        )
        .bind(&new_amount.amount)
        .bind(&new_amount.currency)
        .bind(new_date.to_string())
        .bind(new_description)
        .bind(&new_category.name)
        .bind(new_category.profit)
        .bind(expense_id)
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

        // Update wallet balance if amount or category profit changed
        // Only update if the currency matches to avoid complex currency conversion
        if existing_expense.amount.currency == new_amount.currency {
            let balance_adjustment = new_impact - old_impact;
            if balance_adjustment != BigDecimal::from(0) {
                sqlx::query(
                    "UPDATE wallet 
                    SET amount_amount = amount_amount + $1 
                    WHERE id = $2 AND amount_currency = $3"
                )
                .bind(&balance_adjustment)
                .bind(wallet_id)
                .bind(&new_amount.currency)
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;
            }
        }

        Ok(updated_expense)
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

    async fn get_budgets(
        &self,
        login: &str,
        authenticated_login: &str,
        start_range: DateRange,
        end_range: DateRange,
    ) -> Result<Vec<BudgetOutputDto>, AppError> {
        // First, check if the target user exists
        self.check_user_exists(login).await?;
        
        // Verify the authenticated user is accessing their own budgets
        // Authorization check: user can only access their own budgets
        if login != authenticated_login {
            return Err(AppError::AuthorizationError(
                "Not authorized to access budgets for this user".to_string()
            ));
        }
        
        // Build the query with optional date range filters
        // Fetch budgets that belong to the authenticated user and fall within the date ranges
        let budgets = sqlx::query(
            r#"
            SELECT 
                b.id,
                b.category_name,
                b.category_profit,
                b.total_amount,
                b.total_currency,
                b.start_date,
                b.end_date,
                COALESCE(SUM(
                    CASE 
                        WHEN e.category_profit = false AND e.date >= b.start_date AND e.date <= b.end_date 
                        THEN e.amount_amount 
                        ELSE 0 
                    END
                ), 0) as spent_amount
            FROM budget b
            LEFT JOIN account_wallet aw ON aw.account_login = b.account_login
            LEFT JOIN expense e ON e.wallet_id = aw.wallet_id AND e.category_name = b.category_name
            WHERE b.account_login = $1
                AND ($2::date IS NULL OR b.start_date >= $2::date)
                AND ($3::date IS NULL OR b.start_date <= $3::date)
                AND ($4::date IS NULL OR b.end_date >= $4::date)
                AND ($5::date IS NULL OR b.end_date <= $5::date)
            GROUP BY b.id, b.category_name, b.category_profit, b.total_amount, b.total_currency, b.start_date, b.end_date
            ORDER BY b.start_date DESC
            "#
        )
        .bind(login)
        .bind(start_range.start)
        .bind(start_range.end)
        .bind(end_range.start)
        .bind(end_range.end)
        .map(|row: sqlx::postgres::PgRow| {
            let total_amount: BigDecimal = row.get("total_amount");
            let spent_amount: BigDecimal = row.get("spent_amount");
            let left_amount = &total_amount - &spent_amount;
            let currency: Option<String> = row.get("total_currency");
            let currency_str = currency.unwrap_or_else(|| "PLN".to_string());
            
            BudgetOutputDto {
                id: row.get("id"),
                category: Category {
                    name: row.get("category_name"),
                    profit: row.get("category_profit"),
                },
                total: Money {
                    amount: total_amount,
                    currency: currency_str.clone(),
                },
                date_range: DateRange::new(
                    row.get("start_date"),
                    row.get("end_date"),
                ),
                spent: Money {
                    amount: spent_amount,
                    currency: currency_str.clone(),
                },
                left: Money {
                    amount: left_amount,
                    currency: currency_str,
                },
            }
        })
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        Ok(budgets)
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

    async fn update_budget(
        &self,
        login: &str,
        budget_id: i32,
        update_data: UpdateBudgetDto,
    ) -> Result<Budget, AppError> {
        // First, check if the user exists
        self.check_user_exists(login).await?;

        // Check if the budget exists and get its current data
        let existing_budget = sqlx::query(
            "SELECT 
                id, 
                category_name, 
                category_profit, 
                total_amount, 
                total_currency, 
                start_date, 
                end_date,
                account_login
            FROM budget 
            WHERE id = $1"
        )
        .bind(budget_id)
        .map(|row: sqlx::postgres::PgRow| {
            (Budget {
                id: row.get("id"),
                category: Category {
                    name: row.get("category_name"),
                    profit: row.get("category_profit"),
                },
                total: Money {
                    amount: row.get("total_amount"),
                    currency: row.get("total_currency"),
                },
                date_range: DateRange::new(
                    row.get("start_date"),
                    row.get("end_date"),
                ),
            }, row.get::<String, _>("account_login"))
        })
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let (existing_budget, owner_login) = match existing_budget {
            Some(data) => data,
            None => {
                return Err(AppError::NotFoundError(
                    format!("Budget with id {} not found", budget_id)
                ));
            }
        };

        // Check if the authenticated user is the owner of the budget
        if owner_login != login {
            return Err(AppError::AuthorizationError(
                "Not authorized to update this budget".to_string()
            ));
        }

        // Prepare the updated values (use existing values if not provided)
        let new_category = update_data.category.as_ref().unwrap_or(&existing_budget.category);
        let new_total = update_data.total.as_ref().unwrap_or(&existing_budget.total);
        let new_date_range = update_data.date_range.as_ref().unwrap_or(&existing_budget.date_range);

        // Validate that the new category exists if it's being updated
        if update_data.category.is_some() {
            let category_exists = sqlx::query(
                "SELECT 1 FROM category 
                WHERE name = $1 AND profit = $2"
            )
            .bind(&new_category.name)
            .bind(new_category.profit)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .is_some();

            if !category_exists {
                return Err(AppError::BadRequestError(
                    format!("Category {} with profit={} does not exist", new_category.name, new_category.profit)
                ));
            }
        }

        // Update the budget in the database
        let updated_budget = sqlx::query(
            "UPDATE budget 
            SET category_name = $1, 
                category_profit = $2, 
                total_amount = $3, 
                total_currency = $4, 
                start_date = $5, 
                end_date = $6 
            WHERE id = $7 
            RETURNING id, category_name, category_profit, total_amount, total_currency, start_date, end_date"
        )
        .bind(&new_category.name)
        .bind(new_category.profit)
        .bind(&new_total.amount)
        .bind(&new_total.currency)
        .bind(new_date_range.start)
        .bind(new_date_range.end)
        .bind(budget_id)
        .map(|row: sqlx::postgres::PgRow| {
            Budget {
                id: row.get("id"),
                category: Category {
                    name: row.get("category_name"),
                    profit: row.get("category_profit"),
                },
                total: Money {
                    amount: row.get("total_amount"),
                    currency: row.get("total_currency"),
                },
                date_range: DateRange::new(
                    row.get("start_date"),
                    row.get("end_date"),
                ),
            }
        })
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(updated_budget)
    }

    async fn delete_budget(&self, login: &str, budget_id: i32) -> Result<(), AppError> {
        // First, check if the user exists
        self.check_user_exists(login).await?;

        // Check if the budget exists and get its owner
        let budget_owner = sqlx::query(
            "SELECT account_login FROM budget WHERE id = $1"
        )
        .bind(budget_id)
        .map(|row: sqlx::postgres::PgRow| row.get::<String, _>("account_login"))
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let budget_owner = match budget_owner {
            Some(owner) => owner,
            None => {
                return Err(AppError::NotFoundError(
                    format!("Budget with id {} not found", budget_id)
                ));
            }
        };

        // Check if the budget belongs to the user
        if budget_owner != login {
            return Err(AppError::AuthorizationError(
                "Not authorized to delete this budget".to_string()
            ));
        }

        // Delete the budget
        sqlx::query("DELETE FROM budget WHERE id = $1")
            .bind(budget_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
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
            async fn update_wallet(&self, login: &str, wallet_id: i32, update_data: UpdateWalletDto) -> Result<WalletDto, AppError>;
            async fn delete_wallet(&self, login: &str, wallet_id: i32) -> Result<(), AppError>;
            async fn get_summary(&self, login: &str, wallet_id: i32, date_range: DateRange) -> Result<Summary, AppError>;
            async fn get_expenses(&self, login: &str, wallet_id: i32, date_range: DateRange) -> Result<Vec<Expense>, AppError>;
            async fn get_highest_expense(&self, login: &str, wallet_id: i32, date_range: DateRange) -> Result<Option<Expense>, AppError>;
            async fn add_expense(&self, login: &str, wallet_id: i32, expense: ExpenseInputDto) -> Result<Expense, AppError>;
            async fn update_expense(&self, login: &str, wallet_id: i32, expense_id: i32, update_data: UpdateExpenseDto) -> Result<Expense, AppError>;
            async fn delete_expense(&self, login: &str, wallet_id: i32, expense_id: i32) -> Result<(), AppError>;
            async fn get_counted_categories(&self, login: &str, wallet_id: i32, date_range: DateRange) -> Result<HashMap<String, BigDecimal>, AppError>;
            async fn get_budgets(&self, login: &str, authenticated_login: &str, start: DateRange, end: DateRange) -> Result<Vec<BudgetOutputDto>, AppError>;
            async fn add_budget(&self, login: &str, budget: Budget) -> Result<Budget, AppError>;
            async fn update_budget(&self, login: &str, budget_id: i32, update_data: UpdateBudgetDto) -> Result<Budget, AppError>;
            async fn delete_budget(&self, login: &str, budget_id: i32) -> Result<(), AppError>;
        }
    }
}
