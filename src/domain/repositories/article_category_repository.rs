use crate::domain::entities::article_category::ArticleCategory;
use async_trait::async_trait;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ArticleCategoryRepositoryError {
    #[error("Category not found")]
    #[allow(dead_code)]
    NotFound,
    
    #[error("Category name already exists")]
    NameAlreadyExists,
    
    #[error("Database error: {0}")]
    #[allow(dead_code)]
    DatabaseError(String),
}

#[async_trait]
pub trait ArticleCategoryRepository: Send + Sync {
    #[allow(dead_code)]
    async fn create(&self, category: ArticleCategory) -> Result<ArticleCategory, ArticleCategoryRepositoryError>;
    #[allow(dead_code)]
    async fn find_all(&self) -> Result<Vec<ArticleCategory>, ArticleCategoryRepositoryError>;
    #[allow(dead_code)]
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ArticleCategory>, ArticleCategoryRepositoryError>;
    #[allow(dead_code)]
    async fn find_by_name(&self, name: &str) -> Result<Option<ArticleCategory>, ArticleCategoryRepositoryError>;
}

#[allow(dead_code)]
pub type DynArticleCategoryRepository = Arc<dyn ArticleCategoryRepository>;
