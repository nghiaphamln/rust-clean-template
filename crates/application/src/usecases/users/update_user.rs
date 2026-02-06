use rust_clean_domain::{DomainError, UserRepository};
use std::sync::Arc;
use uuid::Uuid;

use crate::dto::UserResponse;

pub struct UpdateUserUseCase {
    user_repository: Arc<dyn UserRepository>,
}

impl UpdateUserUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, id: Uuid, name: String) -> Result<UserResponse, DomainError> {
        let mut user = self
            .user_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("User not found".to_string()))?;

        user.name = name;
        let updated_user = self.user_repository.update(&user).await?;
        Ok(UserResponse::from(&updated_user))
    }
}
