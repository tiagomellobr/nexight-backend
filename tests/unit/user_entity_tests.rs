use nexight_backend::domain::entities::user::{CreateUserDto, LoginDto, User, UserResponse};
use uuid::Uuid;
use validator::Validate;

#[test]
fn test_create_user_dto_valid() {
    let dto = CreateUserDto {
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
        name: "Test User".to_string(),
    };
    
    assert!(dto.validate().is_ok());
}

#[test]
fn test_create_user_dto_invalid_email() {
    let dto = CreateUserDto {
        email: "invalid-email".to_string(),
        password: "password123".to_string(),
        name: "Test User".to_string(),
    };
    
    assert!(dto.validate().is_err());
}

#[test]
fn test_create_user_dto_password_too_short() {
    let dto = CreateUserDto {
        email: "test@example.com".to_string(),
        password: "short".to_string(),
        name: "Test User".to_string(),
    };
    
    assert!(dto.validate().is_err());
}

#[test]
fn test_create_user_dto_name_too_short() {
    let dto = CreateUserDto {
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
        name: "T".to_string(),
    };
    
    assert!(dto.validate().is_err());
}

#[test]
fn test_user_creation() {
    let user = User::new(
        "test@example.com".to_string(),
        "hashed_password".to_string(),
        "Test User".to_string(),
    );
    
    assert_eq!(user.email, "test@example.com");
    assert_eq!(user.password_hash, "hashed_password");
    assert_eq!(user.name, "Test User");
    assert!(user.id != Uuid::nil());
}

#[test]
fn test_user_to_user_response() {
    let user = User::new(
        "test@example.com".to_string(),
        "hashed_password".to_string(),
        "Test User".to_string(),
    );
    
    let response: UserResponse = user.clone().into();
    
    assert_eq!(response.id, user.id);
    assert_eq!(response.email, user.email);
    assert_eq!(response.name, user.name);
    assert_eq!(response.created_at, user.created_at);
}

#[test]
fn test_login_dto_valid() {
    let dto = LoginDto {
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };
    
    assert!(dto.validate().is_ok());
}

#[test]
fn test_login_dto_invalid_email() {
    let dto = LoginDto {
        email: "invalid-email".to_string(),
        password: "password123".to_string(),
    };
    
    assert!(dto.validate().is_err());
}
