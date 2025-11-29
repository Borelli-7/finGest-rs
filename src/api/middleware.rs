use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::LocalBoxFuture;
use std::future::{ready, Ready};

use crate::errors::AppError;
use crate::services::auth_service::Claims;

/// Middleware for JWT authentication
pub struct JwtAuth {
    jwt_secret: String,
}

impl JwtAuth {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }
}

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuthMiddleware {
            service,
            jwt_secret: self.jwt_secret.clone(),
        }))
    }
}

pub struct JwtAuthMiddleware<S> {
    service: S,
    jwt_secret: String,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let jwt_secret = self.jwt_secret.clone();
        
        // Extract token from Authorization header
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .map(|t| t.to_string());
        
        if let Some(token) = token {
            // Verify the token
            match verify_jwt_token(&token, &jwt_secret) {
                Ok(claims) => {
                    // Store claims in request extensions for later use
                    req.extensions_mut().insert(claims);
                    
                    let fut = self.service.call(req);
                    Box::pin(async move {
                        let res = fut.await?;
                        Ok(res)
                    })
                }
                Err(e) => {
                    Box::pin(async move {
                        Err(Error::from(e))
                    })
                }
            }
        } else {
            Box::pin(async move {
                Err(Error::from(AppError::AuthenticationError(
                    "Missing Authorization header".to_string(),
                )))
            })
        }
    }
}

/// Helper function to verify JWT token
fn verify_jwt_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    use jsonwebtoken::{decode, DecodingKey, Validation};
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| AppError::AuthenticationError(format!("Invalid token: {}", e)))?;
    
    Ok(token_data.claims)
}

/// Helper function to extract claims from request
pub fn get_claims_from_request(req: &actix_web::HttpRequest) -> Option<Claims> {
    req.extensions().get::<Claims>().cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_verify_jwt_token() {
        let secret = "test_secret_key_12345678901234567890";
        
        // Create a test user and generate a token
        use crate::models::User;
        
        let user = User {
            login: "testuser".to_string(),
            first_name: Some("Test".to_string()),
            last_name: Some("User".to_string()),
            password: Some("hashed_password".to_string()),
            admin: false,
        };
        
        // Generate token using the auth service logic
        use jsonwebtoken::{encode, EncodingKey, Header};
        use chrono::{Duration, Utc};
        
        let now = Utc::now();
        let expiration = now + Duration::hours(24);
        
        let claims = Claims {
            sub: user.login.clone(),
            admin: user.admin,
            exp: expiration.timestamp(),
            iat: now.timestamp(),
        };
        
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        ).unwrap();
        
        // Verify the token
        let result = verify_jwt_token(&token, secret);
        assert!(result.is_ok());
        
        let verified_claims = result.unwrap();
        assert_eq!(verified_claims.sub, "testuser");
        assert_eq!(verified_claims.admin, false);
    }
}
