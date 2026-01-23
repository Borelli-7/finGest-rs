use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use validator::Validate;

use crate::{
    errors::AppError,
    models::CreateCategoryDto,
    services::CategoryService,
    services::category_service::CategoryServiceTrait,
};

pub async fn get_categories(pool: web::Data<PgPool>) -> Result<impl Responder, AppError> {
    let category_service = CategoryService::new(pool.get_ref().clone());
    let categories = category_service.get_categories().await?;
    
    Ok(HttpResponse::Ok().json(categories))
}

pub async fn create_category(
    pool: web::Data<PgPool>,
    body: web::Json<CreateCategoryDto>,
) -> Result<impl Responder, AppError> {
    // Validate input
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let category_service = CategoryService::new(pool.get_ref().clone());
    let category = category_service.create_category(body.into_inner()).await?;
    
    Ok(HttpResponse::Created().json(category))
}
