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

    pub fn execute(&self, token: &str) -> Result<TokenResponse, DomainError> {
        let claims = self.token_provider.verify_token(token)?;
        self.token_provider.refresh_tokens(&claims)
    }
}
