use actix_web::{web, HttpResponse};

use crate::dto::{RegisterRequest, LoginRequest};
use crate::state::AppState;
use crate::handlers::HandlerError;

pub async fn register(
    state: web::Data<AppState>,
    dto: web::Json<RegisterRequest>,
) -> Result<HttpResponse, HandlerError> {
    let request = RegisterRequest {
        email: dto.email.clone(),
        password: dto.password.clone(),
        name: dto.name.clone(),
    };

    let user = state.auth_service.register(request).await?;
    
    Ok(HttpResponse::Created().json(&crate::dto::UserResponse::from(&user)))
}

pub async fn login(
    state: web::Data<AppState>,
    dto: web::Json<LoginRequest>,
) -> Result<HttpResponse, HandlerError> {
    let request = LoginRequest {
        email: dto.email.clone(),
        password: dto.password.clone(),
    };

    let token_response = state.auth_service.login(request).await?;
    
    Ok(HttpResponse::Ok().json(&token_response))
}
