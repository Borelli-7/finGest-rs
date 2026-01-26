//! Integration tests for the Money Manager API

use actix_web::{test, web, App, HttpResponse, http::StatusCode, ResponseError};
use async_trait::async_trait;
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use money_manager_api::{
    models::{Category, CreateCategoryDto, UpdateCategoryDto, UserDto, CreateUserDto, WalletDto, UpdateWalletDto, Money, ExpenseInputDto, Expense, DateRange, Budget, BudgetOutputDto, Summary},
    services::{CategoryServiceTrait, auth_service::{AuthServiceTrait, LoginDto, LoginResponse, Claims}, user_service::UserServiceTrait},
    errors::AppError,
    api::handlers::auth_handler,
};
use std::collections::HashMap;

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

    async fn create_category(&self, dto: CreateCategoryDto) -> Result<Category, AppError> {
        // Check for duplicate simulation
        if dto.name.to_lowercase() == "food" && !dto.profit {
            return Err(AppError::ConflictError(
                format!("Category '{}' with profit={} already exists", dto.name, dto.profit)
            ));
        }
        
        Ok(Category::new(dto.name, dto.profit))
    }

    async fn update_category(&self, name: String, profit: bool, dto: UpdateCategoryDto) -> Result<Category, AppError> {
        // Simulate category not found
        if name == "NonExistent" {
            return Err(AppError::NotFoundError(
                format!("Category '{}' with profit={} not found", name, profit)
            ));
        }
        
        // Simulate conflict with existing category
        if dto.new_name.to_lowercase() == "transport" && !profit {
            return Err(AppError::ConflictError(
                format!("Category '{}' with profit={} already exists", dto.new_name, profit)
            ));
        }
        
        Ok(Category::new(dto.new_name, profit))
    }

    async fn delete_category(&self, name: String, profit: bool) -> Result<(), AppError> {
        // Simulate category not found
        if name == "NonExistent" || name == "NotFound" {
            return Err(AppError::NotFoundError(
                format!("Category '{}' with profit={} not found", name, profit)
            ));
        }
        
        // Simulate successful deletion for other categories
        Ok(())
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
    
    async fn delete_user(&self, login: &str) -> Result<(), AppError> {
        // Mock behavior: simulate successful deletion for known users, NotFoundError for unknown users
        if login == "testuser" || login == "user1" {
            Ok(())
        } else {
            Err(AppError::NotFoundError(format!("User `{}` does not exist", login)))
        }
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
    
    async fn add_wallet(&self, login: &str, wallet: WalletDto) -> Result<WalletDto, AppError> {
        // Simulate user validation
        if login != "testuser" {
            return Err(AppError::NotFoundError(format!("User with login '{}' not found", login)));
        }
        
        Ok(WalletDto {
            id: Some(3),
            name: wallet.name,
            amount: wallet.amount,
        })
    }
    
    async fn update_wallet(&self, login: &str, wallet_id: i32, update_data: UpdateWalletDto) -> Result<WalletDto, AppError> {
        // Simulate user validation
        if login != "testuser" {
            return Err(AppError::NotFoundError(format!("User with login '{}' not found", login)));
        }
        
        // Simulate wallet not found or not owned by user
        if wallet_id == 999 {
            return Err(AppError::AuthorizationError(
                "Not authorized to update this wallet or wallet not found".to_string()
            ));
        }
        
        // Get existing wallet (mock data)
        let existing = WalletDto {
            id: Some(wallet_id),
            name: "Main Wallet".to_string(),
            amount: Money::new(BigDecimal::from(1000), None),
        };
        
        // Apply updates
        let new_name = update_data.name.unwrap_or(existing.name);
        let new_amount = update_data.amount.unwrap_or(existing.amount);
        
        Ok(WalletDto {
            id: Some(wallet_id),
            name: new_name,
            amount: new_amount,
        })
    }

    async fn delete_wallet(&self, login: &str, wallet_id: i32) -> Result<(), AppError> {
        // Simulate user validation
        if login != "testuser" {
            return Err(AppError::NotFoundError(format!("User with login '{}' not found", login)));
        }
        
        // Simulate wallet not found (wallet 999 doesn't exist)
        if wallet_id == 999 {
            return Err(AppError::NotFoundError(
                format!("Wallet with id {} not found", wallet_id)
            ));
        }
        
        // Simulate not authorized (wallet 888 exists but belongs to another user)
        if wallet_id == 888 {
            return Err(AppError::AuthorizationError(
                "Not authorized to delete this wallet".to_string()
            ));
        }
        
        // Wallets 1 and 2 belong to testuser and can be deleted
        Ok(())
    }
    
    async fn get_summary(&self, login: &str, _wallet_id: i32, _date_range: DateRange) -> Result<Summary, AppError> {
        // Simulate user validation
        if login != "testuser" {
            return Err(AppError::NotFoundError(format!("User with login '{}' not found", login)));
        }
        
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
    
    async fn delete_expense(&self, _login: &str, wallet_id: i32, expense_id: i32) -> Result<(), AppError> {
        // Mock behavior: simulate NotFoundError for non-existent expenses
        // Assume expense ID 1 exists in wallet 1
        if wallet_id == 1 && expense_id == 1 {
            Ok(())
        } else {
            Err(AppError::NotFoundError(format!("The expense with id {} does not exist", expense_id)))
        }
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
    
    async fn add_budget(&self, _login: &str, budget: Budget) -> Result<Budget, AppError> {
        Ok(Budget {
            id: Some(1),
            ..budget
        })
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
async fn test_create_category_handler_success() {
    // Arrange
    let mock_service = MockCategoryService;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_service))
            .route("/resources/categories", web::post().to(|svc: web::Data<MockCategoryService>, body: web::Json<CreateCategoryDto>| async move {
                match svc.create_category(body.into_inner()).await {
                    Ok(category) => HttpResponse::Created().json(category),
                    Err(e) => e.error_response(),
                }
            }))
    )
    .await;
    
    // Act
    let dto = CreateCategoryDto {
        name: "Transport".to_string(),
        profit: false,
    };
    
    let req = test::TestRequest::post()
        .uri("/resources/categories")
        .set_json(&dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    // Assert
    assert_eq!(resp.status(), StatusCode::CREATED);
    
    let body = test::read_body(resp).await;
    let category: Category = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(category.name, "Transport");
    assert_eq!(category.profit, false);
}

#[actix_rt::test]
async fn test_create_category_handler_duplicate() {
    // Arrange
    let mock_service = MockCategoryService;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_service))
            .route("/resources/categories", web::post().to(|svc: web::Data<MockCategoryService>, body: web::Json<CreateCategoryDto>| async move {
                match svc.create_category(body.into_inner()).await {
                    Ok(category) => HttpResponse::Created().json(category),
                    Err(e) => e.error_response(),
                }
            }))
    )
    .await;
    
    // Act - Try to create duplicate "Food" category
    let dto = CreateCategoryDto {
        name: "Food".to_string(),
        profit: false,
    };
    
    let req = test::TestRequest::post()
        .uri("/resources/categories")
        .set_json(&dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    // Assert
    assert_eq!(resp.status(), StatusCode::CONFLICT);
}

#[actix_rt::test]
async fn test_create_category_handler_invalid_input() {
    // Arrange
    let mock_service = MockCategoryService;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_service))
            .route("/resources/categories", web::post().to(|_svc: web::Data<MockCategoryService>, body: web::Json<CreateCategoryDto>| async move {
                use validator::Validate;
                if let Err(e) = body.validate() {
                    return AppError::ValidationError(e.to_string()).error_response();
                }
                HttpResponse::Created().finish()
            }))
    )
    .await;
    
    // Act - Empty name
    let dto = CreateCategoryDto {
        name: "".to_string(),
        profit: false,
    };
    
    let req = test::TestRequest::post()
        .uri("/resources/categories")
        .set_json(&dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    // Assert
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_rt::test]
async fn test_update_category_handler_success() {
    // Arrange
    let mock_service = MockCategoryService;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_service))
            .route("/resources/categories/{name}/{profit}", web::put().to(|svc: web::Data<MockCategoryService>, path: web::Path<(String, bool)>, body: web::Json<UpdateCategoryDto>| async move {
                let (name, profit) = path.into_inner();
                match svc.update_category(name, profit, body.into_inner()).await {
                    Ok(category) => HttpResponse::Ok().json(category),
                    Err(e) => e.error_response(),
                }
            }))
    )
    .await;
    
    // Act
    let dto = UpdateCategoryDto {
        new_name: "Groceries".to_string(),
    };
    
    let req = test::TestRequest::put()
        .uri("/resources/categories/Food/false")
        .set_json(&dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    // Assert
    assert_eq!(resp.status(), StatusCode::OK);
    
    let body = test::read_body(resp).await;
    let category: Category = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(category.name, "Groceries");
    assert_eq!(category.profit, false);
}

