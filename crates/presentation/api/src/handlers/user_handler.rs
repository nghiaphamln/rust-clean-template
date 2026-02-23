use actix_web::{delete, get, put, web, HttpResponse};
use thiserror::Error;
use uuid::Uuid;

use crate::state::AppState;
use rust_clean_application::dto::{ErrorResponse, UserUpdateRequest};
use rust_clean_domain::DomainError;

#[derive(Error, Debug)]
pub enum UserHandlerError {
    #[error("User not found")]
    NotFound,
    #[error("Validation error: {0}")]
    ValidationError(String),
}

impl actix_web::ResponseError for UserHandlerError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            UserHandlerError::NotFound => actix_web::http::StatusCode::NOT_FOUND,
            UserHandlerError::ValidationError(_) => actix_web::http::StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let status_code = self.status_code();
        let (error, message) = match self {
            UserHandlerError::NotFound => ("Not Found".to_string(), "User not found".to_string()),
            UserHandlerError::ValidationError(msg) => ("Validation Error".to_string(), msg.clone()),
        };

        HttpResponse::build(status_code).json(ErrorResponse {
            error,
            message,
            status_code: status_code.as_u16(),
        })
    }
}

impl From<DomainError> for UserHandlerError {
    fn from(e: DomainError) -> Self {
        match e {
            DomainError::NotFound(_) => UserHandlerError::NotFound,
            _ => UserHandlerError::NotFound,
        }
    }
}

#[get("")]
pub async fn get_users(state: web::Data<AppState>) -> Result<HttpResponse, UserHandlerError> {
    let users = state
        .users
        .get_users
        .execute()
        .await
        .map_err(UserHandlerError::from)?;

    Ok(HttpResponse::Ok().json(users))
}

#[get("/{id}")]
pub async fn get_user_by_id(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, UserHandlerError> {
    let user = state
        .users
        .get_user_by_id
        .execute(id.into_inner())
        .await
        .map_err(UserHandlerError::from)?;

    Ok(HttpResponse::Ok().json(user))
}

#[put("/{id}")]
pub async fn update_user(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    req: web::Json<UserUpdateRequest>,
) -> Result<HttpResponse, UserHandlerError> {
    let name = req.name.clone().ok_or(UserHandlerError::ValidationError(
        "Name is required".to_string(),
    ))?;

    let user = state
        .users
        .update_user
        .execute(id.into_inner(), name)
        .await
        .map_err(UserHandlerError::from)?;

    Ok(HttpResponse::Ok().json(user))
}

#[delete("/{id}")]
pub async fn delete_user(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, UserHandlerError> {
    state
        .users
        .delete_user
        .execute(id.into_inner())
        .await
        .map_err(UserHandlerError::from)?;

    Ok(HttpResponse::NoContent().finish())
}
