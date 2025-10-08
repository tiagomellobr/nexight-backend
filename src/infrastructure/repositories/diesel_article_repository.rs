use crate::domain::entities::article::{Article, ArticleResponse, PaginatedArticles};
use crate::domain::repositories::article_repository::{ArticleRepository, ArticleRepositoryError};
use crate::infrastructure::database::models::article_model::{ArticleModel, NewArticle, UpdateArticleModel};
use crate::infrastructure::database::DbPool;
use async_trait::async_trait;
use diesel::prelude::*;
use uuid::Uuid;

pub struct DieselArticleRepository {
    pool: DbPool,
}

impl DieselArticleRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    fn model_to_entity(model: ArticleModel) -> Article {
        Article {
            id: model.id,
            title: model.title,
            description: model.description,
            link: model.link,
            pub_date: model.pub_date,
            media: model.media,
            content: model.content,
            creator: model.creator,
            feed_id: model.feed_id,
            ai_summary: model.ai_summary,
            rate: model.rate,
            keywords: model.keywords,
            processing_ai_summary: model.processing_ai_summary,
            processing_rating: model.processing_rating,
            processing_keywords: model.processing_keywords,
            category_id: model.category_id,
            processing_categorizing: model.processing_categorizing,
            ai_columnist: model.ai_columnist,
            processing_columnist: model.processing_columnist,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }

    fn entity_to_new_model(article: &Article) -> NewArticle {
        NewArticle {
            id: article.id,
            title: article.title.clone(),
            description: article.description.clone(),
            link: article.link.clone(),
            pub_date: article.pub_date,
            media: article.media.clone(),
            content: article.content.clone(),
            creator: article.creator.clone(),
            feed_id: article.feed_id,
            ai_summary: article.ai_summary.clone(),
            rate: article.rate,
            keywords: article.keywords.clone(),
            processing_ai_summary: article.processing_ai_summary,
            processing_rating: article.processing_rating,
            processing_keywords: article.processing_keywords,
            category_id: article.category_id,
            processing_categorizing: article.processing_categorizing,
            ai_columnist: article.ai_columnist.clone(),
            processing_columnist: article.processing_columnist,
            created_at: article.created_at,
            updated_at: article.updated_at,
        }
    }
}

#[async_trait]
impl ArticleRepository for DieselArticleRepository {
    async fn create(&self, article: Article) -> Result<Article, ArticleRepositoryError> {
        use crate::infrastructure::database::schema::articles::dsl::*;

        let new_article = Self::entity_to_new_model(&article);
        let mut conn = self.pool.get().map_err(|e| {
            ArticleRepositoryError::DatabaseError(format!("Failed to get connection: {}", e))
        })?;

        let result = diesel::insert_into(articles)
            .values(&new_article)
            .get_result::<ArticleModel>(&mut conn)
            .map_err(|e| {
                ArticleRepositoryError::DatabaseError(format!("Failed to create article: {}", e))
            })?;

        Ok(Self::model_to_entity(result))
    }

    async fn find_by_id(&self, article_id: Uuid) -> Result<Option<Article>, ArticleRepositoryError> {
        use crate::infrastructure::database::schema::articles::dsl::*;

        let mut conn = self.pool.get().map_err(|e| {
            ArticleRepositoryError::DatabaseError(format!("Failed to get connection: {}", e))
        })?;

        let result = articles
            .filter(id.eq(article_id))
            .first::<ArticleModel>(&mut conn)
            .optional()
            .map_err(|e| {
                ArticleRepositoryError::DatabaseError(format!("Failed to find article: {}", e))
            })?;

        Ok(result.map(Self::model_to_entity))
    }