#[actix_rt::test]
async fn test_update_category_handler_not_found() {
    // Arrange
    let mock_service = MockCategoryService;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_service))
            .route("/resources/categories/{name}/{profit}", web::put().to(|svc: web::Data<MockCategoryService>, path: web::Path<(String, bool)>, body: web::Json<UpdateCategoryDto>| async move {
                let (name, profit) = path.into_inner();
                match svc.update_category(name, profit, body.into_inner()).await {
                    Ok(category) => HttpResponse::Ok().json(category),
                    Err(e) => e.error_response(),
                }
            }))
    )
    .await;
    
    // Act - Try to update non-existent category
    let dto = UpdateCategoryDto {
        new_name: "Updated".to_string(),
    };
    
    let req = test::TestRequest::put()
        .uri("/resources/categories/NonExistent/false")
        .set_json(&dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    // Assert
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_rt::test]
async fn test_update_category_handler_conflict() {
    // Arrange
    let mock_service = MockCategoryService;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_service))
            .route("/resources/categories/{name}/{profit}", web::put().to(|svc: web::Data<MockCategoryService>, path: web::Path<(String, bool)>, body: web::Json<UpdateCategoryDto>| async move {
                let (name, profit) = path.into_inner();
                match svc.update_category(name, profit, body.into_inner()).await {
                    Ok(category) => HttpResponse::Ok().json(category),
                    Err(e) => e.error_response(),
                }
            }))
    )
    .await;
    
    // Act - Try to update to an existing category name
    let dto = UpdateCategoryDto {
        new_name: "Transport".to_string(),
    };
    
    let req = test::TestRequest::put()
        .uri("/resources/categories/Food/false")
        .set_json(&dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    // Assert
    assert_eq!(resp.status(), StatusCode::CONFLICT);
}

