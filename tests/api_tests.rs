//! Integration tests for the Money Manager API

use actix_web::{test, web, App};
use async_trait::async_trait;
use money_manager_api::{
    models::Category,
    services::CategoryServiceTrait,
    errors::AppError,
};

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

#[actix_rt::test]
async fn test_get_categories_handler() {
    // Arrange
    let mock_service = MockCategoryService;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_service))
            .configure(|cfg| {
                cfg.service(
                    web::scope("/resources/categories")
                        .route("", web::get().to(|svc: web::Data<MockCategoryService>| async move {
                            let categories = svc.get_categories().await.unwrap();
                            web::Json(categories)
                        }))
                );
            })
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
