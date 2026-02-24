use actix_web::{delete, get, put, web, HttpResponse};
use uuid::Uuid;

use crate::handlers::auth_handler::ApiError;
use crate::state::AppState;
use rust_clean_application::dto::UserUpdateRequest;
use rust_clean_application::error_types::AppError;

#[get("")]
pub async fn get_users(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let users = state.users.get_users.execute().await?;
    Ok(HttpResponse::Ok().json(users))
}

#[get("/{id}")]
pub async fn get_user_by_id(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let user = state.users.get_user_by_id.execute(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(user))
}

#[put("/{id}")]
pub async fn update_user(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    req: web::Json<UserUpdateRequest>,
) -> Result<HttpResponse, ApiError> {
    let name = req
        .name
        .clone()
        .ok_or_else(|| ApiError::new(AppError::ValidationError("Name is required".to_string())))?;

    let user = state
        .users
        .update_user
        .execute(id.into_inner(), name)
        .await?;
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/{id}")]
pub async fn delete_user(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    state.users.delete_user.execute(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
