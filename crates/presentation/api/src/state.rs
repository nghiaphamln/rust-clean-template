use std::sync::Arc;

use rust_clean_application::abstractions::TokenProvider;
use rust_clean_application::usecases::auth::{
    LoginUseCase, RefreshTokenUseCase, RegisterUserUseCase,
};
use rust_clean_application::usecases::users::{
    DeleteUserUseCase, GetUserByIdUseCase, GetUsersUseCase, UpdateUserUseCase,
};

pub struct AppState {
    pub register_user: Arc<RegisterUserUseCase>,
    pub login_user: Arc<LoginUseCase>,
    pub refresh_token: Arc<RefreshTokenUseCase>,
    pub get_users: Arc<GetUsersUseCase>,
    pub get_user_by_id: Arc<GetUserByIdUseCase>,
    pub update_user: Arc<UpdateUserUseCase>,
    pub delete_user: Arc<DeleteUserUseCase>,
    pub token_provider: Arc<dyn TokenProvider>,
}

impl AppState {
    pub fn new(
        register_user: Arc<RegisterUserUseCase>,
        login_user: Arc<LoginUseCase>,
        refresh_token: Arc<RefreshTokenUseCase>,
        get_users: Arc<GetUsersUseCase>,
        get_user_by_id: Arc<GetUserByIdUseCase>,
        update_user: Arc<UpdateUserUseCase>,
        delete_user: Arc<DeleteUserUseCase>,
        token_provider: Arc<dyn TokenProvider>,
    ) -> Self {
        Self {
            register_user,
            login_user,
            refresh_token,
            get_users,
            get_user_by_id,
            update_user,
            delete_user,
            token_provider,
        }
    }
}
