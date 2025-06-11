use actix_web::web;

use crate::api::handlers::category_handler;

pub fn category_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/resources/categories")
            .route("", web::get().to(category_handler::get_categories))
    );
}