#[actix_rt::test]
async fn test_update_category_handler_invalid_input() {
    // Arrange
    let mock_service = MockCategoryService;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_service))
            .route("/resources/categories/{name}/{profit}", web::put().to(|_svc: web::Data<MockCategoryService>, _path: web::Path<(String, bool)>, body: web::Json<UpdateCategoryDto>| async move {
                use validator::Validate;
                if let Err(e) = body.validate() {
                    return AppError::ValidationError(e.to_string()).error_response();
                }
                HttpResponse::Ok().finish()
            }))
    )
    .await;
    
    // Act - Empty new name
    let dto = UpdateCategoryDto {
        new_name: "".to_string(),
    };
    
    let req = test::TestRequest::put()
        .uri("/resources/categories/Food/false")
        .set_json(&dto)
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    // Assert
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_rt::test]
async fn test_delete_category_handler_success() {
    // Arrange
    let mock_service = MockCategoryService;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_service))
            .route("/resources/categories/{name}/{profit}", web::delete().to(|svc: web::Data<MockCategoryService>, path: web::Path<(String, bool)>| async move {
                let (name, profit) = path.into_inner();
                match svc.delete_category(name, profit).await {
                    Ok(_) => HttpResponse::NoContent().finish(),
                    Err(e) => e.error_response(),
                }
            }))
    )
    .await;
    
    // Act - Delete existing category
    let req = test::TestRequest::delete()
        .uri("/resources/categories/Food/false")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    // Assert
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);
}

