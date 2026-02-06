use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand::Rng;
use rust_clean_application::abstractions::TokenProvider;
use rust_clean_application::dto::{TokenClaims, TokenResponse};
use rust_clean_domain::{DomainError, User};
use sha2::Digest;

pub struct JwtTokenProvider {
    expiry_hours: i64,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtTokenProvider {
    pub fn new(secret: String, expiry_hours: i64) -> Self {
        let encoding_key = EncodingKey::from_secret(secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());
        Self {
            expiry_hours,
            encoding_key,
            decoding_key,
        }
    }

    fn generate_refresh_token(&self) -> String {
        let random_bytes: [u8; 32] = rand::thread_rng().gen();
        let token = hex::encode(random_bytes);
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

        let refresh_token = self.generate_refresh_token();
        let expires_in = (self.expiry_hours * 3600) as u64;

        Ok(TokenResponse::new(access_token, refresh_token, expires_in))
    }

    fn verify_token(&self, token: &str) -> Result<TokenClaims, DomainError> {
        let validation = Validation::new(Algorithm::HS256);
        jsonwebtoken::decode(token, &self.decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|_| DomainError::Unauthorized("Invalid token".to_string()))
    }

    fn refresh_tokens(&self, claims: &TokenClaims) -> Result<TokenResponse, DomainError> {
        let now = Utc::now();
        let exp = now + Duration::hours(self.expiry_hours);

        let new_claims = TokenClaims {
            sub: claims.sub.clone(),
            email: claims.email.clone(),
            role: claims.role.clone(),
            exp: exp.timestamp() as usize,
        };

        let access_token = encode(
            &Header::new(Algorithm::HS256),
            &new_claims,
            &self.encoding_key,
        )
        .map_err(|e| DomainError::InternalError(e.to_string()))?;

        let refresh_token = self.generate_refresh_token();
        let expires_in = (self.expiry_hours * 3600) as u64;

        Ok(TokenResponse::new(access_token, refresh_token, expires_in))
    }
}
