use std::sync::Arc;

use rust_clean_infrastructure::Database;
use rust_clean_application::{AuthService, UserService};

pub struct AppState {
    pub database: Arc<Database>,
    pub auth_service: Arc<AuthService<rust_clean_infrastructure::PgUserRepository>>,
    pub user_service: Arc<UserService<rust_clean_infrastructure::PgUserRepository>>,
}

impl AppState {
    pub async fn new(
        database: Database,
        jwt_secret: String,
        jwt_expiry_hours: i64,
    ) -> Self {
        let repository = Arc::new(rust_clean_infrastructure::PgUserRepository::new(
            database.pool().clone(),
        ));

        let auth_service = Arc::new(AuthService::new(
            repository.clone(),
            jwt_secret,
            jwt_expiry_hours,
        ));

        let user_service = Arc::new(UserService::new(repository));

        Self {
            database: Arc::new(database),
            auth_service,
            user_service,
        }
    }
}
