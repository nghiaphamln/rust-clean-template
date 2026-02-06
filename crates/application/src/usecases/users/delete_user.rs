use std::sync::Arc;
use rust_clean_domain::{DomainError, UserRepository};
use uuid::Uuid;

pub struct DeleteUserUseCase {
    user_repository: Arc<dyn UserRepository>,
}

impl DeleteUserUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), DomainError> {
        self.user_repository.delete(id).await
    }
}
