use actix_web::{post, rt::spawn, web, HttpRequest, HttpResponse};
use thiserror::Error;

use crate::state::AppState;
use rust_clean_application::dto::{
    ErrorResponse, LoginRequest, RefreshTokenRequest, RegisterRequest,
};
use rust_clean_domain::DomainError;

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),
}

impl actix_web::ResponseError for HandlerError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            HandlerError::ValidationError(_) => actix_web::http::StatusCode::BAD_REQUEST,
            HandlerError::AuthError(_) => actix_web::http::StatusCode::UNAUTHORIZED,
            HandlerError::NotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
            HandlerError::Conflict(_) => actix_web::http::StatusCode::CONFLICT,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            error: self.to_string(),
            message: self.to_string(),
            status_code: status_code.as_u16(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

impl From<DomainError> for HandlerError {
    fn from(e: DomainError) -> Self {
        match e {
            DomainError::ValidationError(msg) => HandlerError::ValidationError(msg),
            DomainError::Unauthorized(msg) => HandlerError::AuthError(msg),
            DomainError::NotFound(msg) => HandlerError::NotFound(msg),
            DomainError::Conflict(msg) => HandlerError::Conflict(msg),
            DomainError::DatabaseError(msg) => HandlerError::ValidationError(msg),
            DomainError::InternalError(msg) => HandlerError::ValidationError(msg),
        }
    }
}

#[post("/register")]
pub async fn register(
    state: web::Data<AppState>,
    dto: web::Json<RegisterRequest>,
) -> Result<HttpResponse, HandlerError> {
    let request = RegisterRequest {
        email: dto.email.clone(),
        password: dto.password.clone(),
        name: dto.name.clone(),
    };

    let user = state
        .auth
        .register_user
        .execute(request)
        .await
        .map_err(HandlerError::from)?;

    Ok(HttpResponse::Created().json(rust_clean_application::dto::UserResponse::from(&user)))
}

#[post("/login")]
pub async fn login(
    state: web::Data<AppState>,
    dto: web::Json<LoginRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, HandlerError> {
    let request = LoginRequest {
        email: dto.email.clone(),
        password: dto.password.clone(),
    };

    let ip_address = req
        .connection_info()
        .realip_remote_addr()
        .map(|s| s.to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let locked = state
        .auth
        .failed_login_repo
        .find_by_email_and_ip(&dto.email, &ip_address)
        .await
        .ok()
        .flatten()
        .is_some_and(|f| f.is_locked());

    if locked {
        return Err(HandlerError::AuthError(
            "Account temporarily locked due to too many failed login attempts".to_string(),
        ));
    }

    let brute_force_config = crate::middleware::BruteForceConfig::from_env();

    let token_response = state.auth.login_user.execute(request).await;

    match token_response {
        Ok(response) => {
            let failed_login_repo = state.auth.failed_login_repo.clone();
            let email = dto.email.clone();
            spawn(async move {
                let _ = failed_login_repo.delete(&email).await;
            });

            Ok(HttpResponse::Ok().json(&response))
        }
        Err(e) => {
            let failed_login_repo = state.auth.failed_login_repo.clone();
            let email = dto.email.clone();
            let ip_address = ip_address.clone();
            let brute_force_config = brute_force_config.clone();
            spawn(async move {
                let _ = failed_login_repo
                    .register_failed_attempt(
                        &email,
                        &ip_address,
                        brute_force_config.max_login_attempts,
                        brute_force_config.lockout_duration_minutes,
                    )
                    .await;
            });

            Err(HandlerError::from(e))
        }
    }
}

#[post("/refresh")]
pub async fn refresh(
    state: web::Data<AppState>,
    req: HttpRequest,
    _dto: web::Json<RefreshTokenRequest>,
) -> Result<HttpResponse, HandlerError> {
    // Extract the access token from Authorization header
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| HandlerError::AuthError("Missing authorization header".to_string()))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(HandlerError::AuthError("Invalid token format".to_string()));
    }

    let token = &auth_header[7..];

    let token_response = state
        .auth
        .refresh_token
        .execute(token)
        .map_err(HandlerError::from)?;

    Ok(HttpResponse::Ok().json(&token_response))
}
