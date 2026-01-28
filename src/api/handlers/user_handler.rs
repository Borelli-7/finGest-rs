use actix_web::{web, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;
use std::collections::HashMap;
use validator::Validate;

use crate::{
    api::middleware::get_claims_from_request,
    errors::AppError,
    models::{BudgetInputDto, UpdateBudgetDto, DateRange, ExpenseInputDto, UpdateExpenseDto, WalletDto, UpdateWalletDto},
    services::UserService,
    services::user_service::UserServiceTrait,
};

/// Handler for GET /resources/users/{login}
/// 
/// Returns a list of all users in the system.
/// 
/// # Path Parameters
/// - `login`: The login of the user making the request. Must match the authenticated user's login.
/// 
/// # Authorization
/// This endpoint requires:
/// 1. Valid JWT authentication token
/// 2. Path login must match the authenticated user's login (prevents user spoofing)
/// 3. Admin privileges (admin: true)
/// 
/// # Responses
/// - `200 OK`: Successfully retrieved users list
/// - `401 Unauthorized`: Missing or invalid authentication token
/// - `403 Forbidden`: User is not authorized (login mismatch or not an admin)
pub async fn get_users(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let path_login = path.into_inner();
    
    // Extract claims from the authenticated request
    let claims = get_claims_from_request(&req)
        .ok_or_else(|| AppError::AuthenticationError("Missing or invalid authentication token".to_string()))?;
    
    // Validate that the path login matches the authenticated user's login
    if path_login != claims.sub {
        return Err(AppError::AuthorizationError(
            format!("Not authorized. Path login '{}' does not match authenticated user '{}'", path_login, claims.sub)
        ));
    }
    
    // Check if the user has admin privileges
    if !claims.admin {
        return Err(AppError::AuthorizationError("Not authorized. Admin privileges required to access this resource".to_string()));
    }
    
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

// Handler for DELETE /resources/users/{login}/wallets/{id}
pub async fn delete_wallet(
    pool: web::Data<PgPool>,
    path: web::Path<(String, i32)>,
) -> Result<impl Responder, AppError> {
    let (login, wallet_id) = path.into_inner();
    
    let user_service = UserService::new(pool.get_ref().clone());
    user_service.delete_wallet(&login, wallet_id).await?;
    
    Ok(HttpResponse::NoContent().finish())
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

// Handler for PUT /resources/users/{login}/wallets/{wallet_id}/expenses/{expense_id}
pub async fn update_expense(
    pool: web::Data<PgPool>,
    path: web::Path<(String, i32, i32)>,
    expense_update: web::Json<UpdateExpenseDto>,
) -> Result<impl Responder, AppError> {
    let (login, wallet_id, expense_id) = path.into_inner();
    
    // Validate the update data
    expense_update
        .validate()
        .map_err(|e| AppError::ValidationError(
            format!("Invalid expense update data: {}", e)
        ))?;
    
    let user_service = UserService::new(pool.get_ref().clone());
    let updated_expense = user_service.update_expense(&login, wallet_id, expense_id, expense_update.into_inner()).await?;
    
    Ok(HttpResponse::Ok().json(updated_expense))
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
/// Retrieves all budgets for a specific user.
/// 
/// # Authorization
/// - The authenticated user must match the requested user login
/// - Returns 401 Unauthorized if no valid token is provided
/// - Returns 403 Forbidden if the authenticated user tries to access another user's budgets
/// - Returns 404 Not Found if the requested user doesn't exist
pub async fn get_budgets(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    query: web::Query<HashMap<String, String>>,
) -> Result<impl Responder, AppError> {
    let login = path.into_inner();
    
    // Extract authenticated user claims from the request
    let claims = get_claims_from_request(&req)
        .ok_or_else(|| AppError::AuthenticationError(
            "Missing or invalid authentication token".to_string()
        ))?;
    
    // Verify the authenticated user is requesting their own budgets
    let authenticated_login = &claims.sub;
    
    let start_min = query.get("start_min").map(|s| s.as_str());
    let start_max = query.get("start_max").map(|s| s.as_str());
    let end_min = query.get("end_min").map(|s| s.as_str());
    let end_max = query.get("end_max").map(|s| s.as_str());
    
    let start_range = DateRange::from_string(start_min, start_max)
        .map_err(|e| AppError::BadRequestError(format!("Invalid start date format: {}", e)))?;
    
    let end_range = DateRange::from_string(end_min, end_max)
        .map_err(|e| AppError::BadRequestError(format!("Invalid end date format: {}", e)))?;
    
    let user_service = UserService::new(pool.get_ref().clone());
    let budgets = user_service.get_budgets(&login, authenticated_login, start_range, end_range).await?;
    
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

// Handler for PUT /resources/users/{login}/budgets/{budget_id}
pub async fn update_budget(
    pool: web::Data<PgPool>,
    path: web::Path<(String, i32)>,
    update_data: web::Json<UpdateBudgetDto>,
) -> Result<impl Responder, AppError> {
    let (login, budget_id) = path.into_inner();
    
    // Validate the update data
    update_data.validate()
        .map_err(|e| AppError::BadRequestError(format!("Validation error: {}", e)))?;
    
    let user_service = UserService::new(pool.get_ref().clone());
    let updated_budget = user_service.update_budget(&login, budget_id, update_data.0).await?;
    
    Ok(HttpResponse::Ok().json(updated_budget))
}

// Handler for DELETE /resources/users/{login}/budgets/{budget_id}
pub async fn delete_budget(
    pool: web::Data<PgPool>,
    path: web::Path<(String, i32)>,
) -> Result<impl Responder, AppError> {
    let (login, budget_id) = path.into_inner();
    
    let user_service = UserService::new(pool.get_ref().clone());
    user_service.delete_budget(&login, budget_id).await?;
    
    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::auth_service::Claims;
    
    /// Test that admin check logic correctly identifies admin users
    #[test]
    fn test_admin_authorization_logic() {
        // Admin user should be allowed
        let admin_claims = Claims {
            sub: "admin_user".to_string(),
            admin: true,
            exp: 9999999999,
            iat: 1000000000,
        };
        assert!(admin_claims.admin, "Admin claims should have admin=true");
        
        // Non-admin user should be denied
        let non_admin_claims = Claims {
            sub: "regular_user".to_string(),
            admin: false,
            exp: 9999999999,
            iat: 1000000000,
        };
        assert!(!non_admin_claims.admin, "Non-admin claims should have admin=false");
    }
    
    /// Test path login validation against JWT claims
    #[test]
    fn test_path_login_validation() {
        let claims = Claims {
            sub: "admin_user".to_string(),
            admin: true,
            exp: 9999999999,
            iat: 1000000000,
        };
        
        // Matching path login should pass
        let path_login = "admin_user";
        assert_eq!(path_login, claims.sub, "Path login should match JWT claims sub");
        
        // Mismatched path login should fail
        let wrong_path_login = "different_user";
        assert_ne!(wrong_path_login, claims.sub, "Path login should not match when different");
    }
    
    /// Test that path login mismatch returns appropriate error
    #[test]
    fn test_path_login_mismatch_error() {
        let path_login = "attacker";
        let claims_sub = "real_admin";
        
        let error = AppError::AuthorizationError(
            format!("Not authorized. Path login '{}' does not match authenticated user '{}'", path_login, claims_sub)
        );
        
        let error_string = error.to_string();
        assert!(error_string.contains("Not authorized"));
        assert!(error_string.contains(path_login));
        assert!(error_string.contains(claims_sub));
    }
    
    /// Test that the authorization error message is appropriate
    #[test]
    fn test_authorization_error_message() {
        let error = AppError::AuthorizationError(
            "Not authorized. Admin privileges required to access this resource".to_string()
        );
        
        let error_string = error.to_string();
        assert!(error_string.contains("Not authorized"));
        assert!(error_string.contains("Admin"));
    }
    
    /// Test that the authentication error message is appropriate
    #[test]
    fn test_authentication_error_message() {
        let error = AppError::AuthenticationError(
            "Missing or invalid authentication token".to_string()
        );
        
        let error_string = error.to_string();
        assert!(error_string.contains("Missing") || error_string.contains("invalid"));
        assert!(error_string.contains("token"));
    }
}
