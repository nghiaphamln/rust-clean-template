use crate::dto::UserResponse;
use rust_clean_domain::{DomainError, UserRepository};
use std::sync::Arc;

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
