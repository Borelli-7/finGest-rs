use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use validator::Validate;

use crate::{
    errors::AppError,
    models::{CreateCategoryDto, UpdateCategoryDto},
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

pub async fn update_category(
    pool: web::Data<PgPool>,
    path: web::Path<(String, bool)>,
    body: web::Json<UpdateCategoryDto>,
) -> Result<impl Responder, AppError> {
    let (name, profit) = path.into_inner();
    
    // Validate input
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let category_service = CategoryService::new(pool.get_ref().clone());
    let updated_category = category_service.update_category(name, profit, body.into_inner()).await?;
    
    Ok(HttpResponse::Ok().json(updated_category))
}

pub async fn delete_category(
    pool: web::Data<PgPool>,
    path: web::Path<(String, bool)>,
) -> Result<impl Responder, AppError> {
    let (name, profit) = path.into_inner();

    let category_service = CategoryService::new(pool.get_ref().clone());
    category_service.delete_category(name, profit).await?;
    
    Ok(HttpResponse::NoContent().finish())
}