    async fn list(&self, page: i64, per_page: i64) -> Result<PaginatedArticles, ArticleRepositoryError> {
        use crate::infrastructure::database::schema::articles::dsl::*;

        if page < 1 || per_page < 1 {
            return Err(ArticleRepositoryError::InvalidPagination);
        }

        let mut conn = self.pool.get().map_err(|e| {
            ArticleRepositoryError::DatabaseError(format!("Failed to get connection: {}", e))
        })?;

        // Conta total de artigos
        let total = articles
            .count()
            .get_result::<i64>(&mut conn)
            .map_err(|e| {
                ArticleRepositoryError::DatabaseError(format!("Failed to count articles: {}", e))
            })?;

        // Calcula paginação
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        let offset = (page - 1) * per_page;

        // Busca artigos paginados, ordenados por pub_date descendente
        let results = articles
            .order(pub_date.desc())
            .limit(per_page)
            .offset(offset)
            .load::<ArticleModel>(&mut conn)
            .map_err(|e| {
                ArticleRepositoryError::DatabaseError(format!("Failed to list articles: {}", e))
            })?;

        let article_responses: Vec<ArticleResponse> = results
            .into_iter()
            .map(|model| ArticleResponse::from(Self::model_to_entity(model)))
            .collect();

        Ok(PaginatedArticles {
            articles: article_responses,
            total,
            page,
            per_page,
            total_pages,
        })
    }

    async fn update(&self, article: Article) -> Result<Article, ArticleRepositoryError> {
        use crate::infrastructure::database::schema::articles::dsl::*;

        let mut conn = self.pool.get().map_err(|e| {
            ArticleRepositoryError::DatabaseError(format!("Failed to get connection: {}", e))
        })?;

        let update_model = UpdateArticleModel {
            title: Some(article.title.clone()),
            description: Some(article.description.clone()),
            link: Some(article.link.clone()),
            pub_date: Some(article.pub_date),
            media: Some(article.media.clone()),
            content: Some(article.content.clone()),
            creator: Some(article.creator.clone()),
            ai_summary: Some(article.ai_summary.clone()),
            rate: Some(article.rate),
            keywords: Some(article.keywords.clone()),
            processing_ai_summary: Some(article.processing_ai_summary),
            processing_rating: Some(article.processing_rating),
            processing_keywords: Some(article.processing_keywords),
            category_id: Some(article.category_id),
            processing_categorizing: Some(article.processing_categorizing),
            ai_columnist: Some(article.ai_columnist.clone()),
            processing_columnist: Some(article.processing_columnist),
            updated_at: chrono::Utc::now(),
        };

        let result = diesel::update(articles.filter(id.eq(article.id)))
            .set(&update_model)
            .get_result::<ArticleModel>(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => ArticleRepositoryError::NotFound,
                _ => ArticleRepositoryError::DatabaseError(format!("Failed to update article: {}", e)),
            })?;

        Ok(Self::model_to_entity(result))
    }

    async fn delete(&self, article_id: Uuid) -> Result<(), ArticleRepositoryError> {
        use crate::infrastructure::database::schema::articles::dsl::*;

        let mut conn = self.pool.get().map_err(|e| {
            ArticleRepositoryError::DatabaseError(format!("Failed to get connection: {}", e))
        })?;

        let rows_deleted = diesel::delete(articles.filter(id.eq(article_id)))
            .execute(&mut conn)
            .map_err(|e| {
                ArticleRepositoryError::DatabaseError(format!("Failed to delete article: {}", e))
            })?;

        if rows_deleted == 0 {
            return Err(ArticleRepositoryError::NotFound);
        }

        Ok(())
    }

    async fn count(&self) -> Result<i64, ArticleRepositoryError> {
        use crate::infrastructure::database::schema::articles::dsl::*;

        let mut conn = self.pool.get().map_err(|e| {
            ArticleRepositoryError::DatabaseError(format!("Failed to get connection: {}", e))
        })?;

        let total = articles
            .count()
            .get_result::<i64>(&mut conn)
            .map_err(|e| {
                ArticleRepositoryError::DatabaseError(format!("Failed to count articles: {}", e))
            })?;

        Ok(total)
    }
}
