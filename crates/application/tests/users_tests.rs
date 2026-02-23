use rust_clean_application::usecases::users::{
    DeleteUserUseCase, GetUserByIdUseCase, GetUsersUseCase, UpdateUserUseCase,
};
use rust_clean_domain::{DomainError, User, UserRole};
use std::sync::Arc;
use uuid::Uuid;

mod common;
use common::MockUserRepository;

fn create_test_user(email: &str, name: &str) -> User {
    User::new(
        email.to_string(),
        "$2a$12$R9h/cIPz0gi.URNNX3kh2OPST9/PgBkqquii.V37Yo8ncRP4tYo.6".to_string(),
        name.to_string(),
    )
    .unwrap()
}

#[tokio::test]
async fn test_get_users_success() {
    let mut mock_repo = MockUserRepository::new();

    let users = vec![
        create_test_user("user1@example.com", "User One"),
        create_test_user("user2@example.com", "User Two"),
    ];

    mock_repo
        .expect_find_all()
        .times(1)
        .returning(move || Ok(users.clone()));

    let usecase = GetUsersUseCase::new(Arc::new(mock_repo));

    let result = usecase.execute().await;

    assert!(result.is_ok());
    let result_users = result.unwrap();
    assert_eq!(result_users.len(), 2);
    assert_eq!(result_users[0].email, "user1@example.com");
    assert_eq!(result_users[1].email, "user2@example.com");
}

#[tokio::test]
async fn test_get_users_empty() {
    let mut mock_repo = MockUserRepository::new();

    mock_repo
        .expect_find_all()
        .times(1)
        .returning(|| Ok(vec![]));

    let usecase = GetUsersUseCase::new(Arc::new(mock_repo));

    let result = usecase.execute().await;

    assert!(result.is_ok());
    let result_users = result.unwrap();
    assert!(result_users.is_empty());
}

#[tokio::test]
async fn test_get_user_by_id_success() {
    let mut mock_repo = MockUserRepository::new();

    let user_id = Uuid::new_v4();
    let user = User::new(
        "test@example.com".to_string(),
        "$2a$12$R9h/cIPz0gi.URNNX3kh2OPST9/PgBkqquii.V37Yo8ncRP4tYo.6".to_string(),
        "Test User".to_string(),
    )
    .unwrap();

    mock_repo
        .expect_find_by_id()
        .with(mockall::predicate::eq(user_id))
        .times(1)
        .returning(move |_| Ok(Some(user.clone())));

    let usecase = GetUserByIdUseCase::new(Arc::new(mock_repo));

    let result = usecase.execute(user_id).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().email, "test@example.com");
}

#[tokio::test]
async fn test_get_user_by_id_not_found() {
    let mut mock_repo = MockUserRepository::new();

    let user_id = Uuid::new_v4();

    mock_repo
        .expect_find_by_id()
        .with(mockall::predicate::eq(user_id))
        .times(1)
        .returning(|_| Ok(None));

    let usecase = GetUserByIdUseCase::new(Arc::new(mock_repo));

    let result = usecase.execute(user_id).await;

    match result {
        Err(DomainError::NotFound(msg)) => assert_eq!(msg, "User not found"),
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_update_user_success() {
    let mut mock_repo = MockUserRepository::new();

    let user_id = Uuid::new_v4();
    let mut user = User::new(
        "test@example.com".to_string(),
        "$2a$12$R9h/cIPz0gi.URNNX3kh2OPST9/PgBkqquii.V37Yo8ncRP4tYo.6".to_string(),
        "Old Name".to_string(),
    )
    .unwrap();
    user.id = user_id;

    let updated_user = User {
        id: user_id,
        email: user.email.clone(),
        password_hash: user.password_hash.clone(),
        name: "New Name".to_string(),
        role: UserRole::User,
        created_at: user.created_at,
        updated_at: chrono::Utc::now(),
    };

    mock_repo
        .expect_find_by_id()
        .with(mockall::predicate::eq(user_id))
        .times(1)
        .returning(move |_| Ok(Some(user.clone())));

    mock_repo
        .expect_update()
        .times(1)
        .returning(move |_| Ok(updated_user.clone()));

    let usecase = UpdateUserUseCase::new(Arc::new(mock_repo));

    let result = usecase.execute(user_id, "New Name".to_string()).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().name, "New Name");
}

#[tokio::test]
async fn test_update_user_not_found() {
    let mut mock_repo = MockUserRepository::new();

    let user_id = Uuid::new_v4();

    mock_repo
        .expect_find_by_id()
        .with(mockall::predicate::eq(user_id))
        .times(1)
        .returning(|_| Ok(None));

    let usecase = UpdateUserUseCase::new(Arc::new(mock_repo));

    let result = usecase.execute(user_id, "New Name".to_string()).await;

    match result {
        Err(DomainError::NotFound(msg)) => assert_eq!(msg, "User not found"),
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_delete_user_success() {
    let mut mock_repo = MockUserRepository::new();

    let user_id = Uuid::new_v4();

    mock_repo
        .expect_delete()
        .with(mockall::predicate::eq(user_id))
        .times(1)
        .returning(|_| Ok(()));

    let usecase = DeleteUserUseCase::new(Arc::new(mock_repo));

    let result = usecase.execute(user_id).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_user_not_found() {
    let mut mock_repo = MockUserRepository::new();

    let user_id = Uuid::new_v4();

    mock_repo
        .expect_delete()
        .with(mockall::predicate::eq(user_id))
        .times(1)
        .returning(|_| Err(DomainError::NotFound("User not found".to_string())));

    let usecase = DeleteUserUseCase::new(Arc::new(mock_repo));

    let result = usecase.execute(user_id).await;

    match result {
        Err(DomainError::NotFound(msg)) => assert_eq!(msg, "User not found"),
        _ => panic!("Expected NotFound error"),
    }
}
