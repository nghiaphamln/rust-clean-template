use std::sync::Arc;

use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand::Rng;
use rust_clean_application::abstractions::TokenProvider;
use rust_clean_application::dto::{TokenClaims, TokenResponse};
use rust_clean_domain::{DomainError, RefreshToken, RefreshTokenRepository, User};
use sha2::Digest;

pub struct JwtTokenProvider {
    expiry_hours: i64,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    refresh_token_repo: Arc<dyn RefreshTokenRepository>,
}

impl JwtTokenProvider {
    pub fn new(
        secret: String,
        expiry_hours: i64,
        refresh_token_repo: Arc<dyn RefreshTokenRepository>,
    ) -> Self {
        let encoding_key = EncodingKey::from_secret(secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());
        Self {
            expiry_hours,
            encoding_key,
            decoding_key,
            refresh_token_repo,
        }
    }

    fn generate_refresh_token_string(&self) -> String {
        let random_bytes: [u8; 32] = rand::thread_rng().gen();
        hex::encode(random_bytes)
    }

    fn hash_token(&self, token: &str) -> String {
        let hash = sha2::Sha256::digest(token.as_bytes());
        hex::encode(hash)
    }
}

#[async_trait]
impl TokenProvider for JwtTokenProvider {
    fn generate_tokens(&self, user: &User) -> Result<TokenResponse, DomainError> {
        let now = Utc::now();
        let exp = now + Duration::hours(self.expiry_hours);

        let claims = TokenClaims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            role: user.role.to_string(),
            exp: exp.timestamp() as usize,
        };

        let access_token = encode(&Header::new(Algorithm::HS256), &claims, &self.encoding_key)
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        let refresh_token_string = self.generate_refresh_token_string();
        let refresh_token_hash = self.hash_token(&refresh_token_string);
        let expires_at = now + Duration::hours(self.expiry_hours * 7);

        let refresh_token = RefreshToken::new(user.id, refresh_token_hash, expires_at);

        let rt_repo = self.refresh_token_repo.clone();
        tokio::runtime::Handle::current()
            .block_on(rt_repo.create(&refresh_token))
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        let expires_in = (self.expiry_hours * 3600) as u64;

        Ok(TokenResponse::new(
            access_token,
            refresh_token_string,
            expires_in,
        ))
    }

    fn verify_token(&self, token: &str) -> Result<TokenClaims, DomainError> {
        let validation = Validation::new(Algorithm::HS256);
        jsonwebtoken::decode(token, &self.decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|_| DomainError::Unauthorized("Invalid token".to_string()))
    }

    fn refresh_tokens(&self, refresh_token: &str) -> Result<TokenResponse, DomainError> {
        let refresh_token_hash = self.hash_token(refresh_token);

        let rt_repo = self.refresh_token_repo.clone();
        let token_opt = tokio::runtime::Handle::current()
            .block_on(rt_repo.find_by_hash(&refresh_token_hash))
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        let token = token_opt
            .ok_or_else(|| DomainError::Unauthorized("Invalid refresh token".to_string()))?;

        if token.is_expired() {
            return Err(DomainError::Unauthorized(
                "Refresh token expired".to_string(),
            ));
        }

        let user_id = token.user_id;

        let rt_repo = self.refresh_token_repo.clone();
        tokio::runtime::Handle::current()
            .block_on(rt_repo.delete(token.id))
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        let now = Utc::now();
        let exp = now + Duration::hours(self.expiry_hours);

        let claims = TokenClaims {
            sub: user_id.to_string(),
            email: String::new(),
            role: String::new(),
            exp: exp.timestamp() as usize,
        };

        let access_token = encode(&Header::new(Algorithm::HS256), &claims, &self.encoding_key)
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        let new_refresh_token_string = self.generate_refresh_token_string();
        let new_refresh_token_hash = self.hash_token(&new_refresh_token_string);
        let expires_at = now + Duration::hours(self.expiry_hours * 7);

        let new_refresh_token = RefreshToken::new(user_id, new_refresh_token_hash, expires_at);

        let rt_repo = self.refresh_token_repo.clone();
        tokio::runtime::Handle::current()
            .block_on(rt_repo.create(&new_refresh_token))
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        let expires_in = (self.expiry_hours * 3600) as u64;

        Ok(TokenResponse::new(
            access_token,
            new_refresh_token_string,
            expires_in,
        ))
    }
}
