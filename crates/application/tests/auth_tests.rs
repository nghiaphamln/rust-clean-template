use mockall::predicate::*;
use rust_clean_application::dto::{LoginRequest, RegisterRequest};
use rust_clean_application::usecases::auth::login::LoginUseCase;
use rust_clean_application::usecases::auth::refresh_token::RefreshTokenUseCase;
use rust_clean_application::usecases::auth::register::RegisterUserUseCase;
use rust_clean_domain::{DomainError, User};
use std::sync::Arc;

mod common;
use common::{MockPasswordHasher, MockTokenProvider, MockUserRepository};

#[tokio::test]
async fn test_register_success() {
    let mut mock_repo = MockUserRepository::new();
    let mut mock_hasher = MockPasswordHasher::new();

    let email = "test@example.com";
    let password = "password123";
    let name = "Test User";
    let hashed_password = "$2a$12$R9h/cIPz0gi.URNNX3kh2OPST9/PgBkqquii.V37Yo8ncRP4tYo.6";

    mock_repo
        .expect_find_by_email()
        .with(eq(email))
        .times(1)
        .returning(|_| Ok(None));

    mock_hasher
        .expect_hash()
        .with(eq(password))
        .times(1)
        .returning(move |_| Ok(hashed_password.to_string()));

    mock_repo
        .expect_create()
        .times(1)
        .returning(|u| Ok(u.clone()));

    let usecase = RegisterUserUseCase::new(Arc::new(mock_repo), Arc::new(mock_hasher));

    let request = RegisterRequest {
        email: email.to_string(),
        password: password.to_string(),
        name: name.to_string(),
    };

    let result = usecase.execute(request).await;

    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user.email, email);
    assert_eq!(user.name, name);
    assert_eq!(user.password_hash, hashed_password);
}

#[tokio::test]
async fn test_register_email_exists() {
    let mut mock_repo = MockUserRepository::new();
    let mock_hasher = MockPasswordHasher::new();

    let email = "existing@example.com";
    let existing_user = User::new(
        email.to_string(),
        "$2a$12$R9h/cIPz0gi.URNNX3kh2OPST9/PgBkqquii.V37Yo8ncRP4tYo.6".to_string(),
        "Existing".to_string(),
    )
    .unwrap();

    mock_repo
        .expect_find_by_email()
        .with(eq(email))
        .times(1)
        .returning(move |_| Ok(Some(existing_user.clone())));

    let usecase = RegisterUserUseCase::new(Arc::new(mock_repo), Arc::new(mock_hasher));

    let request = RegisterRequest {
        email: email.to_string(),
        password: "any".to_string(),
        name: "any".to_string(),
    };

    let result = usecase.execute(request).await;

    match result {
        Err(DomainError::Conflict(msg)) => assert_eq!(msg, "Email already exists"),
        _ => panic!("Expected Conflict error"),
    }
}

#[tokio::test]
async fn test_login_success() {
    let mut mock_repo = MockUserRepository::new();
    let mut mock_hasher = MockPasswordHasher::new();
    let mut mock_provider = MockTokenProvider::new();

    let email = "user@example.com";
    let password = "password";
    let hash = "$2a$12$R9h/cIPz0gi.URNNX3kh2OPST9/PgBkqquii.V37Yo8ncRP4tYo.6";
    let user = User::new(email.to_string(), hash.to_string(), "User".to_string()).unwrap();

    mock_repo
        .expect_find_by_email()
        .with(eq(email))
        .returning(move |_| Ok(Some(user.clone())));

    mock_hasher
        .expect_verify()
        .with(eq(password), eq(hash))
        .returning(|_, _| Ok(true));

    mock_provider.expect_generate_tokens().returning(|_| {
        Ok(rust_clean_application::dto::TokenResponse {
            access_token: "access".into(),
            refresh_token: "refresh".into(),
            token_type: "Bearer".into(),
            expires_in: 3600,
        })
    });

    let usecase = LoginUseCase::new(
        Arc::new(mock_repo),
        Arc::new(mock_hasher),
        Arc::new(mock_provider),
    );

    let result = usecase
        .execute(LoginRequest {
            email: email.into(),
            password: password.into(),
        })
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_login_user_not_found() {
    let mut mock_repo = MockUserRepository::new();
    let mock_hasher = MockPasswordHasher::new();
    let mock_provider = MockTokenProvider::new();

    mock_repo.expect_find_by_email().returning(|_| Ok(None));

    let usecase = LoginUseCase::new(
        Arc::new(mock_repo),
        Arc::new(mock_hasher),
        Arc::new(mock_provider),
    );

    let result = usecase
        .execute(LoginRequest {
            email: "unknown@example.com".into(),
            password: "pass".into(),
        })
        .await;

    match result {
        Err(DomainError::Unauthorized(_)) => {}
        _ => panic!("Expected Unauthorized"),
    }
}

#[tokio::test]
async fn test_login_invalid_password() {
    let mut mock_repo = MockUserRepository::new();
    let mut mock_hasher = MockPasswordHasher::new();
    let mock_provider = MockTokenProvider::new();

    let user = User::new(
        "e@e.com".into(),
        "$2a$12$R9h/cIPz0gi.URNNX3kh2OPST9/PgBkqquii.V37Yo8ncRP4tYo.6".into(),
        "u".into(),
    )
    .unwrap();

    mock_repo
        .expect_find_by_email()
        .returning(move |_| Ok(Some(user.clone())));

    mock_hasher.expect_verify().returning(|_, _| Ok(false));

    let usecase = LoginUseCase::new(
        Arc::new(mock_repo),
        Arc::new(mock_hasher),
        Arc::new(mock_provider),
    );

    let result = usecase
        .execute(LoginRequest {
            email: "e@e.com".into(),
            password: "wrong".into(),
        })
        .await;

    match result {
        Err(DomainError::Unauthorized(_)) => {}
        _ => panic!("Expected Unauthorized"),
    }
}

#[tokio::test]
async fn test_refresh_token_success() {
    let mut mock_provider = MockTokenProvider::new();

    mock_provider
        .expect_refresh_tokens()
        .with(eq("valid_refresh_token"))
        .times(1)
        .returning(|_| {
            Ok(rust_clean_application::dto::TokenResponse {
                access_token: "new_access".into(),
                refresh_token: "new_refresh".into(),
                token_type: "Bearer".into(),
                expires_in: 3600,
            })
        });

    let usecase = RefreshTokenUseCase::new(Arc::new(mock_provider));

    let result = usecase.execute("valid_refresh_token");

    assert!(result.is_ok());
    let token_response = result.unwrap();
    assert_eq!(token_response.access_token, "new_access");
    assert_eq!(token_response.refresh_token, "new_refresh");
}

#[tokio::test]
async fn test_refresh_token_invalid() {
    let mut mock_provider = MockTokenProvider::new();

    mock_provider
        .expect_refresh_tokens()
        .with(eq("invalid_token"))
        .times(1)
        .returning(|_| {
            Err(DomainError::Unauthorized(
                "Invalid refresh token".to_string(),
            ))
        });

    let usecase = RefreshTokenUseCase::new(Arc::new(mock_provider));

    let result = usecase.execute("invalid_token");

    match result {
        Err(DomainError::Unauthorized(_)) => {}
        _ => panic!("Expected Unauthorized error"),
    }
}
