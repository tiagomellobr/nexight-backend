use crate::domain::entities::article_category::ArticleCategoryResponse;
use crate::domain::repositories::article_category_repository::{DynArticleCategoryRepository, ArticleCategoryRepositoryError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ListArticleCategoriesError {
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

impl From<ArticleCategoryRepositoryError> for ListArticleCategoriesError {
    fn from(err: ArticleCategoryRepositoryError) -> Self {
        ListArticleCategoriesError::RepositoryError(err.to_string())
    }
}

#[allow(dead_code)]
pub struct ListArticleCategoriesUseCase {
    category_repository: DynArticleCategoryRepository,
}

#[allow(dead_code)]
impl ListArticleCategoriesUseCase {
    pub fn new(category_repository: DynArticleCategoryRepository) -> Self {
        Self {
            category_repository,
        }
    }

    pub async fn execute(&self) -> Result<Vec<ArticleCategoryResponse>, ListArticleCategoriesError> {
        let categories = self
            .category_repository
            .find_all()
            .await?;

        let response: Vec<ArticleCategoryResponse> = categories
            .into_iter()
            .map(|c| c.into())
            .collect();

        Ok(response)
    }
}
