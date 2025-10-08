use crate::domain::repositories::article_repository::{ArticleRepository, ArticleRepositoryError};
use std::sync::Arc;
use uuid::Uuid;

pub struct DeleteArticleUseCase {
    article_repository: Arc<dyn ArticleRepository>,
}

impl DeleteArticleUseCase {
    pub fn new(article_repository: Arc<dyn ArticleRepository>) -> Self {
        Self { article_repository }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), ArticleRepositoryError> {
        self.article_repository.delete(id).await
    }
}
