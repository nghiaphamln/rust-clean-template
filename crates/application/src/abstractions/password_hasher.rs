use async_trait::async_trait;
use rust_clean_domain::DomainError;

#[async_trait]
pub trait PasswordHasher: Send + Sync {
    async fn hash(&self, password: &str) -> Result<String, DomainError>;
    async fn verify(&self, password: &str, hash: &str) -> Result<bool, DomainError>;
}