#[actix_rt::test]
async fn test_delete_category_handler_not_found() {
    // Arrange
    let mock_service = MockCategoryService;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_service))
            .route("/resources/categories/{name}/{profit}", web::delete().to(|svc: web::Data<MockCategoryService>, path: web::Path<(String, bool)>| async move {
                let (name, profit) = path.into_inner();
                match svc.delete_category(name, profit).await {
                    Ok(_) => HttpResponse::NoContent().finish(),
                    Err(e) => e.error_response(),
                }
            }))
    )
    .await;
    
    // Act - Attempt to delete non-existent category
    let req = test::TestRequest::delete()
        .uri("/resources/categories/NonExistent/false")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    // Assert
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
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
async fn test_get_wallets_nonexistent_user() {
    use money_manager_api::api::handlers::user_handler;
    use sqlx::PgPool;
    
    // Create a connection to the test database
    let pool_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://test:test@localhost/test".to_string());
    
    // Skip test if database is not available
    if let Ok(pool) = PgPool::connect(&pool_url).await {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool))
                .route("/resources/users/{login}/wallets", web::get().to(user_handler::get_wallets))
        )
        .await;
        
        let req = test::TestRequest::get()
            .uri("/resources/users/nonexistent_user_xyz123/wallets")
            .to_request();
        let resp = test::call_service(&app, req).await;
        
        // Should return 404 Not Found
        assert_eq!(resp.status(), 404);
        
        // Check error response format
        let body = test::read_body(resp).await;
        let error: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(error.get("message").is_some());
        let message = error["message"].as_str().unwrap();
        assert!(message.contains("not found") || message.contains("User"));
    }
}

#[actix_rt::test]
async fn test_get_wallet_summary_nonexistent_user() {
    use money_manager_api::api::handlers::user_handler;
    use sqlx::PgPool;
    
    // Create a connection to the test database
    let pool_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://test:test@localhost/test".to_string());
    
    // Skip test if database is not available
    if let Ok(pool) = PgPool::connect(&pool_url).await {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool))
                .route("/resources/users/{login}/wallets/{id}/summary", web::get().to(user_handler::get_summary))
        )
        .await;
        
        let req = test::TestRequest::get()
            .uri("/resources/users/nonexistent_user_xyz123/wallets/1/summary?start=2023-01-01&end=2023-12-31")
            .to_request();
        let resp = test::call_service(&app, req).await;
        
        // Should return 404 Not Found
        assert_eq!(resp.status(), 404);
        
        // Check error response format
        let body = test::read_body(resp).await;
        let error: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(error.get("message").is_some());
        let message = error["message"].as_str().unwrap();
        assert!(message.contains("not found") || message.contains("User"));
    }
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
async fn test_create_wallet_nonexistent_user() {
    use money_manager_api::api::handlers::user_handler;
    use sqlx::PgPool;
    
    // Create a connection to the test database
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
        
        // Test with non-existent user
        let wallet = WalletDto {
            id: None,
            name: "Test Wallet".to_string(),
            amount: Money::new(BigDecimal::from(100), None),
        };
        
        let req = test::TestRequest::post()
            .uri("/resources/users/nonexistent_user_login_12345/wallets")
            .set_json(&wallet)
            .to_request();
        let resp = test::call_service(&app, req).await;
        
        // Should return 404 Not Found
        assert_eq!(resp.status(), 404);
        
        // Check error response format
        let body = test::read_body(resp).await;
        let error: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(error.get("message").is_some());
        let message = error["message"].as_str().unwrap();
        assert!(message.contains("not found") || message.contains("User"));
    }
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

// ========================================
// Update Wallet Tests
// ========================================

#[actix_rt::test]
async fn test_update_wallet_success() {
    let mock_service = MockUserService;
    
    let app = test::init_service(
        App::new()
            .route("/resources/users/{login}/wallets/{id}", web::put().to(
                |user_service: web::Data<MockUserService>, path: web::Path<(String, i32)>, update: web::Json<UpdateWalletDto>| async move {
                    let (login, wallet_id) = path.into_inner();
                    let updated_wallet = user_service
                        .update_wallet(&login, wallet_id, update.into_inner())
                        .await
                        .unwrap();
                    HttpResponse::Ok().json(updated_wallet)
                }
            ))
            .app_data(web::Data::new(mock_service))
    )
    .await;
    
    // Test updating wallet name
    let update = serde_json::json!({
        "name": "Updated Wallet Name",
        "amount": null
    });
    
    let req = test::TestRequest::put()
        .uri("/resources/users/testuser/wallets/1")
        .insert_header(("content-type", "application/json"))
        .set_json(&update)
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::OK);
    let body = test::read_body(resp).await;
    let wallet: WalletDto = serde_json::from_slice(&body).unwrap();
    assert_eq!(wallet.name, "Updated Wallet Name");
}

