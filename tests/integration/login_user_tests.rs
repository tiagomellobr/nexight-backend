use nexight_backend::application::services::auth_service::AuthService;
use nexight_backend::application::use_cases::login_user::{LoginUserError, LoginUserUseCase};
use nexight_backend::domain::entities::user::{LoginDto, User};
use nexight_backend::domain::repositories::user_repository::UserRepository;
use nexight_backend::infrastructure::repositories::in_memory_user_repository::InMemoryUserRepository;
use std::sync::Arc;

fn create_test_setup() -> (LoginUserUseCase, Arc<InMemoryUserRepository>, Arc<AuthService>) {
    let repository = Arc::new(InMemoryUserRepository::new());
    let auth_service = Arc::new(AuthService::new("test_secret".to_string(), 24));
    let use_case = LoginUserUseCase::new(repository.clone(), auth_service.clone());

    (use_case, repository, auth_service)
}

async fn create_test_user(
    repository: &InMemoryUserRepository,
    auth_service: &AuthService,
    email: &str,
    password: &str,
    name: &str,
) -> User {
    let password_hash = auth_service.hash_password(password).unwrap();
    let user = User::new(email.to_string(), password_hash, name.to_string());
    repository.create(user.clone()).await.unwrap();
    user
}

#[tokio::test]
async fn test_login_user_success() {
    let (use_case, repository, auth_service) = create_test_setup();

    // Create a test user
    let email = "test@example.com";
    let password = "password123";
    create_test_user(&repository, &auth_service, email, password, "Test User").await;

    // Login
    let dto = LoginDto {
        email: email.to_string(),
        password: password.to_string(),
    };

    let result = use_case.execute(dto).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(!response.token.is_empty());
    assert_eq!(response.user.email, email);
}

#[tokio::test]
async fn test_login_user_invalid_email() {
    let (use_case, _, _) = create_test_setup();

    let dto = LoginDto {
        email: "invalid-email".to_string(),
        password: "password123".to_string(),
    };

    let result = use_case.execute(dto).await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        LoginUserError::ValidationError(_)
    ));
}

#[tokio::test]
async fn test_login_user_empty_password() {
    let (use_case, _, _) = create_test_setup();

    let dto = LoginDto {
        email: "test@example.com".to_string(),
        password: "".to_string(),
    };

    let result = use_case.execute(dto).await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        LoginUserError::ValidationError(_)
    ));
}

#[tokio::test]
async fn test_login_user_not_found() {
    let (use_case, _, _) = create_test_setup();

    let dto = LoginDto {
        email: "nonexistent@example.com".to_string(),
        password: "password123".to_string(),
    };

    let result = use_case.execute(dto).await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        LoginUserError::InvalidCredentials
    ));
}

#[tokio::test]
async fn test_login_user_wrong_password() {
    let (use_case, repository, auth_service) = create_test_setup();

    // Create a test user
    let email = "test@example.com";
    let password = "password123";
    create_test_user(&repository, &auth_service, email, password, "Test User").await;

    // Try to login with wrong password
    let dto = LoginDto {
        email: email.to_string(),
        password: "wrong_password".to_string(),
    };

    let result = use_case.execute(dto).await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        LoginUserError::InvalidCredentials
    ));
}

#[tokio::test]
async fn test_login_generates_valid_token() {
    let (use_case, repository, auth_service) = create_test_setup();

    // Create a test user
    let email = "test@example.com";
    let password = "password123";
    create_test_user(&repository, &auth_service, email, password, "Test User").await;

    // Login
    let dto = LoginDto {
        email: email.to_string(),
        password: password.to_string(),
    };

    let result = use_case.execute(dto).await;
    assert!(result.is_ok());

    let response = result.unwrap();

    // Verify token is valid
    let claims = auth_service.verify_token(&response.token);
    assert!(claims.is_ok());

    let claims = claims.unwrap();
    assert_eq!(claims.email, email);
}

