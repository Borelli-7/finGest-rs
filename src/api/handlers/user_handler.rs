use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use std::collections::HashMap;
use validator::Validate;

use crate::{
    errors::AppError,
    models::{BudgetInputDto, DateRange, ExpenseInputDto, WalletDto, UpdateWalletDto},
    services::UserService,
    services::user_service::UserServiceTrait,
};

// Handler for GET /resources/users
pub async fn get_users(pool: web::Data<PgPool>) -> Result<impl Responder, AppError> {
    let user_service = UserService::new(pool.get_ref().clone());
    let users = user_service.get_users().await?;
    
    Ok(HttpResponse::Ok().json(users))
}

// Handler for PUT /resources/users/{login}
pub async fn update_user(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    query: web::Query<HashMap<String, String>>,
    body: web::Json<HashMap<String, serde_json::Value>>,
) -> Result<impl Responder, AppError> {
    let login = path.into_inner();
    let field = query.get("field").ok_or_else(|| AppError::BadRequestError("Field parameter is required".to_string()))?;
    
    let user_service = UserService::new(pool.get_ref().clone());
    user_service.update_user(&login, field, body.into_inner()).await?;
    
    Ok(HttpResponse::NoContent().finish())
}

// Handler for DELETE /resources/users/{login}
pub async fn delete_user(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let login = path.into_inner();
    
    let user_service = UserService::new(pool.get_ref().clone());
    user_service.delete_user(&login).await?;
    
    let response = serde_json::json!({
        "message": format!("User `{}` deleted successfully", login)
    });
    
    Ok(HttpResponse::Ok().json(response))
}

// Handler for GET /resources/users/{login}/wallets
pub async fn get_wallets(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let login = path.into_inner();
    
    let user_service = UserService::new(pool.get_ref().clone());
    let wallets = user_service.get_wallets(&login).await?;
    
    Ok(HttpResponse::Ok().json(wallets))
}

// Handler for POST /resources/users/{login}/wallets
pub async fn create_wallet(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    wallet: web::Json<WalletDto>,
) -> Result<impl Responder, AppError> {
    let login = path.into_inner();
    
    // Validate the wallet input
    wallet
        .validate()
        .map_err(|_| AppError::ValidationError(
            "The amount of wallet to be created is not valid".to_string()
        ))?;
    
    let user_service = UserService::new(pool.get_ref().clone());
    let created_wallet = user_service.add_wallet(&login, wallet.into_inner()).await?;
    
    // Build location header using the wallet ID
    let location = format!("/resources/users/{}/wallets/{}", login, created_wallet.id.unwrap_or(0));
    
    Ok(HttpResponse::Created()
        .append_header(("Location", location))
        .json(created_wallet))
}

// Handler for PUT /resources/users/{login}/wallets/{id}
pub async fn update_wallet(
    pool: web::Data<PgPool>,
    path: web::Path<(String, i32)>,
    wallet_update: web::Json<UpdateWalletDto>,
) -> Result<impl Responder, AppError> {
    let (login, wallet_id) = path.into_inner();
    
    // Validate the update data
    wallet_update
        .validate()
        .map_err(|_| AppError::ValidationError(
            "The wallet update data is not valid".to_string()
        ))?;
    
    let user_service = UserService::new(pool.get_ref().clone());
    let updated_wallet = user_service.update_wallet(&login, wallet_id, wallet_update.into_inner()).await?;
    
    Ok(HttpResponse::Ok().json(updated_wallet))
}

// Handler for GET /resources/users/{login}/wallets/{id}/summary
pub async fn get_summary(
    pool: web::Data<PgPool>,
    path: web::Path<(String, i32)>,
    query: web::Query<HashMap<String, String>>,
) -> Result<impl Responder, AppError> {
    let (login, id) = path.into_inner();
    let start = query.get("start").map(|s| s.as_str());
    let end = query.get("end").map(|e| e.as_str());
    
    let date_range = DateRange::from_string(start, end)
        .map_err(|e| AppError::BadRequestError(format!("Invalid date format: {}", e)))?;
    
    let user_service = UserService::new(pool.get_ref().clone());
    let summary = user_service.get_summary(&login, id, date_range).await?;
    
    Ok(HttpResponse::Ok().json(summary))
}

// Handler for GET /resources/users/{login}/wallets/{id}/expenses
pub async fn get_expenses(
    pool: web::Data<PgPool>,
    path: web::Path<(String, i32)>,
    query: web::Query<HashMap<String, String>>,
) -> Result<impl Responder, AppError> {
    let (login, id) = path.into_inner();
    let start = query.get("start").map(|s| s.as_str());
    let end = query.get("end").map(|e| e.as_str());
    
    let date_range = DateRange::from_string(start, end)
        .map_err(|e| AppError::BadRequestError(format!("Invalid date format: {}", e)))?;
    
    let user_service = UserService::new(pool.get_ref().clone());
    let expenses = user_service.get_expenses(&login, id, date_range).await?;
    
    Ok(HttpResponse::Ok().json(expenses))
}