#[actix_rt::test]
async fn test_update_wallet_amount_only() {
    let mock_service = MockUserService;
    
    let app = test::init_service(
        App::new()
            .route("/resources/users/{login}/wallets/{id}", web::put().to(
                |user_service: web::Data<MockUserService>, path: web::Path<(String, i32)>, update: web::Json<UpdateWalletDto>| async move {
                    let (login, wallet_id) = path.into_inner();
                    let updated_wallet = user_service
                        .update_wallet(&login, wallet_id, update.into_inner())
                        .await
                        .unwrap();
                    HttpResponse::Ok().json(updated_wallet)
                }
            ))
            .app_data(web::Data::new(mock_service))
    )
    .await;
    
    // Test updating only amount
    let update = serde_json::json!({
        "name": null,
        "amount": {
            "amount": "2500.50",
            "currency": "EUR"
        }
    });
    
    let req = test::TestRequest::put()
        .uri("/resources/users/testuser/wallets/1")
        .insert_header(("content-type", "application/json"))
        .set_json(&update)
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::OK);
    let body = test::read_body(resp).await;
    let wallet: WalletDto = serde_json::from_slice(&body).unwrap();
    assert_eq!(wallet.amount.currency, "EUR");
}

#[actix_rt::test]
async fn test_update_wallet_nonexistent_user() {
    let mock_service = MockUserService;
    
    let app = test::init_service(
        App::new()
            .route("/resources/users/{login}/wallets/{id}", web::put().to(
                |user_service: web::Data<MockUserService>, path: web::Path<(String, i32)>, update: web::Json<UpdateWalletDto>| async move {
                    let (login, wallet_id) = path.into_inner();
                    match user_service.update_wallet(&login, wallet_id, update.into_inner()).await {
                        Ok(wallet) => HttpResponse::Ok().json(wallet),
                        Err(e) => e.error_response(),
                    }
                }
            ))
            .app_data(web::Data::new(mock_service))
    )
    .await;
    
    let update = serde_json::json!({
        "name": "Updated Wallet"
    });
    
    let req = test::TestRequest::put()
        .uri("/resources/users/nonexistentuser/wallets/1")
        .insert_header(("content-type", "application/json"))
        .set_json(&update)
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_rt::test]
async fn test_update_wallet_not_authorized() {
    let mock_service = MockUserService;
    
    let app = test::init_service(
        App::new()
            .route("/resources/users/{login}/wallets/{id}", web::put().to(
                |user_service: web::Data<MockUserService>, path: web::Path<(String, i32)>, update: web::Json<UpdateWalletDto>| async move {
                    let (login, wallet_id) = path.into_inner();
                    match user_service.update_wallet(&login, wallet_id, update.into_inner()).await {
                        Ok(wallet) => HttpResponse::Ok().json(wallet),
                        Err(e) => e.error_response(),
                    }
                }
            ))
            .app_data(web::Data::new(mock_service))
    )
    .await;
    
    let update = serde_json::json!({
        "name": "Trying to update"
    });
    
    // Wallet ID 999 simulates a wallet that doesn't belong to the user
    let req = test::TestRequest::put()
        .uri("/resources/users/testuser/wallets/999")
        .insert_header(("content-type", "application/json"))
        .set_json(&update)
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[actix_rt::test]
async fn test_update_wallet_empty_name() {
    use validator::Validate;
    let mock_service = MockUserService;
    
    let app = test::init_service(
        App::new()
            .route("/resources/users/{login}/wallets/{id}", web::put().to(
                |_user_service: web::Data<MockUserService>, _path: web::Path<(String, i32)>, update: web::Json<UpdateWalletDto>| async move {
                    // Validate before processing
                    if let Err(e) = update.validate() {
                        return HttpResponse::BadRequest().json(serde_json::json!({
                            "status": "400",
                            "message": format!("Validation error: {}", e)
                        }));
                    }
                    HttpResponse::Ok().finish()
                }
            ))
            .app_data(web::Data::new(mock_service))
    )
    .await;
    
    let update = serde_json::json!({
        "name": ""
    });
    
    let req = test::TestRequest::put()
        .uri("/resources/users/testuser/wallets/1")
        .insert_header(("content-type", "application/json"))
        .set_json(&update)
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
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
            .route(
                "/resources/users/{login}/wallets/{id}/expenses",
                web::post().to(
                    |user_service: web::Data<MockUserService>, path: web::Path<(String, i32)>, expense: web::Json<ExpenseInputDto>| async move {
                        let (login, id) = path.into_inner();
                        let created_expense = user_service
                            .add_expense(&login, id, expense.into_inner())
                            .await
                            .unwrap();
                        HttpResponse::Created().json(created_expense)
                    },
                ),
            )
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
    assert_eq!(created_expense.id, Some(2));
    assert_eq!(created_expense.description, "Test expense");
    assert_eq!(created_expense.category.name, "Food");
    assert_eq!(created_expense.amount.currency, "USD");
}

#[actix_rt::test]
async fn test_delete_expense_success() {
    let mock_service = MockUserService;
    
    let app = test::init_service(
        App::new()
            .route(
                "/resources/users/{login}/wallets/{wallet_id}/expenses/{expense_id}",
                web::delete().to(
                    |user_service: web::Data<MockUserService>, path: web::Path<(String, i32, i32)>| async move {
                        let (login, wallet_id, expense_id) = path.into_inner();
                        match user_service.delete_expense(&login, wallet_id, expense_id).await {
                            Ok(()) => HttpResponse::NoContent().finish(),
                            Err(e) => e.error_response(),
                        }
                    },
                ),
            )
            .app_data(web::Data::new(mock_service))
    )
    .await;
    
    let req = test::TestRequest::delete()
        .uri("/resources/users/testuser/wallets/1/expenses/1")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);
}

