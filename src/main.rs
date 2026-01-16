use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer, HttpResponse};
use dotenv::dotenv;
use money_manager_api::{
    config::Config, 
    db::init_db,
    api::routes::configure_routes,
    services::{AuthService, AuthServiceTrait},
};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use std::io;
use tracing_subscriber::fmt::format::FmtSpan;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Initialize tracing (instead of using both env_logger and tracing_subscriber)
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
        .try_init()
        .expect("Failed to initialize tracing subscriber");
    
    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");
    
    // Set up database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .connect(&config.db_url)
        .await
        .expect("Failed to create connection pool");
    
    // Apply database migrations
    init_db(&pool).await.expect("Failed to initialize database");
    
    // Create auth service
    let auth_service: Box<dyn AuthServiceTrait> = Box::new(
        AuthService::new(pool.clone(), config.jwt_secret.clone())
            .with_expiration(config.jwt_expiration_hours)
    );
    let auth_service = web::Data::new(auth_service);
    
    tracing::info!("Starting server at {}:{}", config.host, config.port);
    
    // Start HTTP server
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allowed_origin("http://localhost:8081")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec!["Authorization", "Content-Type"])
            .supports_credentials()
            .max_age(3600);
            
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(auth_service.clone())
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                // Custom JSON error handler for deserialization errors
                let error_message = if err.to_string().contains("invalid digit") 
                    || err.to_string().contains("invalid type") 
                    || err.to_string().contains("expected") {
                    "The amount is invalid"
                } else {
                    "Invalid request data"
                };
                
                actix_web::error::InternalError::from_response(
                    err,
                    HttpResponse::BadRequest().json(json!({
                        "status": "400",
                        "message": error_message
                    }))
                ).into()
            }))
            .configure(configure_routes)
    })
    .bind((config.host.as_str(), config.port))?
    .run()
    .await
}
