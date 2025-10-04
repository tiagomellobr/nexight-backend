use crate::domain::entities::user::User;
use async_trait::async_trait;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("User not found")]
    NotFound,
    
    #[error("Email already exists")]
    EmailAlreadyExists,
    
    #[error("Database error: {0}")]
    DatabaseError(String),
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> Result<User, UserRepositoryError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserRepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, UserRepositoryError>;
    async fn update(&self, user: User) -> Result<User, UserRepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<(), UserRepositoryError>;
}

pub type DynUserRepository = Arc<dyn UserRepository>;
