use async_trait::async_trait;
use uuid::Uuid;

use crate::{DomainError, RefreshToken};

#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    async fn create(&self, token: &RefreshToken) -> Result<RefreshToken, DomainError>;
    async fn find_by_hash(&self, token_hash: &str) -> Result<Option<RefreshToken>, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
    async fn delete_by_user_id(&self, user_id: Uuid) -> Result<(), DomainError>;
}
