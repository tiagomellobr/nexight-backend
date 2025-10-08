use crate::domain::entities::article::{Article, ArticleResponse, CreateArticleDto};
use crate::domain::repositories::article_repository::{ArticleRepository, ArticleRepositoryError};
use std::sync::Arc;

pub struct CreateArticleUseCase {
    article_repository: Arc<dyn ArticleRepository>,
}

impl CreateArticleUseCase {
    pub fn new(article_repository: Arc<dyn ArticleRepository>) -> Self {
        Self { article_repository }
    }

    pub async fn execute(&self, dto: CreateArticleDto) -> Result<ArticleResponse, ArticleRepositoryError> {
        let article = Article::new(dto);
        let created = self.article_repository.create(article).await?;
        Ok(ArticleResponse::from(created))
    }
}
