use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

use crate::models::{Budget, Wallet, Saving};

#[derive(Debug, Clone, Serialize, Deserialize, Validate, FromRow)]
pub struct User {
    #[validate(length(min = 1, message = "Login cannot be empty"))]
    pub login: String,
    
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    
    #[serde(skip_serializing)]
    pub password: Option<String>,
    
    pub admin: bool,
}

// DTO for User responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDto {
    pub login: String,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    pub admin: bool,
}

// For user creation
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateUserDto {
    #[validate(length(min = 1, message = "Login cannot be empty"))]
    pub login: String,
    
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    
    pub admin: Option<bool>,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            login: user.login,
            first_name: user.first_name,
            last_name: user.last_name,
            admin: user.admin,
        }
    }
}

// Complete user with relations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserWithRelations {
    pub login: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub admin: bool,
    pub wallets: Vec<Wallet>,
    pub budgets: Vec<Budget>,
    pub savings: Vec<Saving>,
}
