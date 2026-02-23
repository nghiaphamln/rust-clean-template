use async_trait::async_trait;
use mockall::mock;
use rust_clean_application::abstractions::{PasswordHasher, TokenProvider};
use rust_clean_application::dto::{TokenClaims, TokenResponse};
use rust_clean_domain::{DomainError, User, UserRepository};
use uuid::Uuid;

mock! {
    pub UserRepository {}
    #[async_trait]
    impl UserRepository for UserRepository {
        async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
        async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;
        async fn find_all(&self) -> Result<Vec<User>, DomainError>;
        async fn create(&self, user: &User) -> Result<User, DomainError>;
        async fn update(&self, user: &User) -> Result<User, DomainError>;
        async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
    }
}

mock! {
    pub PasswordHasher {}
    #[async_trait]
    impl PasswordHasher for PasswordHasher {
        async fn hash(&self, password: &str) -> Result<String, DomainError>;
        async fn verify(&self, password: &str, hash: &str) -> Result<bool, DomainError>;
    }
}

mock! {
    pub TokenProvider {}
    #[async_trait]
    impl TokenProvider for TokenProvider {
         fn generate_tokens(&self, user: &User) -> Result<TokenResponse, DomainError>;
         fn verify_token(&self, token: &str) -> Result<TokenClaims, DomainError>;
         fn refresh_tokens(&self, refresh_token: &str) -> Result<TokenResponse, DomainError>;
    }
}
