use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    components(
        schemas(
            rust_clean_application::dto::RegisterRequest,
            rust_clean_application::dto::LoginRequest,
            rust_clean_application::dto::TokenResponse,
            rust_clean_application::dto::UserResponse,
            rust_clean_application::dto::ErrorResponse,
        )
    ),
    tags(
        (name = "Auth", description = "Authentication endpoints"),
        (name = "Users", description = "User management endpoints"),
    )
)]
pub struct ApiDoc;

pub fn swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi())
}
