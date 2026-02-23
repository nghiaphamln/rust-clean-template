use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RefreshTokenModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl RefreshTokenModel {
    pub fn to_domain(&self) -> rust_clean_domain::RefreshToken {
        rust_clean_domain::RefreshToken {
            id: self.id,
            user_id: self.user_id,
            token_hash: self.token_hash.clone(),
            expires_at: self.expires_at,
            created_at: self.created_at,
        }
    }

    pub fn from_domain(token: &rust_clean_domain::RefreshToken) -> Self {
        Self {
            id: token.id,
            user_id: token.user_id,
            token_hash: token.token_hash.clone(),
            expires_at: token.expires_at,
            created_at: token.created_at,
        }
    }
}
