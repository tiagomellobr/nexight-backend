use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Article {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub link: String,
    pub pub_date: DateTime<Utc>,
    pub media: Option<String>,
    pub content: String,
    pub creator: String,
    pub feed_id: Uuid,
    pub ai_summary: Option<String>,
    pub rate: Option<i32>,
    pub keywords: Option<String>,
    pub processing_ai_summary: bool,
    pub processing_rating: bool,
    pub processing_keywords: bool,
    pub category_id: Option<Uuid>,
    pub processing_categorizing: bool,
    pub ai_columnist: Option<String>,
    pub processing_columnist: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateArticleDto {
    #[validate(length(min = 1, message = "Title is required"))]
    pub title: String,
    
    #[validate(length(min = 1, message = "Description is required"))]
    pub description: String,
    
    #[validate(length(min = 1, message = "Link is required"))]
    pub link: String,
    
    pub pub_date: DateTime<Utc>,
    pub media: Option<String>,
    
    #[validate(length(min = 1, message = "Content is required"))]
    pub content: String,
    
    #[validate(length(min = 1, message = "Creator is required"))]
    pub creator: String,
    
    pub feed_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateArticleDto {
    pub title: Option<String>,
    pub description: Option<String>,
    pub link: Option<String>,
    pub pub_date: Option<DateTime<Utc>>,
    pub media: Option<String>,
    pub content: Option<String>,
    pub creator: Option<String>,
    pub ai_summary: Option<String>,
    pub rate: Option<i32>,
    pub keywords: Option<String>,
    pub category_id: Option<Uuid>,
    pub ai_columnist: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub link: String,
    pub pub_date: DateTime<Utc>,
    pub media: Option<String>,
    pub content: String,
    pub creator: String,
    pub feed_id: Uuid,
    pub ai_summary: Option<String>,
    pub rate: Option<i32>,
    pub keywords: Option<String>,
    pub category_id: Option<Uuid>,
    pub ai_columnist: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedArticles {
    pub articles: Vec<ArticleResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

impl From<Article> for ArticleResponse {
    fn from(article: Article) -> Self {
        ArticleResponse {
            id: article.id,
            title: article.title,
            description: article.description,
            link: article.link,
            pub_date: article.pub_date,
            media: article.media,
            content: article.content,
            creator: article.creator,
            feed_id: article.feed_id,
            ai_summary: article.ai_summary,
            rate: article.rate,
            keywords: article.keywords,
            category_id: article.category_id,
            ai_columnist: article.ai_columnist,
            created_at: article.created_at,
        }
    }
}

impl Article {
    #[allow(dead_code)]
    pub fn new(dto: CreateArticleDto) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title: dto.title,
            description: dto.description,
            link: dto.link,
            pub_date: dto.pub_date,
            media: dto.media,
            content: dto.content,
            creator: dto.creator,
            feed_id: dto.feed_id,
            ai_summary: None,
            rate: None,
            keywords: None,
            processing_ai_summary: false,
            processing_rating: false,
            processing_keywords: false,
            category_id: None,
            processing_categorizing: false,
            ai_columnist: None,
            processing_columnist: false,
            created_at: now,
            updated_at: now,
        }
    }

    #[allow(dead_code)]
    pub fn update(&mut self, dto: UpdateArticleDto) {
        if let Some(title) = dto.title {
            self.title = title;
        }
        if let Some(description) = dto.description {
            self.description = description;
        }
        if let Some(link) = dto.link {
            self.link = link;
        }
        if let Some(pub_date) = dto.pub_date {
            self.pub_date = pub_date;
        }
        if dto.media.is_some() {
            self.media = dto.media;
        }
        if let Some(content) = dto.content {
            self.content = content;
        }
        if let Some(creator) = dto.creator {
            self.creator = creator;
        }
        if dto.ai_summary.is_some() {
            self.ai_summary = dto.ai_summary;
        }
        if dto.rate.is_some() {
            self.rate = dto.rate;
        }
        if dto.keywords.is_some() {
            self.keywords = dto.keywords;
        }
        if dto.category_id.is_some() {
            self.category_id = dto.category_id;
        }
        if dto.ai_columnist.is_some() {
            self.ai_columnist = dto.ai_columnist;
        }
        self.updated_at = Utc::now();
    }
}