#[actix_rt::test]
async fn test_delete_expense_not_found() {
    let mock_service = MockUserService;
    
    let app = test::init_service(
        App::new()
            .route(
                "/resources/users/{login}/wallets/{wallet_id}/expenses/{expense_id}",
                web::delete().to(
                    |user_service: web::Data<MockUserService>, path: web::Path<(String, i32, i32)>| async move {
                        let (login, wallet_id, expense_id) = path.into_inner();
                        match user_service.delete_expense(&login, wallet_id, expense_id).await {
                            Ok(()) => HttpResponse::NoContent().finish(),
                            Err(e) => e.error_response(),
                        }
                    },
                ),
            )
            .app_data(web::Data::new(mock_service))
    )
    .await;
    
    // Try to delete a non-existent expense (ID 999999)
    let req = test::TestRequest::delete()
        .uri("/resources/users/testuser/wallets/7/expenses/999999")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    
    // Verify the response body contains the expected error message
    let body = test::read_body(resp).await;
    let error_response: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(error_response["status"], "404");
    assert!(error_response["message"].as_str().unwrap().contains("999999"));
    assert!(error_response["message"].as_str().unwrap().contains("does not exist"));
}

#[actix_rt::test]
async fn test_create_budget_handler() {
    let mock_service = MockUserService;
    
    let app = test::init_service(
        App::new()
            .route(
                "/resources/users/{login}/budgets",
                web::post().to(
                    |user_service: web::Data<MockUserService>, path: web::Path<String>, budget: web::Json<money_manager_api::models::BudgetInputDto>| async move {
                        let login = path.into_inner();
                        let budget_model: Budget = budget.into_inner().into();
                        let created_budget = user_service
                            .add_budget(&login, budget_model)
                            .await
                            .unwrap();
                        
                        let budget_id = created_budget.id.unwrap();
                        let location = format!("/{}/budgets/{}", login, budget_id);
                        
                        HttpResponse::Created()
                            .append_header(("Location", location))
                            .json(created_budget)
                    },
                ),
            )
            .app_data(web::Data::new(mock_service))
    )
    .await;
    
    let budget_input = money_manager_api::models::BudgetInputDto {
        category: Category::new("Food".to_string(), false),
        total: Money::new(BigDecimal::from(500), Some("USD".to_string())),
        date_range: DateRange {
            start: NaiveDate::from_ymd_opt(2025, 11, 1).unwrap(),
            end: NaiveDate::from_ymd_opt(2025, 11, 30).unwrap(),
        },
    };
    
    let req = test::TestRequest::post()
        .uri("/resources/users/testuser/budgets")
        .set_json(&budget_input)
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    // Assert HTTP 201 Created status
    assert_eq!(resp.status(), StatusCode::CREATED);
    
    // Assert Location header is present
    let location_header = resp.headers().get("Location");
    assert!(location_header.is_some());
    assert_eq!(location_header.unwrap().to_str().unwrap(), "/testuser/budgets/1");
    
    // Assert response body contains the created Budget
    let body = test::read_body(resp).await;
    let created_budget: Budget = serde_json::from_slice(&body).unwrap();
    
    // Verify the created budget has all the expected fields
    assert_eq!(created_budget.id, Some(1));
    assert_eq!(created_budget.category.name, "Food");
    assert_eq!(created_budget.category.profit, false);
    assert_eq!(created_budget.total.amount, BigDecimal::from(500));
    assert_eq!(created_budget.total.currency, "USD");
    assert_eq!(created_budget.date_range.start, NaiveDate::from_ymd_opt(2025, 11, 1).unwrap());
    assert_eq!(created_budget.date_range.end, NaiveDate::from_ymd_opt(2025, 11, 30).unwrap());
}

