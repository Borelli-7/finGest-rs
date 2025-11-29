use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use serde::{Deserialize, Serialize};

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    
    #[error("Authorization error: {0}")]
    AuthorizationError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Not found: {0}")]
    NotFoundError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Bad request: {0}")]
    BadRequestError(String),
    
    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    status: String,
    message: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::AuthenticationError(message) => {
                HttpResponse::Unauthorized().json(ErrorResponse {
                    status: "401".to_string(),
                    message: message.to_string(),
                })
            }
            AppError::AuthorizationError(message) => {
                HttpResponse::Forbidden().json(ErrorResponse {
                    status: "403".to_string(),
                    message: message.to_string(),
                })
            }
            AppError::NotFoundError(message) => {
                HttpResponse::NotFound().json(ErrorResponse {
                    status: "404".to_string(),
                    message: message.to_string(),
                })
            }
            AppError::ValidationError(message) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    status: "400".to_string(),
                    message: message.to_string(),
                })
            }
            AppError::BadRequestError(message) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    status: "400".to_string(),
                    message: message.to_string(),
                })
            }
            AppError::DatabaseError(message) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    status: "500".to_string(),
                    message: message.to_string(),
                })
            }
            AppError::InternalServerError(message) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    status: "500".to_string(),
                    message: message.to_string(),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    
    #[test]
    fn test_authentication_error_response() {
        let error = AppError::AuthenticationError("Invalid credentials".to_string());
        let response = error.error_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
    
    #[test]
    fn test_authorization_error_response() {
        let error = AppError::AuthorizationError("Forbidden".to_string());
        let response = error.error_response();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
    
    #[test]
    fn test_not_found_error_response() {
        let error = AppError::NotFoundError("Resource not found".to_string());
        let response = error.error_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
    
    #[test]
    fn test_validation_error_response() {
        let error = AppError::ValidationError("Invalid input".to_string());
        let response = error.error_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
    
    #[test]
    fn test_bad_request_error_response() {
        let error = AppError::BadRequestError("Bad request".to_string());
        let response = error.error_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
    
    #[test]
    fn test_database_error_response() {
        let error = AppError::DatabaseError("Database connection failed".to_string());
        let response = error.error_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    #[test]
    fn test_internal_server_error_response() {
        let error = AppError::InternalServerError("Something went wrong".to_string());
        let response = error.error_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    #[test]
    fn test_error_display() {
        let error = AppError::AuthenticationError("Test message".to_string());
        assert_eq!(error.to_string(), "Authentication error: Test message");
    }
}