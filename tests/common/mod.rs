// Common test utilities and helpers
#![allow(dead_code)]

use nexight_backend::application::services::auth_service::AuthService;
use nexight_backend::infrastructure::repositories::in_memory_user_repository::InMemoryUserRepository;
use std::sync::Arc;

/// Create a test AuthService with default settings
pub fn create_test_auth_service() -> Arc<AuthService> {
    Arc::new(AuthService::new("test_secret_key_for_testing".to_string(), 24))
}

/// Create a test InMemoryUserRepository
pub fn create_test_user_repository() -> Arc<InMemoryUserRepository> {
    Arc::new(InMemoryUserRepository::new())
}
