use crate::domain::entities::article::PaginatedArticles;
use crate::domain::repositories::article_repository::{ArticleRepository, ArticleRepositoryError};
use std::sync::Arc;

pub struct ListArticlesUseCase {
    article_repository: Arc<dyn ArticleRepository>,
}

impl ListArticlesUseCase {
    pub fn new(article_repository: Arc<dyn ArticleRepository>) -> Self {
        Self { article_repository }
    }

    pub async fn execute(&self, page: i64, per_page: i64) -> Result<PaginatedArticles, ArticleRepositoryError> {
        self.article_repository.list(page, per_page).await
    }
}
