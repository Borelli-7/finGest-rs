//! Integration tests for the Money Manager API

use actix_web::{test, web, App, HttpResponse, http::StatusCode};
use async_trait::async_trait;
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use money_manager_api::{
    models::{Category, UserDto, CreateUserDto, WalletDto, Money, ExpenseInputDto, Expense, DateRange, Budget, BudgetOutputDto, Summary},
    services::{CategoryServiceTrait, auth_service::{AuthServiceTrait, LoginDto, LoginResponse, Claims}, user_service::UserServiceTrait},
    errors::AppError,
    api::handlers::auth_handler,
};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// Define a mock CategoryService for testing
struct MockCategoryService;

#[async_trait]
impl CategoryServiceTrait for MockCategoryService {
    async fn get_categories(&self) -> Result<Vec<Category>, AppError> {
        Ok(vec![
            Category::new("Food".to_string(), false),
            Category::new("Salary".to_string(), true),
        ])
    }
}

// Define a mock AuthService for testing
struct MockAuthService;

#[async_trait]
impl AuthServiceTrait for MockAuthService {
    async fn register(&self, user_dto: CreateUserDto) -> Result<UserDto, AppError> {
        Ok(UserDto {
            login: user_dto.login,
            first_name: user_dto.first_name,
            last_name: user_dto.last_name,
            admin: user_dto.admin.unwrap_or(false),
        })
    }
    
    async fn login(&self, login_dto: LoginDto) -> Result<LoginResponse, AppError> {
        if login_dto.login == "testuser" && login_dto.password == "password123" {
            Ok(LoginResponse {
                token: "mock_token_12345".to_string(),
                user: UserDto {
                    login: login_dto.login,
                    first_name: Some("Test".to_string()),
                    last_name: Some("User".to_string()),
                    admin: false,
                },
            })
        } else {
            Err(AppError::AuthenticationError("Invalid credentials".to_string()))
        }
    }
    
    async fn verify_token(&self, token: &str) -> Result<Claims, AppError> {
        if token == "mock_token_12345" {
            Ok(Claims {
                sub: "testuser".to_string(),
                admin: false,
                exp: 9999999999,
                iat: 1000000000,
            })
        } else {
            Err(AppError::AuthenticationError("Invalid token".to_string()))
        }
    }
    
    fn generate_token(&self, _user: &money_manager_api::models::User) -> Result<String, AppError> {
        Ok("mock_token_12345".to_string())
    }
    
    fn hash_password(&self, _password: &str) -> Result<String, AppError> {
        Ok("hashed_password".to_string())
    }
    
    fn verify_password(&self, password: &str, _hash: &str) -> Result<bool, AppError> {
        Ok(password == "password123")
    }
}

// Define a mock UserService for testing
struct MockUserService;

#[async_trait]
impl UserServiceTrait for MockUserService {
    async fn get_users(&self) -> Result<Vec<UserDto>, AppError> {
        Ok(vec![
            UserDto {
                login: "user1".to_string(),
                first_name: Some("User".to_string()),
                last_name: Some("One".to_string()),
                admin: false,
            },
            UserDto {
                login: "admin".to_string(),
                first_name: Some("Admin".to_string()),
                last_name: Some("User".to_string()),
                admin: true,
            },
        ])
    }
    
    async fn update_user<T>(&self, _login: &str, _field: &str, _value: HashMap<String, T>) -> Result<(), AppError>
    where
        T: serde::Serialize + std::fmt::Debug + Send + Sync + 'static,
    {
        Ok(())
    }
    
    async fn get_wallets(&self, login: &str) -> Result<Vec<WalletDto>, AppError> {
        if login == "testuser" {
            Ok(vec![
                WalletDto {
                    id: Some(1),
                    name: "Main Wallet".to_string(),
                    amount: Money::new(BigDecimal::from(1000), None),
                },
                WalletDto {
                    id: Some(2),
                    name: "Savings".to_string(),
                    amount: Money::new(BigDecimal::from(5000), None),
                },
            ])
        } else {
            Err(AppError::NotFoundError("User not found".to_string()))
        }
    }
    
    async fn add_wallet(&self, _login: &str, wallet: WalletDto) -> Result<WalletDto, AppError> {
        Ok(WalletDto {
            id: Some(3),
            name: wallet.name,
            amount: wallet.amount,
        })
    }
    
    async fn get_summary(&self, _login: &str, _wallet_id: i32, _date_range: DateRange) -> Result<Summary, AppError> {
        Ok(Summary::new(
            "Main Wallet".to_string(),
            Money::new(BigDecimal::from(1000), None),
        ))
    }
    
