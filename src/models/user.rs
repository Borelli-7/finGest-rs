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
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;
    
    #[test]
    fn test_user_validation_success() {
        let user = User {
            login: "testuser".to_string(),
            first_name: Some("Test".to_string()),
            last_name: Some("User".to_string()),
            password: Some("password123".to_string()),
            admin: false,
        };
        assert!(user.validate().is_ok());
    }
    
    #[test]
    fn test_user_validation_empty_login() {
        let user = User {
            login: "".to_string(),
            first_name: None,
            last_name: None,
            password: None,
            admin: false,
        };
        assert!(user.validate().is_err());
    }
    
    #[test]
    fn test_user_to_dto_conversion() {
        let user = User {
            login: "testuser".to_string(),
            first_name: Some("Test".to_string()),
            last_name: Some("User".to_string()),
            password: Some("password123".to_string()),
            admin: true,
        };
        let dto: UserDto = user.into();
        assert_eq!(dto.login, "testuser");
        assert_eq!(dto.first_name, Some("Test".to_string()));
        assert_eq!(dto.last_name, Some("User".to_string()));
        assert_eq!(dto.admin, true);
    }
    
    #[test]
    fn test_create_user_dto_validation_success() {
        let dto = CreateUserDto {
            login: "newuser".to_string(),
            first_name: Some("New".to_string()),
            last_name: Some("User".to_string()),
            password: "securepassword123".to_string(),
            admin: Some(false),
        };
        assert!(dto.validate().is_ok());
    }
    
    #[test]
    fn test_create_user_dto_validation_short_password() {
        let dto = CreateUserDto {
            login: "newuser".to_string(),
            first_name: None,
            last_name: None,
            password: "short".to_string(),
            admin: None,
        };
        assert!(dto.validate().is_err());
    }
    
    #[test]
    fn test_create_user_dto_validation_empty_login() {
        let dto = CreateUserDto {
            login: "".to_string(),
            first_name: None,
            last_name: None,
            password: "password123".to_string(),
            admin: None,
        };
        assert!(dto.validate().is_err());
    }
}
