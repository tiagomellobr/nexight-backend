use crate::domain::entities::article_category::ArticleCategory;
use crate::domain::repositories::article_category_repository::{ArticleCategoryRepository, ArticleCategoryRepositoryError};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Implementação em memória do ArticleCategoryRepository para testes
#[derive(Debug, Clone)]
pub struct InMemoryArticleCategoryRepository {
    categories: Arc<RwLock<HashMap<Uuid, ArticleCategory>>>,
    name_index: Arc<RwLock<HashMap<String, Uuid>>>,
}

impl InMemoryArticleCategoryRepository {
    pub fn new() -> Self {
        Self {
            categories: Arc::new(RwLock::new(HashMap::new())),
            name_index: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryArticleCategoryRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ArticleCategoryRepository for InMemoryArticleCategoryRepository {
    async fn create(&self, category: ArticleCategory) -> Result<ArticleCategory, ArticleCategoryRepositoryError> {
        let mut name_index = self.name_index.write().await;
        
        // Verifica se o nome já existe
        if name_index.contains_key(&category.name) {
            return Err(ArticleCategoryRepositoryError::NameAlreadyExists);
        }
        
        let mut categories = self.categories.write().await;
        
        // Adiciona a categoria
        name_index.insert(category.name.clone(), category.id);
        categories.insert(category.id, category.clone());
        
        Ok(category)
    }

    async fn find_all(&self) -> Result<Vec<ArticleCategory>, ArticleCategoryRepositoryError> {
        let categories = self.categories.read().await;
        let mut result: Vec<ArticleCategory> = categories.values().cloned().collect();
        
        // Ordena por nome para ter um resultado consistente
        result.sort_by(|a, b| a.name.cmp(&b.name));
        
        Ok(result)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<ArticleCategory>, ArticleCategoryRepositoryError> {
        let categories = self.categories.read().await;
        Ok(categories.get(&id).cloned())
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<ArticleCategory>, ArticleCategoryRepositoryError> {
        let name_index = self.name_index.read().await;
        
        if let Some(category_id) = name_index.get(name) {
            let categories = self.categories.read().await;
            Ok(categories.get(category_id).cloned())
        } else {
            Ok(None)
        }
    }
}
