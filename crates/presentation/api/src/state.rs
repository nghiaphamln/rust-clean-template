use std::sync::Arc;

use rust_clean_application::abstractions::TokenProvider;
use rust_clean_application::usecases::auth::{
    LoginUseCase, RefreshTokenUseCase, RegisterUserUseCase,
};
use rust_clean_application::usecases::users::{
    DeleteUserUseCase, GetUserByIdUseCase, GetUsersUseCase, UpdateUserUseCase,
};
use rust_clean_domain::FailedLoginRepository;

pub struct AuthUseCases {
    pub register_user: Arc<RegisterUserUseCase>,
    pub login_user: Arc<LoginUseCase>,
    pub refresh_token: Arc<RefreshTokenUseCase>,
    pub token_provider: Arc<dyn TokenProvider>,
    pub failed_login_repo: Arc<dyn FailedLoginRepository>,
}

pub struct UserUseCases {
    pub get_users: Arc<GetUsersUseCase>,
    pub get_user_by_id: Arc<GetUserByIdUseCase>,
    pub update_user: Arc<UpdateUserUseCase>,
    pub delete_user: Arc<DeleteUserUseCase>,
}

pub struct AppState {
    pub auth: AuthUseCases,
    pub users: UserUseCases,
}

impl AppState {
    pub fn new(auth: AuthUseCases, users: UserUseCases) -> Self {
        Self { auth, users }
    }
}