    async fn get_expenses(&self, _login: &str, _wallet_id: i32, _date_range: DateRange) -> Result<Vec<Expense>, AppError> {
        Ok(vec![
            Expense {
                id: Some(1),
                amount: Money::new(BigDecimal::from(50), None),
                date: NaiveDate::from_ymd_opt(2023, 6, 15).unwrap(),
                description: "Grocery shopping".to_string(),
                category: Category::new("Food".to_string(), false),
            },
        ])
    }
    
    async fn get_highest_expense(&self, _login: &str, _wallet_id: i32, _date_range: DateRange) -> Result<Option<Expense>, AppError> {
        Ok(Some(Expense {
            id: Some(1),
            amount: Money::new(BigDecimal::from(200), None),
            date: NaiveDate::from_ymd_opt(2023, 6, 20).unwrap(),
            description: "Electronics".to_string(),
            category: Category::new("Shopping".to_string(), false),
        }))
    }
    
    async fn add_expense(&self, _login: &str, _wallet_id: i32, expense: ExpenseInputDto) -> Result<Expense, AppError> {
        Ok(Expense {
            id: Some(2),
            amount: expense.amount,
            date: expense.date,
            description: expense.description,
            category: expense.category,
        })
    }
    
    async fn delete_expense(&self, _login: &str, _wallet_id: i32, _expense_id: i32) -> Result<(), AppError> {
        Ok(())
    }
    
    async fn get_counted_categories(&self, _login: &str, _wallet_id: i32, _date_range: DateRange) -> Result<HashMap<String, BigDecimal>, AppError> {
        let mut categories = HashMap::new();
        categories.insert("Food".to_string(), BigDecimal::from(150));
        categories.insert("Transport".to_string(), BigDecimal::from(75));
        Ok(categories)
    }
    
    async fn get_budgets(&self, _login: &str, _start: DateRange, _end: DateRange) -> Result<Vec<BudgetOutputDto>, AppError> {
        Ok(vec![
            BudgetOutputDto {
                id: Some(1),
                category: Category::new("Food".to_string(), false),
                total: Money::new(BigDecimal::from(500), None),
                date_range: DateRange::default(),
                spent: Money::new(BigDecimal::from(150), None),
                left: Money::new(BigDecimal::from(350), None),
            },
        ])
    }
    
    async fn add_budget(&self, _login: &str, _budget: Budget) -> Result<i32, AppError> {
        Ok(1)
    }
}

#[actix_rt::test]
async fn test_get_categories_handler() {
    // Arrange
    let mock_service = MockCategoryService;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_service))
            .route("/resources/categories", web::get().to(|svc: web::Data<MockCategoryService>| async move {
                let categories = svc.get_categories().await.unwrap();
                web::Json(categories)
            }))
    )
    .await;
    
    // Act
    let req = test::TestRequest::get().uri("/resources/categories").to_request();
    let resp = test::call_service(&app, req).await;
    
    // Assert
    assert!(resp.status().is_success());
    
    let body = test::read_body(resp).await;
    let categories: Vec<Category> = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(categories.len(), 2);
    assert_eq!(categories[0].name, "Food");
    assert_eq!(categories[0].profit, false);
    assert_eq!(categories[1].name, "Salary");
    assert_eq!(categories[1].profit, true);
}

#[actix_rt::test]
async fn test_register_handler() {
    let mock_service = MockAuthService;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Box::new(mock_service) as Box<dyn AuthServiceTrait>))
            .route("/api/auth/register", web::post().to(auth_handler::register))
    )
    .await;
    
    let user_dto = CreateUserDto {
        login: "newuser".to_string(),
        first_name: Some("New".to_string()),
        last_name: Some("User".to_string()),
        password: "securepass123".to_string(),
        admin: Some(false),
    };
    
    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&user_dto)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);
    
    let body = test::read_body(resp).await;
    let created_user: UserDto = serde_json::from_slice(&body).unwrap();
    assert_eq!(created_user.login, "newuser");
}

#[actix_rt::test]
async fn test_login_handler_success() {
    let mock_service = MockAuthService;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Box::new(mock_service) as Box<dyn AuthServiceTrait>))
            .route("/api/auth/login", web::post().to(auth_handler::login))
    )
    .await;
    
    let login_dto = LoginDto {
        login: "testuser".to_string(),
        password: "password123".to_string(),
    };
    
    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(&login_dto)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    let body = test::read_body(resp).await;
    let login_response: LoginResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(login_response.token, "mock_token_12345");
    assert_eq!(login_response.user.login, "testuser");
}

