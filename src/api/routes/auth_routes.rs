use actix_web::web;

use crate::api::handlers::auth_handler::{login, register, verify_token};

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/verify", web::get().to(verify_token))
    );
}
