use std::sync::Arc;
use rust_clean_domain::{DomainError, UserRepository};
use crate::dto::UserResponse;

pub struct GetUsersUseCase {
    user_repository: Arc<dyn UserRepository>,
}

impl GetUsersUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self) -> Result<Vec<UserResponse>, DomainError> {
        let users = self.user_repository.find_all().await?;
        Ok(users.iter().map(UserResponse::from).collect())
    }
}
