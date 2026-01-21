use actix_web::web;

use crate::api::handlers::user_handler;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/resources/users")
            // User endpoints
            .route("", web::get().to(user_handler::get_users))
            .route("/{login}", web::put().to(user_handler::update_user))
            .route("/{login}", web::delete().to(user_handler::delete_user))
            
            // Wallet endpoints
            .route("/{login}/wallets", web::get().to(user_handler::get_wallets))
            .route("/{login}/wallets", web::post().to(user_handler::create_wallet))
            .route("/{login}/wallets/{id}/summary", web::get().to(user_handler::get_summary))
            
            // Expense endpoints
            .route("/{login}/wallets/{id}/expenses", web::get().to(user_handler::get_expenses))
            .route("/{login}/wallets/{id}/expenses", web::post().to(user_handler::create_expense))
            .route("/{login}/wallets/{id}/highest_expense", web::get().to(user_handler::get_highest_expense))
            .route("/{login}/wallets/{wallet_id}/expenses/{expense_id}", web::delete().to(user_handler::delete_expense))
            .route("/{login}/wallets/{id}/counted_categories", web::get().to(user_handler::get_counted_categories))
            
            // Budget endpoints
            .route("/{login}/budgets", web::get().to(user_handler::get_budgets))
            .route("/{login}/budgets", web::post().to(user_handler::create_budget))
    );
}
