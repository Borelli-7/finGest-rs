use async_trait::async_trait;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;

use crate::errors::AppError;
use crate::models::{CreateUserDto, User, UserDto};

/// JWT Claims structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // Subject (user login)
    pub admin: bool,      // Admin flag
    pub exp: i64,         // Expiration time
    pub iat: i64,         // Issued at
}

/// Login request DTO
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LoginDto {
    #[validate(length(min = 1, message = "Login cannot be empty"))]
    pub login: String,
    
    #[validate(length(min = 1, message = "Password cannot be empty"))]
    pub password: String,
}

/// Login response DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserDto,
}

/// Token validation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenValidation {
    pub valid: bool,
    pub login: Option<String>,
    pub admin: Option<bool>,
}

#[async_trait]
pub trait AuthServiceTrait: Send + Sync {
    /// Register a new user
    async fn register(&self, user_dto: CreateUserDto) -> Result<UserDto, AppError>;
    
    /// Login a user and return a JWT token
    async fn login(&self, login_dto: LoginDto) -> Result<LoginResponse, AppError>;
    
    /// Verify a JWT token and return the claims
    async fn verify_token(&self, token: &str) -> Result<Claims, AppError>;
    
    /// Generate a JWT token for a user
    fn generate_token(&self, user: &User) -> Result<String, AppError>;
    
    /// Hash a password
    fn hash_password(&self, password: &str) -> Result<String, AppError>;
    
    /// Verify a password against a hash
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, AppError>;
}

pub struct AuthService {
    pool: PgPool,
    jwt_secret: String,
    jwt_expiration_hours: i64,
}

impl AuthService {
    pub fn new(pool: PgPool, jwt_secret: String) -> Self {
        Self {
            pool,
            jwt_secret,
            jwt_expiration_hours: 24, // Default 24 hours
        }
    }
    
    pub fn with_expiration(mut self, hours: i64) -> Self {
        self.jwt_expiration_hours = hours;
        self
    }
}

#[async_trait]
impl AuthServiceTrait for AuthService {
    async fn register(&self, user_dto: CreateUserDto) -> Result<UserDto, AppError> {
        // Validate input
        user_dto
            .validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;
        
        // Check if user already exists
        let existing_user = sqlx::query_as::<_, User>(
            "SELECT login, first_name, last_name, password, admin FROM account WHERE login = $1"
        )
        .bind(&user_dto.login)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        if existing_user.is_some() {
            return Err(AppError::BadRequestError(format!(
                "User with login '{}' already exists",
                user_dto.login
            )));
        }
        
        // Hash the password
        let hashed_password = self.hash_password(&user_dto.password)?;
        
        // Insert the new user
        let admin = user_dto.admin.unwrap_or(false);
        
        sqlx::query(
            "INSERT INTO account (login, first_name, last_name, password, admin) 
             VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(&user_dto.login)
        .bind(&user_dto.first_name)
        .bind(&user_dto.last_name)
        .bind(&hashed_password)
        .bind(admin)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        // Return the created user (without password)
        Ok(UserDto {
            login: user_dto.login,
            first_name: user_dto.first_name,
            last_name: user_dto.last_name,
            admin,
        })
    }
    
    async fn login(&self, login_dto: LoginDto) -> Result<LoginResponse, AppError> {
        // Validate input
        login_dto
            .validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;
        
        // Fetch user from database
        let user = sqlx::query_as::<_, User>(
            "SELECT login, first_name, last_name, password, admin FROM account WHERE login = $1"
        )
        .bind(&login_dto.login)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::AuthenticationError("Invalid credentials".to_string()))?;
        
        // Verify password
        let password_hash = user.password.as_ref().ok_or_else(|| {
            AppError::AuthenticationError("User has no password set".to_string())
        })?;
        
        let is_valid = self.verify_password(&login_dto.password, password_hash)?;
        
        if !is_valid {
            return Err(AppError::AuthenticationError("Invalid credentials".to_string()));
        }
        
        // Generate JWT token
        let token = self.generate_token(&user)?;
        
        Ok(LoginResponse {
            token,
            user: UserDto::from(user),
        })
    }
    
    async fn verify_token(&self, token: &str) -> Result<Claims, AppError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| AppError::AuthenticationError(format!("Invalid token: {}", e)))?;
        
        Ok(token_data.claims)
    }
    
    fn generate_token(&self, user: &User) -> Result<String, AppError> {
        let now = Utc::now();
        let expiration = now + Duration::hours(self.jwt_expiration_hours);
        
        let claims = Claims {
            sub: user.login.clone(),
            admin: user.admin,
            exp: expiration.timestamp(),
            iat: now.timestamp(),
        };
        
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::InternalServerError(format!("Failed to generate token: {}", e)))
    }
    
    fn hash_password(&self, password: &str) -> Result<String, AppError> {
        hash(password, DEFAULT_COST)
            .map_err(|e| AppError::InternalServerError(format!("Failed to hash password: {}", e)))
    }
    
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, AppError> {
        verify(password, hash)
            .map_err(|e| AppError::InternalServerError(format!("Failed to verify password: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;

    mock! {
        pub AuthService {}

        #[async_trait]
        impl AuthServiceTrait for AuthService {
            async fn register(&self, user_dto: CreateUserDto) -> Result<UserDto, AppError>;
            async fn login(&self, login_dto: LoginDto) -> Result<LoginResponse, AppError>;
            async fn verify_token(&self, token: &str) -> Result<Claims, AppError>;
            fn generate_token(&self, user: &User) -> Result<String, AppError>;
            fn hash_password(&self, password: &str) -> Result<String, AppError>;
            fn verify_password(&self, password: &str, hash: &str) -> Result<bool, AppError>;
        }
    }
    
    #[test]
    fn test_hash_and_verify_password() {
        // This is a simple integration test for password hashing
        let password = "SecurePassword123!";
        let hashed = hash(password, DEFAULT_COST).unwrap();
        
        assert!(verify(password, &hashed).unwrap());
        assert!(!verify("WrongPassword", &hashed).unwrap());
    }
}

#[cfg(test)]
pub use tests::MockAuthService;
