use crate::domain::entities::article::{ArticleResponse, UpdateArticleDto};
use crate::domain::repositories::article_repository::{ArticleRepository, ArticleRepositoryError};
use std::sync::Arc;
use uuid::Uuid;

pub struct UpdateArticleUseCase {
    article_repository: Arc<dyn ArticleRepository>,
}

impl UpdateArticleUseCase {
    pub fn new(article_repository: Arc<dyn ArticleRepository>) -> Self {
        Self { article_repository }
    }

    pub async fn execute(&self, id: Uuid, dto: UpdateArticleDto) -> Result<ArticleResponse, ArticleRepositoryError> {
        // Busca o artigo existente
        let mut article = self.article_repository
            .find_by_id(id)
            .await?
            .ok_or(ArticleRepositoryError::NotFound)?;
        
        // Atualiza os campos
        article.update(dto);
        
        // Salva no banco
        let updated = self.article_repository.update(article).await?;
        Ok(ArticleResponse::from(updated))
    }
}
