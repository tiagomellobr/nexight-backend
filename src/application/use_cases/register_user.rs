use crate::application::services::auth_service::{AuthError, AuthService};
use crate::domain::entities::user::{AuthResponse, CreateUserDto, User};
use crate::domain::repositories::user_repository::{DynUserRepository, UserRepositoryError};
use std::sync::Arc;
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Error)]
pub enum RegisterUserError {
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Email already in use")]
    EmailAlreadyExists,

    #[error("Failed to hash password: {0}")]
    PasswordHashError(String),

    #[error("Failed to create user: {0}")]
    RepositoryError(String),

    #[error("Failed to generate token: {0}")]
    TokenError(String),
}

impl From<UserRepositoryError> for RegisterUserError {
    fn from(err: UserRepositoryError) -> Self {
        match err {
            UserRepositoryError::EmailAlreadyExists => RegisterUserError::EmailAlreadyExists,
            _ => RegisterUserError::RepositoryError(err.to_string()),
        }
    }
}

impl From<AuthError> for RegisterUserError {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::HashError => RegisterUserError::PasswordHashError(err.to_string()),
            AuthError::TokenGenerationError => RegisterUserError::TokenError(err.to_string()),
            _ => RegisterUserError::TokenError(err.to_string()),
        }
    }
}

pub struct RegisterUserUseCase {
    user_repository: DynUserRepository,
    auth_service: Arc<AuthService>,
}

impl RegisterUserUseCase {
    pub fn new(user_repository: DynUserRepository, auth_service: Arc<AuthService>) -> Self {
        Self {
            user_repository,
            auth_service,
        }
    }

    pub async fn execute(&self, dto: CreateUserDto) -> Result<AuthResponse, RegisterUserError> {
        // Validate input
        dto.validate()
            .map_err(|e| RegisterUserError::ValidationError(e.to_string()))?;

        // Check if email already exists
        if let Some(_) = self.user_repository.find_by_email(&dto.email).await? {
            return Err(RegisterUserError::EmailAlreadyExists);
        }

        // Hash password
        let password_hash = self.auth_service.hash_password(&dto.password)?;

        // Create user
        let user = User::new(dto.email.clone(), password_hash, dto.name.clone());

        // Save user
        let created_user = self.user_repository.create(user).await?;

        // Generate token
        let token = self
            .auth_service
            .generate_token(created_user.id, &created_user.email)?;

        Ok(AuthResponse {
            token,
            user: created_user.into(),
        })
    }
}
