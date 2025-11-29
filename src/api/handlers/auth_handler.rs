use actix_web::{web, HttpResponse, Result};
use validator::Validate;

use crate::errors::AppError;
use crate::models::CreateUserDto;
use crate::services::auth_service::{AuthServiceTrait, LoginDto};

/// Handler for user registration
/// POST /api/auth/register
pub async fn register(
    auth_service: web::Data<Box<dyn AuthServiceTrait>>,
    user_dto: web::Json<CreateUserDto>,
) -> Result<HttpResponse, AppError> {
    // Validate the input
    user_dto
        .validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    // Register the user
    let created_user = auth_service.register(user_dto.into_inner()).await?;
    
    Ok(HttpResponse::Created().json(created_user))
}

/// Handler for user login
/// POST /api/auth/login
pub async fn login(
    auth_service: web::Data<Box<dyn AuthServiceTrait>>,
    login_dto: web::Json<LoginDto>,
) -> Result<HttpResponse, AppError> {
    // Validate the input
    login_dto
        .validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    // Login the user
    let login_response = auth_service.login(login_dto.into_inner()).await?;
    
    Ok(HttpResponse::Ok().json(login_response))
}

/// Handler for token verification
/// GET /api/auth/verify
pub async fn verify_token(
    auth_service: web::Data<Box<dyn AuthServiceTrait>>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, AppError> {
    // Extract token from Authorization header
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::AuthenticationError("Missing or invalid Authorization header".to_string()))?;
    
    // Verify the token
    let claims = auth_service.verify_token(token).await?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "valid": true,
        "login": claims.sub,
        "admin": claims.admin,
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::UserDto;
    use crate::services::auth_service::MockAuthService;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_register_success() {
        let mut mock_service = MockAuthService::new();
        
        mock_service
            .expect_register()
            .times(1)
            .returning(|user_dto| {
                Ok(UserDto {
                    login: user_dto.login,
                    first_name: user_dto.first_name,
                    last_name: user_dto.last_name,
                    admin: user_dto.admin.unwrap_or(false),
                })
            });
        
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Box::new(mock_service) as Box<dyn AuthServiceTrait>))
                .route("/register", web::post().to(register))
        )
        .await;
        
        let user_dto = CreateUserDto {
            login: "testuser".to_string(),
            first_name: Some("Test".to_string()),
            last_name: Some("User".to_string()),
            password: "password123".to_string(),
            admin: Some(false),
        };
        
        let req = test::TestRequest::post()
            .uri("/register")
            .set_json(&user_dto)
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 201);
    }
}
