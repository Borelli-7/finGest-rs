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