#[tokio::test]
async fn test_login_multiple_times_generates_different_tokens() {
    let (use_case, repository, auth_service) = create_test_setup();

    // Create a test user
    let email = "test@example.com";
    let password = "password123";
    create_test_user(&repository, &auth_service, email, password, "Test User").await;

    // First login
    let dto1 = LoginDto {
        email: email.to_string(),
        password: password.to_string(),
    };
    let result1 = use_case.execute(dto1).await;
    assert!(result1.is_ok());

    // Wait 1 second to ensure different timestamps
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let dto2 = LoginDto {
        email: email.to_string(),
        password: password.to_string(),
    };
    let result2 = use_case.execute(dto2).await;
    assert!(result2.is_ok());

    // Tokens should be different due to different timestamps
    assert_ne!(result1.unwrap().token, result2.unwrap().token);
}

#[tokio::test]
async fn test_login_returns_correct_user_data() {
    let (use_case, repository, auth_service) = create_test_setup();

    // Create a test user
    let email = "test@example.com";
    let password = "password123";
    let name = "Test User";
    let user = create_test_user(&repository, &auth_service, email, password, name).await;

    // Login
    let dto = LoginDto {
        email: email.to_string(),
        password: password.to_string(),
    };

    let result = use_case.execute(dto).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.user.id, user.id);
    assert_eq!(response.user.email, email);
    assert_eq!(response.user.name, name);
}

#[tokio::test]
async fn test_login_case_sensitive_email() {
    let (use_case, repository, auth_service) = create_test_setup();

    // Create a test user with lowercase email
    let email = "test@example.com";
    let password = "password123";
    create_test_user(&repository, &auth_service, email, password, "Test User").await;

    // Try to login with uppercase email
    let dto = LoginDto {
        email: "TEST@EXAMPLE.COM".to_string(),
        password: password.to_string(),
    };

    let result = use_case.execute(dto).await;
    // Should fail because emails are case-sensitive in this implementation
    assert!(result.is_err());
}

// ============================================================================
// HIGH PRIORITY - Additional Tests
// ============================================================================

#[tokio::test]
async fn test_login_empty_email() {
    let (use_case, _, _) = create_test_setup();

    let dto = LoginDto {
        email: "".to_string(),
        password: "password123".to_string(),
    };

    let result = use_case.execute(dto).await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        LoginUserError::ValidationError(_)
    ));
}

#[tokio::test]
async fn test_login_email_with_whitespace() {
    let (use_case, repository, auth_service) = create_test_setup();

    // Create a test user
    let email = "test@example.com";
    let password = "password123";
    create_test_user(&repository, &auth_service, email, password, "Test User").await;

    // Try to login with email containing leading/trailing whitespace
    let dto = LoginDto {
        email: " test@example.com ".to_string(),
        password: password.to_string(),
    };

    let result = use_case.execute(dto).await;
    // Should fail because email is not trimmed in current implementation
    assert!(result.is_err());
}

#[tokio::test]
async fn test_login_password_with_whitespace() {
    let (use_case, repository, auth_service) = create_test_setup();

    // Create a test user
    let email = "test@example.com";
    let password = "password123";
    create_test_user(&repository, &auth_service, email, password, "Test User").await;

    // Try to login with password containing leading/trailing whitespace
    let dto = LoginDto {
        email: email.to_string(),
        password: " password123 ".to_string(),
    };

    let result = use_case.execute(dto).await;
    // Should fail because password whitespace is significant
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        LoginUserError::InvalidCredentials
    ));
}

#[tokio::test]
async fn test_login_with_sql_injection_attempt() {
    let (use_case, _, _) = create_test_setup();

    let dto = LoginDto {
        email: "admin'--@example.com".to_string(),
        password: "password123".to_string(),
    };

    let result = use_case.execute(dto).await;
    // Should either fail validation or not find user (safe behavior)
    assert!(result.is_err());
}

#[tokio::test]
async fn test_login_very_long_email() {
    let (use_case, _, _) = create_test_setup();

    // Create an email longer than typical database limits (255 chars)
    let long_email = format!("{}@example.com", "a".repeat(300));

    let dto = LoginDto {
        email: long_email,
        password: "password123".to_string(),
    };

    let result = use_case.execute(dto).await;
    // Should fail validation or not find user
    assert!(result.is_err());
}