#[actix_rt::test]
async fn test_create_budget_returns_json_response() {
    // This test specifically validates that the response is JSON and not empty
    let mock_service = MockUserService;
    
    let app = test::init_service(
        App::new()
            .route(
                "/resources/users/{login}/budgets",
                web::post().to(
                    |user_service: web::Data<MockUserService>, path: web::Path<String>, budget: web::Json<money_manager_api::models::BudgetInputDto>| async move {
                        let login = path.into_inner();
                        let budget_model: Budget = budget.into_inner().into();
                        let created_budget = user_service
                            .add_budget(&login, budget_model)
                            .await
                            .unwrap();
                        
                        let budget_id = created_budget.id.unwrap();
                        let location = format!("/{}/budgets/{}", login, budget_id);
                        
                        HttpResponse::Created()
                            .append_header(("Location", location))
                            .json(created_budget)
                    },
                ),
            )
            .app_data(web::Data::new(mock_service))
    )
    .await;
    
    let budget_input = money_manager_api::models::BudgetInputDto {
        category: Category::new("Entertainment".to_string(), false),
        total: Money::new(BigDecimal::from(300), Some("PLN".to_string())),
        date_range: DateRange {
            start: NaiveDate::from_ymd_opt(2025, 12, 1).unwrap(),
            end: NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
        },
    };
    
    let req = test::TestRequest::post()
        .uri("/resources/users/user3/budgets")
        .set_json(&budget_input)
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    // Verify status code
    assert_eq!(resp.status(), StatusCode::CREATED);
    
    // Verify Content-Type is application/json
    let content_type = resp.headers().get("content-type");
    assert!(content_type.is_some());
    assert!(content_type.unwrap().to_str().unwrap().contains("application/json"));
    
    // Verify response body is not empty
    let body = test::read_body(resp).await;
    assert!(!body.is_empty());
    
    // Verify it can be deserialized to Budget
    let budget: Budget = serde_json::from_slice(&body).unwrap();
    assert!(budget.id.is_some());
    assert_eq!(budget.category.name, "Entertainment");
    assert_eq!(budget.total.currency, "PLN");
}

#[actix_rt::test]
async fn test_delete_user_success() {
    // Arrange
    let mock_service = MockUserService;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_service))
            .route("/resources/users/{login}", web::delete().to(|
                path: web::Path<String>,
                svc: web::Data<MockUserService>
            | async move {
                let login = path.into_inner();
                svc.delete_user(&login).await?;
                
                let response = serde_json::json!({
                    "message": format!("User `{}` deleted successfully", login)
                });
                
                Ok::<_, AppError>(HttpResponse::Ok().json(response))
            }))
    )
    .await;
    
    // Act
    let req = test::TestRequest::delete()
        .uri("/resources/users/testuser")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    // Assert
    assert_eq!(resp.status(), StatusCode::OK);
    
    let body = test::read_body(resp).await;
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert!(response.get("message").is_some());
    assert!(response["message"].as_str().unwrap().contains("testuser"));
    assert!(response["message"].as_str().unwrap().contains("deleted successfully"));
}

