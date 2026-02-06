use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::state::AppState;
use crate::dto::UserResponse;
use crate::handlers::HandlerError;

pub async fn get_users(
    state: web::Data<AppState>,
) -> Result<HttpResponse, HandlerError> {
    let users = state.user_service.get_all_users().await?;
    
    Ok(HttpResponse::Ok().json(users))
}

pub async fn get_user_by_id(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, HandlerError> {
    let user = state.user_service.get_user_by_id(id.into_inner()).await?;
    
    Ok(HttpResponse::Ok().json(user))
}
