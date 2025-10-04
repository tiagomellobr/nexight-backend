use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArticleCategory {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateArticleCategoryDto {
    #[validate(length(min = 2, message = "Name must be at least 2 characters"))]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleCategoryResponse {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl From<ArticleCategory> for ArticleCategoryResponse {
    fn from(category: ArticleCategory) -> Self {
        ArticleCategoryResponse {
            id: category.id,
            name: category.name,
            created_at: category.created_at,
        }
    }
}

#[allow(dead_code)]
impl ArticleCategory {
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            created_at: now,
            updated_at: now,
        }
    }
}
