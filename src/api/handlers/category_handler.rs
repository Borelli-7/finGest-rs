use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::{
    errors::AppError,
    services::CategoryService,
    services::category_service::CategoryServiceTrait,
};

pub async fn get_categories(pool: web::Data<PgPool>) -> Result<impl Responder, AppError> {
    let category_service = CategoryService::new(pool.get_ref().clone());
    let categories = category_service.get_categories().await?;
    
    Ok(HttpResponse::Ok().json(categories))
}
