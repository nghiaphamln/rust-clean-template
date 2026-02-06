pub mod auth_routes;
pub mod user_routes;

pub use auth_routes::*;
pub use user_routes::*;

use actix_web::web;

use crate::handlers::{auth_handler, user_handler};
use crate::middleware::JwtMiddleware;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(auth_handler::register)
            .service(auth_handler::login)
            .service(auth_handler::refresh)
    );

    cfg.service(
        web::scope("/users")
            .wrap(JwtMiddleware)
            .service(user_handler::get_users)
            .service(user_handler::get_user_by_id)
            .service(user_handler::update_user)
            .service(user_handler::delete_user)
    );
}
