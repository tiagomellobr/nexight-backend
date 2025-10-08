use crate::infrastructure::database::schema::articles;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Modelo Diesel para leitura de artigos do banco de dados
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = articles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ArticleModel {
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

/// Modelo Diesel para inserção de novos artigos
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = articles)]
pub struct NewArticle {
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

/// Modelo Diesel para atualização de artigos
#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = articles)]
pub struct UpdateArticleModel {
    pub title: Option<String>,
    pub description: Option<String>,
    pub link: Option<String>,
    pub pub_date: Option<DateTime<Utc>>,
    pub media: Option<Option<String>>,
    pub content: Option<String>,
    pub creator: Option<String>,
    pub ai_summary: Option<Option<String>>,
    pub rate: Option<Option<i32>>,
    pub keywords: Option<Option<String>>,
    pub processing_ai_summary: Option<bool>,
    pub processing_rating: Option<bool>,
    pub processing_keywords: Option<bool>,
    pub category_id: Option<Option<Uuid>>,
    pub processing_categorizing: Option<bool>,
    pub ai_columnist: Option<Option<String>>,
    pub processing_columnist: Option<bool>,
    pub updated_at: DateTime<Utc>,
}
