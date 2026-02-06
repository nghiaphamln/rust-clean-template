use rust_clean_domain::entities::User;

#[test]
fn test_create_user_success() {
    let email = "test@example.com".to_string();
    let password_hash = "a".repeat(60); // Valid length
    let name = "Test User".to_string();

    let user = User::new(email.clone(), password_hash.clone(), name.clone());

    assert!(user.is_ok());
    let user = user.unwrap();
    assert_eq!(user.email, email);
    assert_eq!(user.password_hash, password_hash);
    assert_eq!(user.name, name);
}

#[test]
fn test_create_user_invalid_email() {
    let email = "invalid-email".to_string(); // Invalid format
    let password_hash = "a".repeat(60);
    let name = "Test User".to_string();

    let result = User::new(email, password_hash, name);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Validation error: Invalid email format"
    );
}

#[test]
fn test_create_user_invalid_hash_length() {
    let email = "test@example.com".to_string();
    let password_hash = "short_hash".to_string(); // Too short
    let name = "Test User".to_string();

    let result = User::new(email, password_hash, name);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Validation error: Invalid password hash"
    );
}

#[test]
fn test_create_user_empty_name() {
    let email = "test@example.com".to_string();
    let password_hash = "a".repeat(60);
    let name = "".to_string(); // Empty name

    let result = User::new(email, password_hash, name);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Validation error: Name cannot be empty"
    );
}
