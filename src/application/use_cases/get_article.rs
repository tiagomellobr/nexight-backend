use crate::domain::entities::article::ArticleResponse;
use crate::domain::repositories::article_repository::{ArticleRepository, ArticleRepositoryError};
use std::sync::Arc;
use uuid::Uuid;

pub struct GetArticleUseCase {
    article_repository: Arc<dyn ArticleRepository>,
}

impl GetArticleUseCase {
    pub fn new(article_repository: Arc<dyn ArticleRepository>) -> Self {
        Self { article_repository }
    }

    pub async fn execute(&self, id: Uuid) -> Result<Option<ArticleResponse>, ArticleRepositoryError> {
        let article = self.article_repository.find_by_id(id).await?;
        Ok(article.map(ArticleResponse::from))
    }
}