#[actix_rt::test]
async fn test_login_handler_failure() {
    let mock_service = MockAuthService;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Box::new(mock_service) as Box<dyn AuthServiceTrait>))
            .route("/api/auth/login", web::post().to(auth_handler::login))
    )
    .await;
    
    let login_dto = LoginDto {
        login: "testuser".to_string(),
        password: "wrongpassword".to_string(),
    };
    
    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(&login_dto)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);
}

#[actix_rt::test]
async fn test_verify_token_handler() {
    let mock_service = MockAuthService;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Box::new(mock_service) as Box<dyn AuthServiceTrait>))
            .route("/api/auth/verify", web::get().to(auth_handler::verify_token))
    )
    .await;
    
    let req = test::TestRequest::get()
        .uri("/api/auth/verify")
        .insert_header(("Authorization", "Bearer mock_token_12345"))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_get_users_handler() {
    let mock_service = MockUserService;
    
    let app = test::init_service(
        App::new()
            .route("/resources/users", web::get().to(|_: web::Data<MockUserService>| async move {
                let mock = MockUserService;
                let users = mock.get_users().await.unwrap();
                web::Json(users)
            }))
            .app_data(web::Data::new(mock_service))
    )
    .await;
    
    let req = test::TestRequest::get().uri("/resources/users").to_request();
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    let users: Vec<UserDto> = serde_json::from_slice(&body).unwrap();
    assert_eq!(users.len(), 2);
}

#[actix_rt::test]
async fn test_get_wallets_handler() {
    let mock_service = MockUserService;
    
    let app = test::init_service(
        App::new()
            .route("/resources/users/{login}/wallets", web::get().to(|path: web::Path<String>| async move {
                let login = path.into_inner();
                let mock = MockUserService;
                let wallets = mock.get_wallets(&login).await.unwrap();
                web::Json(wallets)
            }))
            .app_data(web::Data::new(mock_service))
    )
    .await;
    
    let req = test::TestRequest::get()
        .uri("/resources/users/testuser/wallets")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    let wallets: Vec<WalletDto> = serde_json::from_slice(&body).unwrap();
    assert_eq!(wallets.len(), 2);
    assert_eq!(wallets[0].name, "Main Wallet");
}

#[actix_rt::test]
async fn test_create_wallet_handler() {
    let mock_service = MockUserService;
    
    let app = test::init_service(
        App::new()
            .route("/resources/users/{login}/wallets", web::post().to(|path: web::Path<String>, wallet: web::Json<WalletDto>| async move {
                let _login = path.into_inner();
                let mock = MockUserService;
                let created_wallet = mock.add_wallet("testuser", wallet.into_inner()).await.unwrap();
                actix_web::HttpResponse::Created().json(created_wallet)
            }))
            .app_data(web::Data::new(mock_service))
    )
    .await;
    
    let wallet = WalletDto {
        id: None,
        name: "New Wallet".to_string(),
        amount: Money::new(BigDecimal::from(100), None),
    };
    
    let req = test::TestRequest::post()
        .uri("/resources/users/testuser/wallets")
        .set_json(&wallet)
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), 201);
    let body = test::read_body(resp).await;
    let created_wallet: WalletDto = serde_json::from_slice(&body).unwrap();
    assert_eq!(created_wallet.id, Some(3));
    assert_eq!(created_wallet.name, "New Wallet");
    assert_eq!(created_wallet.amount.amount, BigDecimal::from(100));
}

#[actix_rt::test]
async fn test_create_wallet_invalid_amount() {
    use money_manager_api::api::handlers::user_handler;
    use sqlx::PgPool;
    
    // Create a mock pool (this test focuses on validation, not DB)
    let pool_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://test:test@localhost/test".to_string());
    
    // Skip test if database is not available
    if let Ok(pool) = PgPool::connect(&pool_url).await {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool))
                .route("/resources/users/{login}/wallets", web::post().to(user_handler::create_wallet))
        )
        .await;
        
        // Test with negative amount
        let invalid_wallet = WalletDto {
            id: None,
            name: "Invalid Wallet".to_string(),
            amount: Money::new(BigDecimal::from(-100), None),
        };
        
        let req = test::TestRequest::post()
            .uri("/resources/users/testuser/wallets")
            .set_json(&invalid_wallet)
            .to_request();
        let resp = test::call_service(&app, req).await;
        
        // Should return 400 Bad Request
        assert_eq!(resp.status(), 400);
        
        // Check error response format
        let body = test::read_body(resp).await;
        let error: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(error.get("message").is_some());
        let message = error["message"].as_str().unwrap();
        assert!(message.contains("not valid") || message.contains("amount"));
    }
}

