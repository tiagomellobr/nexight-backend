use crate::domain::entities::article::{Article, ArticleResponse, PaginatedArticles};
use crate::domain::repositories::article_repository::{ArticleRepository, ArticleRepositoryError};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct InMemoryArticleRepository {
    articles: Arc<RwLock<HashMap<Uuid, Article>>>,
}

impl InMemoryArticleRepository {
    pub fn new() -> Self {
        Self {
            articles: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryArticleRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ArticleRepository for InMemoryArticleRepository {
    async fn create(&self, article: Article) -> Result<Article, ArticleRepositoryError> {
        let mut articles = self.articles.write().await;
        articles.insert(article.id, article.clone());
        Ok(article)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Article>, ArticleRepositoryError> {
        let articles = self.articles.read().await;
        Ok(articles.get(&id).cloned())
    }

    async fn list(&self, page: i64, per_page: i64) -> Result<PaginatedArticles, ArticleRepositoryError> {
        if page < 1 || per_page < 1 {
            return Err(ArticleRepositoryError::InvalidPagination);
        }

        let articles = self.articles.read().await;
        let total = articles.len() as i64;
        
        // Converte para Vec e ordena por data de publicação (mais recente primeiro)
        let mut articles_vec: Vec<Article> = articles.values().cloned().collect();
        articles_vec.sort_by(|a, b| b.pub_date.cmp(&a.pub_date));
        
        // Calcula paginação
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        let skip = ((page - 1) * per_page) as usize;
        let take = per_page as usize;
        
        // Aplica paginação
        let paginated: Vec<ArticleResponse> = articles_vec
            .into_iter()
            .skip(skip)
            .take(take)
            .map(ArticleResponse::from)
            .collect();
        
        Ok(PaginatedArticles {
            articles: paginated,
            total,
            page,
            per_page,
            total_pages,
        })
    }

    async fn update(&self, article: Article) -> Result<Article, ArticleRepositoryError> {
        let mut articles = self.articles.write().await;
        
        if !articles.contains_key(&article.id) {
            return Err(ArticleRepositoryError::NotFound);
        }
        
        articles.insert(article.id, article.clone());
        Ok(article)
    }

    async fn delete(&self, id: Uuid) -> Result<(), ArticleRepositoryError> {
        let mut articles = self.articles.write().await;
        
        if articles.remove(&id).is_none() {
            return Err(ArticleRepositoryError::NotFound);
        }
        
        Ok(())
    }

    async fn count(&self) -> Result<i64, ArticleRepositoryError> {
        let articles = self.articles.read().await;
        Ok(articles.len() as i64)
    }
}
