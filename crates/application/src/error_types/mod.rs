use rust_clean_domain::DomainError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Too many requests: {0}")]
    TooManyRequests(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<DomainError> for AppError {
    fn from(e: DomainError) -> Self {
        match e {
            DomainError::ValidationError(msg) => AppError::ValidationError(msg),
            DomainError::Unauthorized(msg) => AppError::Unauthorized(msg),
            DomainError::NotFound(msg) => AppError::NotFound(msg),
            DomainError::Conflict(msg) => AppError::Conflict(msg),
            DomainError::DatabaseError(msg) => AppError::ValidationError(msg),
            DomainError::InternalError(msg) => AppError::InternalError(msg),
        }
    }
}
