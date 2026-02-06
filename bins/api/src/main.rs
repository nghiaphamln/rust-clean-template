use actix_cors::Cors;
use actix_web::{middleware as actix_middleware, web, App, HttpServer};
use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{info, level_filters::LevelFilter};

use rust_clean_api::{handlers, state::AppState};
use rust_clean_application::usecases::auth::{LoginUseCase, RefreshTokenUseCase, RegisterUserUseCase};
use rust_clean_application::usecases::users::{
    DeleteUserUseCase, GetUserByIdUseCase, GetUsersUseCase, UpdateUserUseCase,
};
use rust_clean_infrastructure::{BcryptHasher, Database, JwtTokenProvider, PgUserRepository};

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

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let jwt_expiry_hours = env::var("JWT_EXPIRY_HOURS")
        .unwrap_or_else(|_| "24".to_string())
        .parse::<i64>()
        .expect("JWT_EXPIRY_HOURS must be a valid integer");

    // Initialize Infrastructure
    let database = Database::new(&database_url, 5)
        .await
        .expect("Failed to initialize database");
    let user_repository = Arc::new(PgUserRepository::new(database.pool().clone()));
    let password_hasher = Arc::new(BcryptHasher::new());
    let token_provider = Arc::new(JwtTokenProvider::new(jwt_secret, jwt_expiry_hours));

    // Initialize Use Cases
    let register_user = Arc::new(RegisterUserUseCase::new(
        user_repository.clone(),
        password_hasher.clone(),
    ));
    let login_user = Arc::new(LoginUseCase::new(
        user_repository.clone(),
        password_hasher.clone(),
        token_provider.clone(),
    ));
    let refresh_token = Arc::new(RefreshTokenUseCase::new(token_provider.clone()));
    let get_users = Arc::new(GetUsersUseCase::new(user_repository.clone()));
    let get_user_by_id = Arc::new(GetUserByIdUseCase::new(user_repository.clone()));
    let update_user = Arc::new(UpdateUserUseCase::new(user_repository.clone()));
    let delete_user = Arc::new(DeleteUserUseCase::new(user_repository.clone()));

    // Initialize Presentation State
    let app_state = web::Data::new(AppState::new(
        register_user,
        login_user,
        refresh_token,
        get_users,
        get_user_by_id,
        update_user,
        delete_user,
        token_provider.clone(),
    ));

    let addr = SocketAddr::new(host.parse().unwrap(), port);
    info!("Starting API server on {}", addr);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Cors::permissive())
            .wrap(actix_middleware::Logger::default())
            .service(
                web::scope("/auth")
                    .service(handlers::auth_handler::register)
                    .service(handlers::auth_handler::login)
                    .service(handlers::auth_handler::refresh),
            )
            .service(
                web::scope("/users")
                    .service(handlers::user_handler::get_users)
                    .service(handlers::user_handler::get_user_by_id)
                    .service(handlers::user_handler::update_user)
                    .service(handlers::user_handler::delete_user),
            )
    })
    .bind(addr)?
    .workers(4)
    .run()
    .await
}
