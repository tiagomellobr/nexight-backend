use crate::domain::entities::article::{Article, PaginatedArticles};
use async_trait::async_trait;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ArticleRepositoryError {
    #[error("Article not found")]
    NotFound,
    
    #[allow(dead_code)]
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Invalid pagination parameters")]
    InvalidPagination,
}

#[async_trait]
pub trait ArticleRepository: Send + Sync {
    /// Cria um novo artigo
    #[allow(dead_code)]
    async fn create(&self, article: Article) -> Result<Article, ArticleRepositoryError>;
    
    /// Busca um artigo por ID
    #[allow(dead_code)]
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Article>, ArticleRepositoryError>;
    
    /// Lista artigos com paginação
    #[allow(dead_code)]
    async fn list(&self, page: i64, per_page: i64) -> Result<PaginatedArticles, ArticleRepositoryError>;
    
    /// Atualiza um artigo
    #[allow(dead_code)]
    async fn update(&self, article: Article) -> Result<Article, ArticleRepositoryError>;
    
    /// Remove um artigo
    #[allow(dead_code)]
    async fn delete(&self, id: Uuid) -> Result<(), ArticleRepositoryError>;
    
    /// Conta o total de artigos
    #[allow(dead_code)]
    async fn count(&self) -> Result<i64, ArticleRepositoryError>;
}

#[allow(dead_code)]
pub type DynArticleRepository = Arc<dyn ArticleRepository>;
