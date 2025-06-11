use actix_web::web;

mod category_routes;
mod user_routes;

pub use category_routes::*;
pub use user_routes::*;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // Configure routes for different API endpoints
    category_routes(cfg);
    user_routes(cfg);
}
