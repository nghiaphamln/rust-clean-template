use rust_clean_domain::DomainError;
use std::sync::Arc;

use crate::abstractions::TokenProvider;
use crate::dto::TokenResponse;

pub struct RefreshTokenUseCase {
    token_provider: Arc<dyn TokenProvider>,
}

impl RefreshTokenUseCase {
    pub fn new(token_provider: Arc<dyn TokenProvider>) -> Self {
        Self { token_provider }
    }

    pub async fn execute(&self, refresh_token: &str) -> Result<TokenResponse, DomainError> {
        self.token_provider.refresh_tokens(refresh_token).await
    }
}
