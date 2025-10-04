use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::{UserRepository, UserRepositoryError};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Implementação em memória do UserRepository para testes
#[derive(Debug, Clone)]
pub struct InMemoryUserRepository {
    users: Arc<RwLock<HashMap<Uuid, User>>>,
    email_index: Arc<RwLock<HashMap<String, Uuid>>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            email_index: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn create(&self, user: User) -> Result<User, UserRepositoryError> {
        let mut email_index = self.email_index.write().await;
        
        // Verifica se o email já existe
        if email_index.contains_key(&user.email) {
            return Err(UserRepositoryError::EmailAlreadyExists);
        }
        
        let mut users = self.users.write().await;
        
        // Adiciona o usuário
        email_index.insert(user.email.clone(), user.id);
        users.insert(user.id, user.clone());
        
        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserRepositoryError> {
        let email_index = self.email_index.read().await;
        
        if let Some(user_id) = email_index.get(email) {
            let users = self.users.read().await;
            Ok(users.get(user_id).cloned())
        } else {
            Ok(None)
        }
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, UserRepositoryError> {
        let users = self.users.read().await;
        Ok(users.get(&id).cloned())
    }

    async fn update(&self, user: User) -> Result<User, UserRepositoryError> {
        let mut users = self.users.write().await;
        
        if !users.contains_key(&user.id) {
            return Err(UserRepositoryError::NotFound);
        }
        
        users.insert(user.id, user.clone());
        Ok(user)
    }

    async fn delete(&self, id: Uuid) -> Result<(), UserRepositoryError> {
        let mut users = self.users.write().await;
        let mut email_index = self.email_index.write().await;
        
        if let Some(user) = users.remove(&id) {
            email_index.remove(&user.email);
            Ok(())
        } else {
            Err(UserRepositoryError::NotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::user::User;

    #[tokio::test]
    async fn test_create_user() {
        let repo = InMemoryUserRepository::new();
        let user = User::new(
            "test@example.com".to_string(),
            "hashed_password".to_string(),
            "Test User".to_string(),
        );

        let result = repo.create(user.clone()).await;
        assert!(result.is_ok());
        
        let created_user = result.unwrap();
        assert_eq!(created_user.email, user.email);
    }

    #[tokio::test]
    async fn test_create_user_duplicate_email() {
        let repo = InMemoryUserRepository::new();
        let user1 = User::new(
            "test@example.com".to_string(),
            "hashed_password1".to_string(),
            "Test User 1".to_string(),
        );
        let user2 = User::new(
            "test@example.com".to_string(),
            "hashed_password2".to_string(),
            "Test User 2".to_string(),
        );

        repo.create(user1).await.unwrap();
        let result = repo.create(user2).await;
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), UserRepositoryError::EmailAlreadyExists));
    }

    #[tokio::test]
    async fn test_find_by_email_existing() {
        let repo = InMemoryUserRepository::new();
        let user = User::new(
            "test@example.com".to_string(),
            "hashed_password".to_string(),
            "Test User".to_string(),
        );

        repo.create(user.clone()).await.unwrap();
        
        let result = repo.find_by_email("test@example.com").await.unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().email, user.email);
    }

    #[tokio::test]
    async fn test_find_by_email_non_existing() {
        let repo = InMemoryUserRepository::new();
        
        let result = repo.find_by_email("nonexistent@example.com").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_find_by_id_existing() {
        let repo = InMemoryUserRepository::new();
        let user = User::new(
            "test@example.com".to_string(),
            "hashed_password".to_string(),
            "Test User".to_string(),
        );

        repo.create(user.clone()).await.unwrap();
        
        let result = repo.find_by_id(user.id).await.unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().id, user.id);
    }

    #[tokio::test]
    async fn test_find_by_id_non_existing() {
        let repo = InMemoryUserRepository::new();
        
        let result = repo.find_by_id(Uuid::new_v4()).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_update_user() {
        let repo = InMemoryUserRepository::new();
        let mut user = User::new(
            "test@example.com".to_string(),
            "hashed_password".to_string(),
            "Test User".to_string(),
        );

        repo.create(user.clone()).await.unwrap();
        
        user.name = "Updated Name".to_string();
        let result = repo.update(user.clone()).await;
        
        assert!(result.is_ok());
        
        let updated_user = repo.find_by_id(user.id).await.unwrap().unwrap();
        assert_eq!(updated_user.name, "Updated Name");
    }

    #[tokio::test]
    async fn test_update_non_existing_user() {
        let repo = InMemoryUserRepository::new();
        let user = User::new(
            "test@example.com".to_string(),
            "hashed_password".to_string(),
            "Test User".to_string(),
        );

        let result = repo.update(user).await;
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), UserRepositoryError::NotFound));
    }

    #[tokio::test]
    async fn test_delete_user() {
        let repo = InMemoryUserRepository::new();
        let user = User::new(
            "test@example.com".to_string(),
            "hashed_password".to_string(),
            "Test User".to_string(),
        );

        repo.create(user.clone()).await.unwrap();
        
        let result = repo.delete(user.id).await;
        assert!(result.is_ok());
        
        let found = repo.find_by_id(user.id).await.unwrap();
        assert!(found.is_none());
        
        let found_by_email = repo.find_by_email(&user.email).await.unwrap();
        assert!(found_by_email.is_none());
    }

    #[tokio::test]
    async fn test_delete_non_existing_user() {
        let repo = InMemoryUserRepository::new();
        
        let result = repo.delete(Uuid::new_v4()).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), UserRepositoryError::NotFound));
    }
}
