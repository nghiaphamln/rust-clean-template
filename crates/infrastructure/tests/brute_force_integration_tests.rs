use chrono::{Duration, Utc};
use rust_clean_domain::FailedLoginRepository;
use rust_clean_infrastructure::PgFailedLoginRepository;
use sqlx::PgPool;
use uuid::Uuid;

async fn setup_test_db() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgresql://postgres:postgres@localhost:5432/rust_clean_db".to_string()
    });

    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database")
}

async fn cleanup_failed_logins(pool: &PgPool, email: &str) {
    sqlx::query("DELETE FROM failed_logins WHERE email = $1")
        .bind(email)
        .execute(pool)
        .await
        .ok();
}

#[tokio::test]
async fn test_create_failed_login() {
    let pool = setup_test_db().await;
    let repo = PgFailedLoginRepository::new(pool.clone());

    let test_email = format!("test_create_{}@example.com", Uuid::new_v4());
    let ip_address = "192.168.1.100";

    cleanup_failed_logins(&pool, &test_email).await;

    let failed_login =
        rust_clean_domain::FailedLogin::new(test_email.clone(), ip_address.to_string());
    let created = repo.create(&failed_login).await;

    assert!(created.is_ok());
    let created = created.unwrap();
    assert_eq!(created.email, test_email);
    assert_eq!(created.ip_address, ip_address);
    assert_eq!(created.attempts, 1);

    cleanup_failed_logins(&pool, &test_email).await;
}

#[tokio::test]
async fn test_find_by_email() {
    let pool = setup_test_db().await;
    let repo = PgFailedLoginRepository::new(pool.clone());

    let test_email = format!("test_find_{}@example.com", Uuid::new_v4());
    let ip_address = "192.168.1.101";

    cleanup_failed_logins(&pool, &test_email).await;

    let found = repo.find_by_email(&test_email).await;
    assert!(found.is_ok());
    assert!(found.unwrap().is_none());

    let failed_login =
        rust_clean_domain::FailedLogin::new(test_email.clone(), ip_address.to_string());
    repo.create(&failed_login).await.unwrap();

    let found = repo.find_by_email(&test_email).await.unwrap();
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.email, test_email);

    cleanup_failed_logins(&pool, &test_email).await;
}

#[tokio::test]
async fn test_find_by_email_and_ip() {
    let pool = setup_test_db().await;
    let repo = PgFailedLoginRepository::new(pool.clone());

    let test_email = format!("test_find_ip_{}@example.com", Uuid::new_v4());
    let ip_address = "192.168.1.102";

    cleanup_failed_logins(&pool, &test_email).await;

    let failed_login =
        rust_clean_domain::FailedLogin::new(test_email.clone(), ip_address.to_string());
    repo.create(&failed_login).await.unwrap();

    let found = repo
        .find_by_email_and_ip(&test_email, ip_address)
        .await
        .unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().ip_address, ip_address);

    let not_found = repo
        .find_by_email_and_ip(&test_email, "10.0.0.1")
        .await
        .unwrap();
    assert!(not_found.is_none());

    cleanup_failed_logins(&pool, &test_email).await;
}

#[tokio::test]
async fn test_register_failed_attempt() {
    let pool = setup_test_db().await;
    let repo = PgFailedLoginRepository::new(pool.clone());

    let test_email = format!("test_register_{}@example.com", Uuid::new_v4());
    let ip_address = "192.168.1.103";
    let max_attempts = 5;
    let lockout_minutes = 30;

    cleanup_failed_logins(&pool, &test_email).await;

    let result = repo
        .register_failed_attempt(&test_email, ip_address, max_attempts, lockout_minutes)
        .await
        .unwrap();

    assert_eq!(result.attempts, 1);
    assert!(result.locked_until.is_none());

    let result = repo
        .register_failed_attempt(&test_email, ip_address, max_attempts, lockout_minutes)
        .await
        .unwrap();

    assert_eq!(result.attempts, 2);
    assert!(result.locked_until.is_none());

    cleanup_failed_logins(&pool, &test_email).await;
}

#[tokio::test]
async fn test_lockout_after_max_attempts() {
    let pool = setup_test_db().await;
    let repo = PgFailedLoginRepository::new(pool.clone());

    let test_email = format!("test_lockout_{}@example.com", Uuid::new_v4());
    let ip_address = "192.168.1.104";
    let max_attempts = 3;
    let lockout_minutes = 15;

    cleanup_failed_logins(&pool, &test_email).await;

    for _ in 0..max_attempts - 1 {
        repo.register_failed_attempt(&test_email, ip_address, max_attempts, lockout_minutes)
            .await
            .unwrap();
    }

    let result = repo
        .register_failed_attempt(&test_email, ip_address, max_attempts, lockout_minutes)
        .await
        .unwrap();

    assert_eq!(result.attempts, max_attempts);
    assert!(result.locked_until.is_some());
    assert!(result.is_locked());

    cleanup_failed_logins(&pool, &test_email).await;
}

#[tokio::test]
async fn test_update_failed_login() {
    let pool = setup_test_db().await;
    let repo = PgFailedLoginRepository::new(pool.clone());

    let test_email = format!("test_update_{}@example.com", Uuid::new_v4());
    let ip_address = "192.168.1.105";

    cleanup_failed_logins(&pool, &test_email).await;

    let failed_login =
        rust_clean_domain::FailedLogin::new(test_email.clone(), ip_address.to_string());
    let mut created = repo.create(&failed_login).await.unwrap();

    created.increment_attempt();
    created.lock(10);

    let updated = repo.update(&created).await.unwrap();

    assert_eq!(updated.attempts, 2);
    assert!(updated.locked_until.is_some());

    cleanup_failed_logins(&pool, &test_email).await;
}

#[tokio::test]
async fn test_delete_failed_login() {
    let pool = setup_test_db().await;
    let repo = PgFailedLoginRepository::new(pool.clone());

    let test_email = format!("test_delete_{}@example.com", Uuid::new_v4());
    let ip_address = "192.168.1.106";

    cleanup_failed_logins(&pool, &test_email).await;

    let failed_login =
        rust_clean_domain::FailedLogin::new(test_email.clone(), ip_address.to_string());
    repo.create(&failed_login).await.unwrap();

    let found = repo.find_by_email(&test_email).await.unwrap();
    assert!(found.is_some());

    repo.delete(&test_email).await.unwrap();

    let found = repo.find_by_email(&test_email).await.unwrap();
    assert!(found.is_none());
}

#[tokio::test]
async fn test_delete_expired() {
    let pool = setup_test_db().await;
    let repo = PgFailedLoginRepository::new(pool.clone());

    let test_email = format!("test_expired_{}@example.com", Uuid::new_v4());
    let ip_address = "192.168.1.107";

    cleanup_failed_logins(&pool, &test_email).await;

    let mut failed_login =
        rust_clean_domain::FailedLogin::new(test_email.clone(), ip_address.to_string());
    failed_login.locked_until = Some(Utc::now() - Duration::hours(1));

    repo.create(&failed_login).await.unwrap();

    repo.delete_expired().await.unwrap();

    let found = repo.find_by_email(&test_email).await.unwrap();
    assert!(found.is_none());
}
