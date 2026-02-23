use chrono::{Duration, Utc};
use rust_clean_domain::entities::FailedLogin;

#[test]
fn test_create_failed_login() {
    let email = "test@example.com".to_string();
    let ip_address = "192.168.1.1".to_string();

    let failed_login = FailedLogin::new(email.clone(), ip_address.clone());

    assert_eq!(failed_login.email, email);
    assert_eq!(failed_login.ip_address, ip_address);
    assert_eq!(failed_login.attempts, 1);
    assert!(failed_login.locked_until.is_none());
    assert!(!failed_login.is_locked());
}

#[test]
fn test_increment_attempt() {
    let mut failed_login =
        FailedLogin::new("test@example.com".to_string(), "127.0.0.1".to_string());

    assert_eq!(failed_login.attempts, 1);

    failed_login.increment_attempt();
    assert_eq!(failed_login.attempts, 2);

    failed_login.increment_attempt();
    assert_eq!(failed_login.attempts, 3);
}

#[test]
fn test_lock_account() {
    let mut failed_login =
        FailedLogin::new("test@example.com".to_string(), "127.0.0.1".to_string());

    assert!(failed_login.locked_until.is_none());
    assert!(!failed_login.is_locked());

    failed_login.lock(30);

    assert!(failed_login.locked_until.is_some());
    assert!(failed_login.is_locked());
}

#[test]
fn test_is_locked_when_expired() {
    let mut failed_login =
        FailedLogin::new("test@example.com".to_string(), "127.0.0.1".to_string());

    failed_login.locked_until = Some(Utc::now() - Duration::minutes(5));

    assert!(!failed_login.is_locked());
}

#[test]
fn test_is_locked_when_active() {
    let mut failed_login =
        FailedLogin::new("test@example.com".to_string(), "127.0.0.1".to_string());

    failed_login.locked_until = Some(Utc::now() + Duration::minutes(30));

    assert!(failed_login.is_locked());
}

#[test]
fn test_reset() {
    let mut failed_login =
        FailedLogin::new("test@example.com".to_string(), "127.0.0.1".to_string());
    failed_login.increment_attempt();
    failed_login.increment_attempt();
    failed_login.lock(30);

    assert_eq!(failed_login.attempts, 3);
    assert!(failed_login.is_locked());

    failed_login.reset();

    assert_eq!(failed_login.attempts, 0);
    assert!(failed_login.locked_until.is_none());
    assert!(!failed_login.is_locked());
}

#[test]
fn test_lockout_flow() {
    let mut failed_login =
        FailedLogin::new("test@example.com".to_string(), "127.0.0.1".to_string());
    let max_attempts = 5;

    for _ in 1..max_attempts {
        failed_login.increment_attempt();
    }

    assert_eq!(failed_login.attempts, max_attempts);
    assert!(!failed_login.is_locked());

    failed_login.lock(15);

    assert!(failed_login.is_locked());
}
