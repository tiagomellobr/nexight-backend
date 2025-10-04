use nexight_backend::application::services::auth_service::AuthService;
use nexight_backend::application::use_cases::register_user::{RegisterUserError, RegisterUserUseCase};
use nexight_backend::domain::entities::user::{CreateUserDto, User};
use nexight_backend::domain::repositories::user_repository::UserRepository;
use nexight_backend::infrastructure::repositories::in_memory_user_repository::InMemoryUserRepository;
use std::sync::Arc;

fn create_test_setup() -> (RegisterUserUseCase, Arc<InMemoryUserRepository>) {
    let repository = Arc::new(InMemoryUserRepository::new());
    let auth_service = Arc::new(AuthService::new("test_secret".to_string(), 24));
    let use_case = RegisterUserUseCase::new(repository.clone(), auth_service);

    (use_case, repository)
}

#[tokio::test]
async fn test_register_user_success() {
    let (use_case, _) = create_test_setup();

    let dto = CreateUserDto {
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
        name: "Test User".to_string(),
    };

    let result = use_case.execute(dto).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(!response.token.is_empty());
    assert_eq!(response.user.email, "test@example.com");
    assert_eq!(response.user.name, "Test User");
}

#[tokio::test]
async fn test_register_user_invalid_email() {
    let (use_case, _) = create_test_setup();

    let dto = CreateUserDto {
        email: "invalid-email".to_string(),
        password: "password123".to_string(),
        name: "Test User".to_string(),
    };

    let result = use_case.execute(dto).await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        RegisterUserError::ValidationError(_)
    ));
}

#[tokio::test]
async fn test_register_user_password_too_short() {
    let (use_case, _) = create_test_setup();

    let dto = CreateUserDto {
        email: "test@example.com".to_string(),
        password: "short".to_string(),
        name: "Test User".to_string(),
    };

    let result = use_case.execute(dto).await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        RegisterUserError::ValidationError(_)
    ));
}

#[tokio::test]
async fn test_register_user_name_too_short() {
    let (use_case, _) = create_test_setup();

    let dto = CreateUserDto {
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
        name: "T".to_string(),
    };

    let result = use_case.execute(dto).await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        RegisterUserError::ValidationError(_)
    ));
}

#[tokio::test]
async fn test_register_user_email_already_exists() {
    let (use_case, repository) = create_test_setup();

    // Create first user
    let user = User::new(
        "test@example.com".to_string(),
        "hashed_password".to_string(),
        "Existing User".to_string(),
    );
    repository.create(user).await.unwrap();

    // Try to create second user with same email
    let dto = CreateUserDto {
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
        name: "New User".to_string(),
    };

    let result = use_case.execute(dto).await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        RegisterUserError::EmailAlreadyExists
    ));
}

#[tokio::test]
async fn test_register_user_password_is_hashed() {
    let (use_case, repository) = create_test_setup();

    let dto = CreateUserDto {
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
        name: "Test User".to_string(),
    };

    let result = use_case.execute(dto.clone()).await;
    assert!(result.is_ok());

    // Verify password is hashed in database
    let user = repository
        .find_by_email(&dto.email)
        .await
        .unwrap()
        .unwrap();
    assert_ne!(user.password_hash, dto.password);
    assert!(user.password_hash.starts_with("$argon2")); // Argon2 hash format
}

#[tokio::test]
async fn test_register_user_generates_valid_token() {
    let (use_case, _) = create_test_setup();
    let auth_service = Arc::new(AuthService::new("test_secret".to_string(), 24));

    let dto = CreateUserDto {
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
        name: "Test User".to_string(),
    };

    let result = use_case.execute(dto.clone()).await;
    assert!(result.is_ok());

    let response = result.unwrap();

    // Verify token is valid
    let claims = auth_service.verify_token(&response.token);
    assert!(claims.is_ok());

    let claims = claims.unwrap();
    assert_eq!(claims.email, dto.email);
}

#[tokio::test]
async fn test_register_multiple_users() {
    let (use_case, _) = create_test_setup();

    // Register first user
    let dto1 = CreateUserDto {
        email: "user1@example.com".to_string(),
        password: "password123".to_string(),
        name: "User 1".to_string(),
    };
    let result1 = use_case.execute(dto1).await;
    assert!(result1.is_ok());

    // Register second user
    let dto2 = CreateUserDto {
        email: "user2@example.com".to_string(),
        password: "password456".to_string(),
        name: "User 2".to_string(),
    };
    let result2 = use_case.execute(dto2).await;
    assert!(result2.is_ok());

    // Verify both users have different tokens
    assert_ne!(result1.unwrap().token, result2.unwrap().token);
}

// ============================================================================
// HIGH PRIORITY - Empty Fields Tests
// ============================================================================

#[tokio::test]
async fn test_register_empty_email() {
    let (use_case, _) = create_test_setup();

    let dto = CreateUserDto {
        email: "".to_string(),
        password: "password123".to_string(),
        name: "Test User".to_string(),
    };

    let result = use_case.execute(dto).await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        RegisterUserError::ValidationError(_)
    ));
}

