use async_trait::async_trait;
use rust_clean_application::abstractions::PasswordHasher;
use rust_clean_domain::DomainError;

pub struct BcryptHasher;

impl BcryptHasher {
    pub fn new() -> Self {
        Self
    }
}

impl Default for BcryptHasher {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PasswordHasher for BcryptHasher {
    async fn hash(&self, password: &str) -> Result<String, DomainError> {
        bcrypt::hash(password, bcrypt::DEFAULT_COST)
            .map_err(|e| DomainError::ValidationError(e.to_string()))
    }

    async fn verify(&self, password: &str, hash: &str) -> Result<bool, DomainError> {
        bcrypt::verify(password, hash)
            .map_err(|_| DomainError::Unauthorized("Invalid credentials".to_string()))
    }
}
