use crate::domain::entities::article_category::ArticleCategory;
use crate::domain::repositories::article_category_repository::{
    ArticleCategoryRepository, ArticleCategoryRepositoryError,
};
use crate::infrastructure::database::schema::article_categories;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid::Uuid;

// Modelo Diesel para inserção
#[derive(Insertable)]
#[diesel(table_name = article_categories)]
struct NewArticleCategory {
    id: Uuid,
    name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

// Modelo Diesel para leitura
#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = article_categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct ArticleCategoryModel {
    id: Uuid,
    name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<ArticleCategoryModel> for ArticleCategory {
    fn from(model: ArticleCategoryModel) -> Self {
        ArticleCategory {
            id: model.id,
            name: model.name,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<ArticleCategory> for NewArticleCategory {
    fn from(category: ArticleCategory) -> Self {
        NewArticleCategory {
            id: category.id,
            name: category.name,
            created_at: category.created_at,
            updated_at: category.updated_at,
        }
    }
}

pub struct DieselArticleCategoryRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl DieselArticleCategoryRepository {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ArticleCategoryRepository for DieselArticleCategoryRepository {
    async fn create(
        &self,
        category: ArticleCategory,
    ) -> Result<ArticleCategory, ArticleCategoryRepositoryError> {
        use crate::infrastructure::database::schema::article_categories::dsl::*;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| ArticleCategoryRepositoryError::DatabaseError(e.to_string()))?;

        let new_category = NewArticleCategory::from(category);

        let result = tokio::task::spawn_blocking(move || {
            diesel::insert_into(article_categories)
                .values(&new_category)
                .returning(ArticleCategoryModel::as_returning())
                .get_result::<ArticleCategoryModel>(&mut conn)
        })
        .await
        .map_err(|e| ArticleCategoryRepositoryError::DatabaseError(e.to_string()))?
        .map_err(|e: diesel::result::Error| match e {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            ) => ArticleCategoryRepositoryError::NameAlreadyExists,
            _ => ArticleCategoryRepositoryError::DatabaseError(e.to_string()),
        })?;

        Ok(result.into())
    }

    async fn find_all(&self) -> Result<Vec<ArticleCategory>, ArticleCategoryRepositoryError> {
        use crate::infrastructure::database::schema::article_categories::dsl::*;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| ArticleCategoryRepositoryError::DatabaseError(e.to_string()))?;

        let results = tokio::task::spawn_blocking(move || {
            article_categories
                .order(name.asc())
                .select(ArticleCategoryModel::as_select())
                .load::<ArticleCategoryModel>(&mut conn)
        })
        .await
        .map_err(|e| ArticleCategoryRepositoryError::DatabaseError(e.to_string()))?
        .map_err(|e| ArticleCategoryRepositoryError::DatabaseError(e.to_string()))?;

        Ok(results.into_iter().map(|model| model.into()).collect())
    }

    async fn find_by_id(
        &self,
        category_id: Uuid,
    ) -> Result<Option<ArticleCategory>, ArticleCategoryRepositoryError> {
        use crate::infrastructure::database::schema::article_categories::dsl::*;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| ArticleCategoryRepositoryError::DatabaseError(e.to_string()))?;

        let result = tokio::task::spawn_blocking(move || {
            article_categories
                .filter(id.eq(category_id))
                .select(ArticleCategoryModel::as_select())
                .first::<ArticleCategoryModel>(&mut conn)
                .optional()
        })
        .await
        .map_err(|e| ArticleCategoryRepositoryError::DatabaseError(e.to_string()))?
        .map_err(|e| ArticleCategoryRepositoryError::DatabaseError(e.to_string()))?;

        Ok(result.map(|model| model.into()))
    }

    async fn find_by_name(
        &self,
        category_name: &str,
    ) -> Result<Option<ArticleCategory>, ArticleCategoryRepositoryError> {
        use crate::infrastructure::database::schema::article_categories::dsl::*;

        let category_name = category_name.to_string();
        let mut conn = self
            .pool
            .get()
            .map_err(|e| ArticleCategoryRepositoryError::DatabaseError(e.to_string()))?;

        let result = tokio::task::spawn_blocking(move || {
            article_categories
                .filter(name.eq(category_name))
                .select(ArticleCategoryModel::as_select())
                .first::<ArticleCategoryModel>(&mut conn)
                .optional()
        })
        .await
        .map_err(|e| ArticleCategoryRepositoryError::DatabaseError(e.to_string()))?
        .map_err(|e| ArticleCategoryRepositoryError::DatabaseError(e.to_string()))?;

        Ok(result.map(|model| model.into()))
    }
}