// Handler for GET /resources/users/{login}/wallets/{id}/highest_expense
pub async fn get_highest_expense(
    pool: web::Data<PgPool>,
    path: web::Path<(String, i32)>,
    query: web::Query<HashMap<String, String>>,
) -> Result<impl Responder, AppError> {
    let (login, id) = path.into_inner();
    let start = query.get("start").map(|s| s.as_str());
    let end = query.get("end").map(|e| e.as_str());
    
    let date_range = DateRange::from_string(start, end)
        .map_err(|e| AppError::BadRequestError(format!("Invalid date format: {}", e)))?;
    
    let user_service = UserService::new(pool.get_ref().clone());
    let highest_expense = user_service.get_highest_expense(&login, id, date_range).await?;
    
    Ok(HttpResponse::Ok().json(highest_expense))
}

// Handler for POST /resources/users/{login}/wallets/{id}/expenses
pub async fn create_expense(
    pool: web::Data<PgPool>,
    path: web::Path<(String, i32)>,
    expense: web::Json<ExpenseInputDto>,
) -> Result<impl Responder, AppError> {
    let (login, id) = path.into_inner();
    
    let user_service = UserService::new(pool.get_ref().clone());
    let created_expense = user_service.add_expense(&login, id, expense.into_inner()).await?;
    
    let expense_id = created_expense
        .id
        .expect("Expense ID must be present after creation - this indicates a bug in the expense creation logic");
    let location = format!("/{}/wallets/{}/expenses/{}", login, id, expense_id);
    
    Ok(HttpResponse::Created()
        .append_header(("Location", location))
        .json(created_expense))
}

// Handler for DELETE /resources/users/{login}/wallets/{wallet_id}/expenses/{expense_id}
pub async fn delete_expense(
    pool: web::Data<PgPool>,
    path: web::Path<(String, i32, i32)>,
) -> Result<impl Responder, AppError> {
    let (login, wallet_id, expense_id) = path.into_inner();
    
    let user_service = UserService::new(pool.get_ref().clone());
    user_service.delete_expense(&login, wallet_id, expense_id).await?;
    
    Ok(HttpResponse::NoContent().finish())
}

// Handler for GET /resources/users/{login}/wallets/{id}/counted_categories
pub async fn get_counted_categories(
    pool: web::Data<PgPool>,
    path: web::Path<(String, i32)>,
    query: web::Query<HashMap<String, String>>,
) -> Result<impl Responder, AppError> {
    let (login, id) = path.into_inner();
    let start = query.get("start").map(|s| s.as_str());
    let end = query.get("end").map(|e| e.as_str());
    
    let date_range = DateRange::from_string(start, end)
        .map_err(|e| AppError::BadRequestError(format!("Invalid date format: {}", e)))?;
    
    let user_service = UserService::new(pool.get_ref().clone());
    let categories = user_service.get_counted_categories(&login, id, date_range).await?;
    
    Ok(HttpResponse::Ok().json(categories))
}

// Handler for GET /resources/users/{login}/budgets
pub async fn get_budgets(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    query: web::Query<HashMap<String, String>>,
) -> Result<impl Responder, AppError> {
    let login = path.into_inner();
    
    let start_min = query.get("start_min").map(|s| s.as_str());
    let start_max = query.get("start_max").map(|s| s.as_str());
    let end_min = query.get("end_min").map(|s| s.as_str());
    let end_max = query.get("end_max").map(|s| s.as_str());
    
    let start_range = DateRange::from_string(start_min, start_max)
        .map_err(|e| AppError::BadRequestError(format!("Invalid start date format: {}", e)))?;
    
    let end_range = DateRange::from_string(end_min, end_max)
        .map_err(|e| AppError::BadRequestError(format!("Invalid end date format: {}", e)))?;
    
    let user_service = UserService::new(pool.get_ref().clone());
    let budgets = user_service.get_budgets(&login, start_range, end_range).await?;
    
    Ok(HttpResponse::Ok().json(budgets))
}

// Handler for POST /resources/users/{login}/budgets
pub async fn create_budget(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    budget: web::Json<BudgetInputDto>,
) -> Result<impl Responder, AppError> {
    let login = path.into_inner();
    
    let user_service = UserService::new(pool.get_ref().clone());
    let created_budget = user_service.add_budget(&login, budget.0.into()).await?;
    
    let budget_id = created_budget
        .id
        .expect("Budget ID must be present after creation - this indicates a bug in the budget creation logic");
    let location = format!("/{}/budgets/{}", login, budget_id);
    
    Ok(HttpResponse::Created()
        .append_header(("Location", location))
        .json(created_budget))
}
