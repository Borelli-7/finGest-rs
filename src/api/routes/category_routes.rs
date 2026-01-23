use actix_web::web;

use crate::api::handlers::category_handler;

pub fn category_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/resources/categories")
            .route("", web::get().to(category_handler::get_categories))
            .route("", web::post().to(category_handler::create_category))
            .route("/{name}/{profit}", web::put().to(category_handler::update_category))
    );
}
