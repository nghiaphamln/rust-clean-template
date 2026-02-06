use std::sync::Arc;

use bcrypt::verify;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header, Algorithm};
use rand::Rng;
use sha2::Digest;
use thiserror::Error;
use uuid::Uuid;

use rust_clean_domain::{User, UserRepository, DomainError};
use crate::dto::{TokenClaims, TokenResponse, RegisterRequest, LoginRequest};

#[derive(Error, Debug)]
pub enum AuthServiceError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Token generation failed: {0}")]
    TokenGenerationError(String),
}

pub struct AuthService<T: UserRepository> {
    repository: Arc<T>,
    jwt_secret: String,
    jwt_expiry_hours: i64,
    encoding_key: EncodingKey,
}

impl<T: UserRepository> AuthService<T> {
    pub fn new(
        repository: Arc<T>,
        jwt_secret: String,
        jwt_expiry_hours: i64,
    ) -> Self {
        let secret = jwt_secret.clone();
        Self {
            repository,
            jwt_secret,
            jwt_expiry_hours,
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
        }
    }

    pub async fn register(&self, request: RegisterRequest) -> Result<User, DomainError> {
        let existing_user = self.repository.find_by_email(&request.email).await?;
        if existing_user.is_some() {
            return Err(DomainError::Conflict("Email already exists".to_string()));
        }

        let password_hash = bcrypt::hash(&request.password, bcrypt::DEFAULT_COST)
            .map_err(|e| DomainError::ValidationError(e.to_string()))?;

        let user = User::new(request.email, password_hash, request.name)?;
        let created_user = self.repository.create(&user).await?;
        
        Ok(created_user)
    }

    pub async fn login(&self, request: LoginRequest) -> Result<TokenResponse, DomainError> {
        let user = self.repository.find_by_email(&request.email)
            .await?
            .ok_or_else(|| DomainError::Unauthorized("Invalid credentials".to_string()))?;

        let is_valid = verify(&request.password, &user.password_hash)
            .map_err(|_| DomainError::Unauthorized("Invalid credentials".to_string()))?;

        if !is_valid {
            return Err(DomainError::Unauthorized("Invalid credentials".to_string()));
        }

        let now = Utc::now();
        let exp = now + Duration::hours(self.jwt_expiry_hours);

        let claims = TokenClaims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            role: user.role.to_string(),
            exp: exp.timestamp() as usize,
        };

        let access_token = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &self.encoding_key,
        ).map_err(|e| DomainError::InternalError(e.to_string()))?;

        let refresh_token = self.generate_refresh_token();
        let expires_in = (self.jwt_expiry_hours * 3600) as u64;

        Ok(TokenResponse::new(access_token, refresh_token, expires_in))
    }

    fn generate_refresh_token(&self) -> String {
        let random_bytes: [u8; 32] = rand::thread_rng().gen();
        let token = hex::encode(random_bytes);
        let hash = sha2::Sha256::digest(token.as_bytes());
        hex::encode(hash)
    }

    pub fn verify_token(&self, token: &str) -> Result<TokenClaims, DomainError> {
        let decoding_key = jsonwebtoken::DecodingKey::from_secret(self.jwt_secret.as_bytes());
        let validation = jsonwebtoken::Validation::new(Algorithm::HS256);

        jsonwebtoken::decode(token, &decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|_| DomainError::Unauthorized("Invalid token".to_string()))
    }

    pub fn refresh_token(&self, claims: TokenClaims) -> Result<TokenResponse, DomainError> {
        // Generate new access token with same claims but new expiry
        let now = Utc::now();
        let exp = now + Duration::hours(self.jwt_expiry_hours);

        let new_claims = TokenClaims {
            sub: claims.sub,
            email: claims.email,
            role: claims.role,
            exp: exp.timestamp() as usize,
        };

        let access_token = encode(
            &Header::new(Algorithm::HS256),
            &new_claims,
            &self.encoding_key,
        ).map_err(|e| DomainError::InternalError(e.to_string()))?;

        let refresh_token = self.generate_refresh_token();
        let expires_in = (self.jwt_expiry_hours * 3600) as u64;

        Ok(TokenResponse::new(access_token, refresh_token, expires_in))
    }
}

