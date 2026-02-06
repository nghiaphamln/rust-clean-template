use rust_clean_domain::{DomainError, User, UserRepository};
use std::sync::Arc;

use crate::abstractions::PasswordHasher;
use crate::dto::RegisterRequest;

pub struct RegisterUserUseCase {
    user_repository: Arc<dyn UserRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
}

impl RegisterUserUseCase {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        password_hasher: Arc<dyn PasswordHasher>,
    ) -> Self {
        Self {
            user_repository,
            password_hasher,
        }
    }

    pub async fn execute(&self, request: RegisterRequest) -> Result<User, DomainError> {
        let existing_user = self.user_repository.find_by_email(&request.email).await?;
        if existing_user.is_some() {
            return Err(DomainError::Conflict("Email already exists".to_string()));
        }

        let password_hash = self.password_hasher.hash(&request.password).await?;

        let user = User::new(request.email, password_hash, request.name)?;
        let created_user = self.user_repository.create(&user).await?;

        Ok(created_user)
    }
}
