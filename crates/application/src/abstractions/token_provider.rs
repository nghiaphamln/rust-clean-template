use async_trait::async_trait;
use rust_clean_domain::{DomainError, User};

use crate::dto::{TokenClaims, TokenResponse};

#[async_trait]
pub trait TokenProvider: Send + Sync {
    fn generate_tokens(&self, user: &User) -> Result<TokenResponse, DomainError>;
    fn verify_token(&self, token: &str) -> Result<TokenClaims, DomainError>;
    fn refresh_tokens(&self, claims: &TokenClaims) -> Result<TokenResponse, DomainError>;
}
