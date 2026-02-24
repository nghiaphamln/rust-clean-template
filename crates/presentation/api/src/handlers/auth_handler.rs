use actix_web::{post, rt::spawn, web, HttpRequest, HttpResponse};

use crate::state::AppState;
use rust_clean_application::dto::{
    ErrorResponse, LoginRequest, RefreshTokenRequest, RegisterRequest,
};
use rust_clean_application::error_types::AppError;

#[derive(Debug)]
pub struct ApiError(AppError);

impl ApiError {
    pub fn new(app_error: AppError) -> Self {
        ApiError(app_error)
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ApiError {}

impl actix_web::ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match &self.0 {
            AppError::ValidationError(_) => actix_web::http::StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => actix_web::http::StatusCode::UNAUTHORIZED,
            AppError::NotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
            AppError::Conflict(_) => actix_web::http::StatusCode::CONFLICT,
            AppError::TooManyRequests(_) => actix_web::http::StatusCode::TOO_MANY_REQUESTS,
            AppError::InternalError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
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

impl From<AppError> for ApiError {
    fn from(e: AppError) -> Self {
        ApiError(e)
    }
}

impl From<rust_clean_domain::DomainError> for ApiError {
    fn from(e: rust_clean_domain::DomainError) -> Self {
        ApiError(AppError::from(e))
    }
}

#[post("/register")]
pub async fn register(
    state: web::Data<AppState>,
    dto: web::Json<RegisterRequest>,
) -> Result<HttpResponse, ApiError> {
    let request = RegisterRequest {
        email: dto.email.clone(),
        password: dto.password.clone(),
        name: dto.name.clone(),
    };

    let user = state.auth.register_user.execute(request).await?;

    Ok(HttpResponse::Created().json(rust_clean_application::dto::UserResponse::from(&user)))
}

#[post("/login")]
pub async fn login(
    state: web::Data<AppState>,
    dto: web::Json<LoginRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
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
        .brute_force
        .is_locked(&dto.email, &ip_address)
        .await?;

    if locked {
        return Err(ApiError(AppError::Unauthorized(
            "Account temporarily locked due to too many failed login attempts".to_string(),
        )));
    }

    let brute_force_config = crate::middleware::BruteForceConfig::from_env();

    let token_response = state.auth.login_user.execute(request).await;

    match token_response {
        Ok(response) => {
            let brute_force = state.auth.brute_force.clone();
            let email = dto.email.clone();
            spawn(async move {
                let _ = brute_force.clear_failures(&email).await;
            });

            Ok(HttpResponse::Ok().json(&response))
        }
        Err(e) => {
            let brute_force = state.auth.brute_force.clone();
            let email = dto.email.clone();
            let ip_address = ip_address.clone();
            let brute_force_config = brute_force_config.clone();
            spawn(async move {
                let _ = brute_force
                    .record_failure(
                        &email,
                        &ip_address,
                        brute_force_config.max_login_attempts,
                        brute_force_config.lockout_duration_minutes,
                    )
                    .await;
            });

            Err(ApiError::from(AppError::from(e)))
        }
    }
}

#[post("/refresh")]
pub async fn refresh(
    state: web::Data<AppState>,
    req: HttpRequest,
    _dto: web::Json<RefreshTokenRequest>,
) -> Result<HttpResponse, ApiError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            ApiError(AppError::Unauthorized(
                "Missing authorization header".to_string(),
            ))
        })?;

    if !auth_header.starts_with("Bearer ") {
        return Err(ApiError(AppError::Unauthorized(
            "Invalid token format".to_string(),
        )));
    }

    let token = &auth_header[7..];

    let token_response = state.auth.refresh_token.execute(token).await?;

    Ok(HttpResponse::Ok().json(&token_response))
}