#[actix_rt::test]
async fn test_delete_user_not_found() {
    // Arrange
    let mock_service = MockUserService;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_service))
            .route("/resources/users/{login}", web::delete().to(|
                path: web::Path<String>,
                svc: web::Data<MockUserService>
            | async move {
                let login = path.into_inner();
                svc.delete_user(&login).await?;
                
                let response = serde_json::json!({
                    "message": format!("User `{}` deleted successfully", login)
                });
                
                Ok::<_, AppError>(HttpResponse::Ok().json(response))
            }))
    )
    .await;
    
    // Act
    let req = test::TestRequest::delete()
        .uri("/resources/users/nonexistentuser")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    // Assert - should return 404 Not Found
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

// ============================================
// Delete Wallet Tests
// ============================================

#[actix_rt::test]
async fn test_delete_wallet_success() {
    // Arrange
    let mock_service = MockUserService;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_service))
            .route("/resources/users/{login}/wallets/{id}", web::delete().to(|
                path: web::Path<(String, i32)>,
                svc: web::Data<MockUserService>
            | async move {
                let (login, wallet_id) = path.into_inner();
                svc.delete_wallet(&login, wallet_id).await?;
                Ok::<_, AppError>(HttpResponse::NoContent().finish())
            }))
    )
    .await;
    
    // Act - Delete wallet ID 1 for testuser
    let req = test::TestRequest::delete()
        .uri("/resources/users/testuser/wallets/1")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    // Assert - should return 204 No Content
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);
}

#[actix_rt::test]
async fn test_delete_wallet_user_not_found() {
    // Arrange
    let mock_service = MockUserService;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_service))
            .route("/resources/users/{login}/wallets/{id}", web::delete().to(|
                path: web::Path<(String, i32)>,
                svc: web::Data<MockUserService>
            | async move {
                let (login, wallet_id) = path.into_inner();
                svc.delete_wallet(&login, wallet_id).await?;
                Ok::<_, AppError>(HttpResponse::NoContent().finish())
            }))
    )
    .await;
    
    // Act - Try to delete wallet for non-existent user
    let req = test::TestRequest::delete()
        .uri("/resources/users/nonexistentuser/wallets/1")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    // Assert - should return 404 Not Found
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_rt::test]
async fn test_delete_wallet_wallet_not_found() {
    // Arrange
    let mock_service = MockUserService;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_service))
            .route("/resources/users/{login}/wallets/{id}", web::delete().to(|
                path: web::Path<(String, i32)>,
                svc: web::Data<MockUserService>
            | async move {
                let (login, wallet_id) = path.into_inner();
                svc.delete_wallet(&login, wallet_id).await?;
                Ok::<_, AppError>(HttpResponse::NoContent().finish())
            }))
    )
    .await;
    
    // Act - Try to delete non-existent wallet (ID 999)
    let req = test::TestRequest::delete()
        .uri("/resources/users/testuser/wallets/999")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    // Assert - should return 404 Not Found
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_rt::test]
async fn test_delete_wallet_not_authorized() {
    // Arrange
    let mock_service = MockUserService;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_service))
            .route("/resources/users/{login}/wallets/{id}", web::delete().to(|
                path: web::Path<(String, i32)>,
                svc: web::Data<MockUserService>
            | async move {
                let (login, wallet_id) = path.into_inner();
                svc.delete_wallet(&login, wallet_id).await?;
                Ok::<_, AppError>(HttpResponse::NoContent().finish())
            }))
    )
    .await;
    
    // Act - Try to delete wallet that belongs to another user (ID 888)
    let req = test::TestRequest::delete()
        .uri("/resources/users/testuser/wallets/888")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    // Assert - should return 403 Forbidden
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
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

