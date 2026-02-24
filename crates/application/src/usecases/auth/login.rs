use rust_clean_domain::{DomainError, UserRepository};
use std::sync::Arc;

use crate::abstractions::{PasswordHasher, TokenProvider};
use crate::dto::{LoginRequest, TokenResponse};

pub struct LoginUseCase {
    user_repository: Arc<dyn UserRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
    token_provider: Arc<dyn TokenProvider>,
}

impl LoginUseCase {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        password_hasher: Arc<dyn PasswordHasher>,
        token_provider: Arc<dyn TokenProvider>,
    ) -> Self {
        Self {
            user_repository,
            password_hasher,
            token_provider,
        }
    }

    pub async fn execute(&self, request: LoginRequest) -> Result<TokenResponse, DomainError> {
        let user = self
            .user_repository
            .find_by_email(&request.email)
            .await?
            .ok_or_else(|| DomainError::Unauthorized("Invalid credentials".to_string()))?;

        let is_valid = self
            .password_hasher
            .verify(&request.password, &user.password_hash)
            .await?;

        if !is_valid {
            return Err(DomainError::Unauthorized("Invalid credentials".to_string()));
        }

        self.token_provider.generate_tokens(&user).await
    }
}
