use actix_web::{web, HttpResponse, get, put, delete};
use thiserror::Error;
use uuid::Uuid;

use crate::dto::{UserResponse, ErrorResponse, UserUpdateRequest};
use crate::state::AppState;
use rust_clean_domain::DomainError;

#[derive(Error, Debug)]
pub enum UserHandlerError {
    #[error("User not found")]
    NotFound,
}

impl actix_web::ResponseError for UserHandlerError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::NOT_FOUND
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::NotFound().json(ErrorResponse {
            error: "Not Found".to_string(),
            message: "User not found".to_string(),
            status_code: 404,
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
pub async fn get_users(
    state: web::Data<AppState>,
) -> Result<HttpResponse, UserHandlerError> {
    let users = state.user_service.get_all_users().await.map_err(UserHandlerError::from)?;
    
    Ok(HttpResponse::Ok().json(users))
}

#[get("/{id}")]
pub async fn get_user_by_id(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, UserHandlerError> {
    let user = state.user_service.get_user_by_id(id.into_inner()).await.map_err(UserHandlerError::from)?;
    
    Ok(HttpResponse::Ok().json(user))
}

#[put("/{id}")]
pub async fn update_user(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    req: web::Json<UserUpdateRequest>,
) -> Result<HttpResponse, UserHandlerError> {
    let user = state.user_service.update_user(id.into_inner(), req.name.clone()).await.map_err(UserHandlerError::from)?;
    
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/{id}")]
pub async fn delete_user(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, UserHandlerError> {
    state.user_service.delete_user(id.into_inner()).await.map_err(UserHandlerError::from)?;
    
    Ok(HttpResponse::NoContent().finish())
}
