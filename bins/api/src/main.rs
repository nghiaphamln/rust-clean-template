use std::env;
use std::net::SocketAddr;
use tracing::{info, level_filters::LevelFilter};
use dotenvy::dotenv;
use actix_web::{HttpServer, App, web, middleware as actix_middleware};
use actix_cors::Cors;

use rust_clean_api::handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();

    let host = env::var("API_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("API_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);

    let addr = SocketAddr::new(host.parse().unwrap(), port);
    info!("Starting API server on {}", addr);

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .wrap(actix_middleware::Logger::default())
            .service(
                web::scope("/auth")
                    .service(handlers::auth_handler::register)
                    .service(handlers::auth_handler::login)
            )
            .service(
                web::scope("/users")
                    .service(handlers::user_handler::get_users)
                    .service(handlers::user_handler::get_user_by_id)
            )
    })
        .bind(addr)?
        .workers(4)
        .run()
        .await
}
