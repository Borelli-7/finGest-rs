use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use money_manager_api::{
    config::Config, 
    db::init_db,
    api::routes::configure_routes
};
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
            .configure(configure_routes)
    })
    .bind((config.host.as_str(), config.port))?
    .run()
    .await
}
