use async_trait::async_trait;
use rust_clean_domain::{DomainError, User};

use crate::dto::{TokenClaims, TokenResponse};

#[async_trait]
pub trait TokenProvider: Send + Sync {
    async fn generate_tokens(&self, user: &User) -> Result<TokenResponse, DomainError>;
    async fn verify_token(&self, token: &str) -> Result<TokenClaims, DomainError>;
    async fn refresh_tokens(&self, refresh_token: &str) -> Result<TokenResponse, DomainError>;
}