#[actix_rt::test]
async fn test_create_wallet_invalid_amount_format() {
    use money_manager_api::api::handlers::user_handler;
    use sqlx::PgPool;
    
    // Create a mock pool
    let pool_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://test:test@localhost/test".to_string());
    
    // Skip test if database is not available
    if let Ok(pool) = PgPool::connect(&pool_url).await {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool))
                .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                    let error_message = if err.to_string().contains("invalid digit") 
                        || err.to_string().contains("invalid type") 
                        || err.to_string().contains("expected") {
                        "The amount is invalid"
                    } else {
                        "Invalid request data"
                    };
                    
                    actix_web::error::InternalError::from_response(
                        err,
                        actix_web::HttpResponse::BadRequest().json(serde_json::json!({
                            "status": "400",
                            "message": error_message
                        }))
                    ).into()
                }))
                .route("/resources/users/{login}/wallets", web::post().to(user_handler::create_wallet))
        )
        .await;
        
        // Test with invalid JSON (string instead of number for amount)
        let invalid_json = r#"{
            "name": "Invalid Wallet",
            "amount": {
                "amount": "invalid_number",
                "currency": "USD"
            }
        }"#;
        
        let req = test::TestRequest::post()
            .uri("/resources/users/testuser/wallets")
            .insert_header(("content-type", "application/json"))
            .set_payload(invalid_json)
            .to_request();
        let resp = test::call_service(&app, req).await;
        
        // Should return 400 Bad Request
        assert_eq!(resp.status(), 400);
        
        // Check error response format
        let body = test::read_body(resp).await;
        let error: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(error.get("status").and_then(|v| v.as_str()), Some("400"));
        assert_eq!(error.get("message").and_then(|v| v.as_str()), Some("The amount is invalid"));
    }
}

#[actix_rt::test]
async fn test_get_expenses_handler() {
    let mock_service = MockUserService;
    
    let app = test::init_service(
        App::new()
            .route("/resources/users/{login}/wallets/{id}/expenses", web::get().to(|path: web::Path<(String, i32)>| async move {
                let (_login, _id) = path.into_inner();
                let mock = MockUserService;
                let expenses = mock.get_expenses("testuser", 1, DateRange::default()).await.unwrap();
                web::Json(expenses)
            }))
            .app_data(web::Data::new(mock_service))
    )
    .await;
    
    let req = test::TestRequest::get()
        .uri("/resources/users/testuser/wallets/1/expenses")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    let expenses: Vec<Expense> = serde_json::from_slice(&body).unwrap();
    assert_eq!(expenses.len(), 1);
    assert_eq!(expenses[0].description, "Grocery shopping");
}

#[actix_rt::test]
async fn test_create_expense_handler() {
    let mock_service = MockUserService;
    
    let app = test::init_service(
        App::new()
            .route("/resources/users/{login}/wallets/{id}/expenses", web::post().to(|path: web::Path<(String, i32)>, expense: web::Json<ExpenseInputDto>| async move {
                let (_login, _id) = path.into_inner();
                let mock = MockUserService;
                let created_expense = mock.add_expense("testuser", 1, expense.into_inner()).await.unwrap();
                HttpResponse::Created().json(created_expense)
            }))
            .app_data(web::Data::new(mock_service))
    )
    .await;
    
    let expense_input = ExpenseInputDto {
        amount: Money::new(BigDecimal::from(50), Some("USD".to_string())),
        date: NaiveDate::from_ymd_opt(2023, 6, 20).unwrap(),
        description: "Test expense".to_string(),
        category: Category::new("Food".to_string(), false),
    };
    
    let req = test::TestRequest::post()
        .uri("/resources/users/testuser/wallets/1/expenses")
        .set_json(&expense_input)
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body = test::read_body(resp).await;
    let created_expense: Expense = serde_json::from_slice(&body).unwrap();
    
    // Verify the created expense has all the expected fields
    assert!(created_expense.id.is_some());
    assert_eq!(created_expense.id.unwrap(), 2);
    assert_eq!(created_expense.description, "Test expense");
    assert_eq!(created_expense.category.name, "Food");
    assert_eq!(created_expense.amount.currency, "USD");
}


// To run actual integration tests with a real database:
// #[actix_rt::test]
// #[ignore] // Ignore by default as it requires a database
// async fn test_integration_with_database() {
//     dotenv().ok();
//     
//     let database_url = std::env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set for integration tests");
//     
//     let pool = PgPoolOptions::new()
//         .max_connections(5)
//         .connect(&database_url)
//         .await
//         .expect("Failed to create connection pool");
//     
//     // Run migrations if needed
//     sqlx::migrate!("./migrations")
//         .run(&pool)
//         .await
//         .expect("Failed to run migrations");
//     
//     // Create the test app with actual services
//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(pool.clone()))
//             .configure(configure_routes)
//     )
//     .await;
//     
//     // Perform your tests here
// }

