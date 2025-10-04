use crate::application::services::auth_service::{AuthError, AuthService};
use crate::domain::entities::user::{AuthResponse, LoginDto};
use crate::domain::repositories::user_repository::{DynUserRepository, UserRepositoryError};
use std::sync::Arc;
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Error)]
pub enum LoginUserError {
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Failed to generate token: {0}")]
    TokenError(String),

    #[error("Repository error: {0}")]
    RepositoryError(String),
}

impl From<UserRepositoryError> for LoginUserError {
    fn from(err: UserRepositoryError) -> Self {
        LoginUserError::RepositoryError(err.to_string())
    }
}

impl From<AuthError> for LoginUserError {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::InvalidPassword => LoginUserError::InvalidCredentials,
            AuthError::TokenGenerationError => LoginUserError::TokenError(err.to_string()),
            _ => LoginUserError::TokenError(err.to_string()),
        }
    }
}

pub struct LoginUserUseCase {
    user_repository: DynUserRepository,
    auth_service: Arc<AuthService>,
}

impl LoginUserUseCase {
    pub fn new(user_repository: DynUserRepository, auth_service: Arc<AuthService>) -> Self {
        Self {
            user_repository,
            auth_service,
        }
    }

    pub async fn execute(&self, dto: LoginDto) -> Result<AuthResponse, LoginUserError> {
        // Validate input
        dto.validate()
            .map_err(|e| LoginUserError::ValidationError(e.to_string()))?;

        // Find user by email
        let user = self
            .user_repository
            .find_by_email(&dto.email)
            .await?
            .ok_or(LoginUserError::InvalidCredentials)?;

        // Verify password
        let is_valid = self
            .auth_service
            .verify_password(&dto.password, &user.password_hash)?;

        if !is_valid {
            return Err(LoginUserError::InvalidCredentials);
        }

        // Generate token
        let token = self
            .auth_service
            .generate_token(user.id, &user.email)?;

        Ok(AuthResponse {
            token,
            user: user.into(),
        })
    }
}
