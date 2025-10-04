use nexight_backend::application::services::auth_service::AuthService;
use uuid::Uuid;
use chrono::Utc;

fn create_test_auth_service() -> AuthService {
    AuthService::new("test_secret_key_for_testing".to_string(), 24)
}

#[test]
fn test_hash_password() {
    let auth_service = create_test_auth_service();
    let password = "my_secure_password";

    let hash = auth_service.hash_password(password);
    assert!(hash.is_ok());
    assert!(!hash.unwrap().is_empty());
}

#[test]
fn test_hash_password_generates_different_hashes() {
    let auth_service = create_test_auth_service();
    let password = "my_secure_password";

    let hash1 = auth_service.hash_password(password).unwrap();
    let hash2 = auth_service.hash_password(password).unwrap();

    // Same password should generate different hashes due to different salts
    assert_ne!(hash1, hash2);
}

#[test]
fn test_verify_password_correct() {
    let auth_service = create_test_auth_service();
    let password = "my_secure_password";

    let hash = auth_service.hash_password(password).unwrap();
    let is_valid = auth_service.verify_password(password, &hash).unwrap();

    assert!(is_valid);
}

#[test]
fn test_verify_password_incorrect() {
    let auth_service = create_test_auth_service();
    let password = "my_secure_password";
    let wrong_password = "wrong_password";

    let hash = auth_service.hash_password(password).unwrap();
    let is_valid = auth_service.verify_password(wrong_password, &hash).unwrap();

    assert!(!is_valid);
}

#[test]
fn test_verify_password_invalid_hash() {
    let auth_service = create_test_auth_service();
    let password = "my_secure_password";
    let invalid_hash = "invalid_hash_string";

    let result = auth_service.verify_password(password, invalid_hash);
    assert!(result.is_err());
}

#[test]
fn test_generate_token() {
    let auth_service = create_test_auth_service();
    let user_id = Uuid::new_v4();
    let email = "test@example.com";

    let token = auth_service.generate_token(user_id, email);
    assert!(token.is_ok());
    assert!(!token.unwrap().is_empty());
}

#[test]
fn test_verify_valid_token() {
    let auth_service = create_test_auth_service();
    let user_id = Uuid::new_v4();
    let email = "test@example.com";

    let token = auth_service.generate_token(user_id, email).unwrap();
    let claims = auth_service.verify_token(&token);

    assert!(claims.is_ok());
    let claims = claims.unwrap();
    assert_eq!(claims.sub, user_id.to_string());
    assert_eq!(claims.email, email);
}

#[test]
fn test_verify_invalid_token() {
    let auth_service = create_test_auth_service();
    let invalid_token = "invalid.token.here";

    let result = auth_service.verify_token(invalid_token);
    assert!(result.is_err());
}

#[test]
fn test_verify_token_with_wrong_secret() {
    let auth_service1 = AuthService::new("secret1".to_string(), 24);
    let auth_service2 = AuthService::new("secret2".to_string(), 24);

    let user_id = Uuid::new_v4();
    let email = "test@example.com";

    let token = auth_service1.generate_token(user_id, email).unwrap();
    let result = auth_service2.verify_token(&token);

    assert!(result.is_err());
}

#[test]
fn test_token_expiration_time() {
    let auth_service = create_test_auth_service();
    let user_id = Uuid::new_v4();
    let email = "test@example.com";

    let token = auth_service.generate_token(user_id, email).unwrap();
    let claims = auth_service.verify_token(&token).unwrap();

    let now = Utc::now().timestamp();
    let expected_expiration = now + (24 * 60 * 60); // 24 hours

    // Allow 5 seconds tolerance for test execution time
    assert!((claims.exp - expected_expiration).abs() < 5);
}

// ============================================================================
// HIGH PRIORITY - Token Expiration Test
// ============================================================================

#[test]
fn test_verify_expired_token() {
    use jsonwebtoken::{encode, EncodingKey, Header};
    use nexight_backend::application::services::auth_service::Claims;

    let auth_service = create_test_auth_service();
    let user_id = Uuid::new_v4();
    let email = "test@example.com";

    // Create a token that expired 1 hour ago
    let now = Utc::now();
    let expired_time = now - chrono::Duration::hours(1);

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        exp: expired_time.timestamp(),
        iat: (now - chrono::Duration::hours(25)).timestamp(),
    };

    // Generate expired token with same secret
    let expired_token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("test_secret_key_for_testing".as_bytes()),
    )
    .unwrap();

    // Try to verify expired token
    let result = auth_service.verify_token(&expired_token);
    assert!(result.is_err());
    
    // Verify it's an error (could be TokenExpired or InvalidToken depending on JWT library)
    // The important thing is that expired tokens are rejected
    if let Err(error) = result {
        // Accept either TokenExpired or InvalidToken as both indicate rejection
        let error_str = format!("{:?}", error);
        assert!(
            error_str.contains("Expired") || error_str.contains("Invalid"),
            "Expected token rejection error, got: {:?}", error
        );
    }
}

// ============================================================================
// MEDIUM PRIORITY - Additional Security Tests
// ============================================================================

#[test]
fn test_hash_empty_password() {
    let auth_service = create_test_auth_service();
    let password = "";

    let hash = auth_service.hash_password(password);
    // Should still work (empty passwords are validated at DTO level)
    assert!(hash.is_ok());
}

#[test]
fn test_hash_very_long_password() {
    let auth_service = create_test_auth_service();
    let password = "a".repeat(1000);

    let hash = auth_service.hash_password(&password);
    assert!(hash.is_ok());
    
    // Verify it can be verified
    let hash_str = hash.unwrap();
    let is_valid = auth_service.verify_password(&password, &hash_str).unwrap();
    assert!(is_valid);
}

#[test]
fn test_verify_password_with_special_characters() {
    let auth_service = create_test_auth_service();
    let password = "P@ssw0rd!#$%^&*()_+-=[]{}|;':\",./<>?";

    let hash = auth_service.hash_password(password).unwrap();
    let is_valid = auth_service.verify_password(password, &hash).unwrap();

    assert!(is_valid);
}

#[test]
fn test_token_with_unicode_email() {
    let auth_service = create_test_auth_service();
    let user_id = Uuid::new_v4();
    let email = "用户@example.com"; // Unicode email

    let token = auth_service.generate_token(user_id, email);
    assert!(token.is_ok());

    let token_str = token.unwrap();
    let claims = auth_service.verify_token(&token_str);
    assert!(claims.is_ok());

    let claims = claims.unwrap();
    assert_eq!(claims.email, email);
}

#[test]
fn test_generate_token_with_nil_uuid() {
    let auth_service = create_test_auth_service();
    let user_id = Uuid::nil();
    let email = "test@example.com";

    let token = auth_service.generate_token(user_id, email);
    assert!(token.is_ok());

    let claims = auth_service.verify_token(&token.unwrap()).unwrap();
    assert_eq!(claims.sub, user_id.to_string());
}