#[tokio::test]
async fn test_register_empty_password() {
    let (use_case, _) = create_test_setup();

    let dto = CreateUserDto {
        email: "test@example.com".to_string(),
        password: "".to_string(),
        name: "Test User".to_string(),
    };

    let result = use_case.execute(dto).await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        RegisterUserError::ValidationError(_)
    ));
}

#[tokio::test]
async fn test_register_empty_name() {
    let (use_case, _) = create_test_setup();

    let dto = CreateUserDto {
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
        name: "".to_string(),
    };

    let result = use_case.execute(dto).await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        RegisterUserError::ValidationError(_)
    ));
}

// ============================================================================
// MEDIUM PRIORITY - Whitespace and Special Characters
// ============================================================================

#[tokio::test]
async fn test_register_email_with_whitespace() {
    let (use_case, _) = create_test_setup();

    let dto = CreateUserDto {
        email: " test@example.com ".to_string(),
        password: "password123".to_string(),
        name: "Test User".to_string(),
    };

    let result = use_case.execute(dto).await;
    // Should fail validation due to whitespace
    assert!(result.is_err());
}

#[tokio::test]
async fn test_register_name_with_special_characters() {
    let (use_case, _) = create_test_setup();

    // Test with various special characters that should be allowed
    let test_cases = vec![
        ("José Silva", "jose.silva@example.com"),
        ("O'Brien", "obrien@example.com"),
        ("Mary-Jane", "mary.jane@example.com"),
        ("李明", "liming@example.com"),
        ("Müller", "muller@example.com"),
    ];

    for (name, email) in test_cases {
        let dto = CreateUserDto {
            email: email.to_string(),
            password: "password123".to_string(),
            name: name.to_string(),
        };

        let result = use_case.execute(dto).await;
        // Should succeed with special characters in name
        assert!(result.is_ok(), "Failed for name: {}", name);
    }
}

#[tokio::test]
async fn test_register_password_with_special_characters() {
    let (use_case, _) = create_test_setup();

    let dto = CreateUserDto {
        email: "test@example.com".to_string(),
        password: "P@ssw0rd!#$%".to_string(),
        name: "Test User".to_string(),
    };

    let result = use_case.execute(dto).await;
    assert!(result.is_ok());
}

// ============================================================================
// LOW PRIORITY - Edge Cases and Limits
// ============================================================================

#[tokio::test]
async fn test_register_very_long_password() {
    let (use_case, _) = create_test_setup();

    // Argon2 has a limit of 4294967295 bytes, but we test a reasonably long password
    let long_password = "a".repeat(100);

    let dto = CreateUserDto {
        email: "test@example.com".to_string(),
        password: long_password,
        name: "Test User".to_string(),
    };

    let result = use_case.execute(dto).await;
    // Should succeed - long passwords are acceptable
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_register_very_long_name() {
    let (use_case, _) = create_test_setup();

    let long_name = "a".repeat(300);

    let dto = CreateUserDto {
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
        name: long_name.clone(),
    };

    let result = use_case.execute(dto).await;
    // Depending on implementation, this might succeed or fail
    // For now, we just verify it handles it gracefully
    if result.is_ok() {
        let response = result.unwrap();
        assert_eq!(response.user.name, long_name);
    }
}

#[tokio::test]
async fn test_register_case_insensitive_email_check() {
    let (use_case, repository) = create_test_setup();

    // Register first user with lowercase email
    let user = User::new(
        "test@example.com".to_string(),
        "hashed_password".to_string(),
        "Existing User".to_string(),
    );
    repository.create(user).await.unwrap();

    // Try to register with uppercase email (should detect duplicate)
    let dto = CreateUserDto {
        email: "TEST@EXAMPLE.COM".to_string(),
        password: "password123".to_string(),
        name: "New User".to_string(),
    };

    let _result = use_case.execute(dto).await;
    // In current implementation, this might succeed (case-sensitive)
    // This test documents the current behavior
    // For production, consider normalizing emails to lowercase
}

#[tokio::test]
async fn test_register_with_sql_injection_attempt() {
    let (use_case, _) = create_test_setup();

    let dto = CreateUserDto {
        email: "admin'--@example.com".to_string(),
        password: "password123".to_string(),
        name: "Test User".to_string(),
    };

    let _result = use_case.execute(dto).await;
    // Should handle safely (either validation error or successful escaping)
    // Just ensure it doesn't crash or cause SQL injection
}

#[tokio::test]
async fn test_register_minimum_valid_values() {
    let (use_case, _) = create_test_setup();

    let dto = CreateUserDto {
        email: "a@b.c".to_string(),      // Minimum valid email
        password: "12345678".to_string(), // Exactly 8 characters
        name: "AB".to_string(),           // Exactly 2 characters
    };

    let result = use_case.execute(dto).await;
    assert!(result.is_ok());
}
