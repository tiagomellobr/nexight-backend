use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::rngs::OsRng;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Failed to hash password")]
    HashError,

    #[error("Invalid password")]
    InvalidPassword,

    #[error("Failed to generate token")]
    TokenGenerationError,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Token expired")]
    TokenExpired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user id)
    pub email: String,
    pub exp: i64, // Expiration time
    pub iat: i64, // Issued at
}

pub struct AuthService {
    jwt_secret: String,
    token_expiration_hours: i64,
}

impl AuthService {
    pub fn new(jwt_secret: String, token_expiration_hours: i64) -> Self {
        Self {
            jwt_secret,
            token_expiration_hours,
        }
    }

    /// Hash a password using Argon2
    pub fn hash_password(&self, password: &str) -> Result<String, AuthError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|_| AuthError::HashError)
    }

    /// Verify a password against a hash
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, AuthError> {
        let parsed_hash = PasswordHash::new(hash).map_err(|_| AuthError::InvalidPassword)?;

        let argon2 = Argon2::default();

        Ok(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    /// Generate a JWT token
    pub fn generate_token(&self, user_id: Uuid, email: &str) -> Result<String, AuthError> {
        let now = Utc::now();
        let expiration = now + Duration::hours(self.token_expiration_hours);

        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            exp: expiration.timestamp(),
            iat: now.timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|_| AuthError::TokenGenerationError)
    }

    /// Verify and decode a JWT token
    pub fn verify_token(&self, token: &str) -> Result<Claims, AuthError> {
        let validation = Validation::default();

        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &validation,
        )
        .map(|data| data.claims)
        .map_err(|err| {
            if err.to_string().contains("expired") {
                AuthError::TokenExpired
            } else {
                AuthError::InvalidToken
            }
        })
    }
}
