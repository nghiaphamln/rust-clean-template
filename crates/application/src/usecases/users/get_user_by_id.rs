use crate::dto::UserResponse;
use rust_clean_domain::{DomainError, UserRepository};
use std::sync::Arc;
use uuid::Uuid;

pub struct GetUserByIdUseCase {
    user_repository: Arc<dyn UserRepository>,
}

impl GetUserByIdUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, id: Uuid) -> Result<UserResponse, DomainError> {
        let user = self
            .user_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("User not found".to_string()))?;
        Ok(UserResponse::from(&user))
    }
}
