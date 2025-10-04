use nexight_backend::domain::entities::article_category::{ArticleCategory, CreateArticleCategoryDto, ArticleCategoryResponse};
use uuid::Uuid;
use validator::Validate;

#[test]
fn test_create_article_category_dto_valid() {
    let dto = CreateArticleCategoryDto {
        name: "Technology".to_string(),
    };
    
    assert!(dto.validate().is_ok());
}

#[test]
fn test_create_article_category_dto_name_too_short() {
    let dto = CreateArticleCategoryDto {
        name: "T".to_string(),
    };
    
    assert!(dto.validate().is_err());
}

#[test]
fn test_create_article_category_dto_empty_name() {
    let dto = CreateArticleCategoryDto {
        name: "".to_string(),
    };
    
    assert!(dto.validate().is_err());
}

#[test]
fn test_article_category_creation() {
    let category = ArticleCategory::new("Technology".to_string());
    
    assert_eq!(category.name, "Technology");
    assert_ne!(category.id, Uuid::nil());
    assert!(category.created_at.timestamp() > 0);
    assert!(category.updated_at.timestamp() > 0);
    assert_eq!(category.created_at, category.updated_at);
}

#[test]
fn test_article_category_unique_ids() {
    let cat1 = ArticleCategory::new("Technology".to_string());
    let cat2 = ArticleCategory::new("Business".to_string());
    
    assert_ne!(cat1.id, cat2.id);
}

#[test]
fn test_article_category_to_response() {
    let category = ArticleCategory::new("Sports".to_string());
    
    let response: ArticleCategoryResponse = category.clone().into();
    
    assert_eq!(response.id, category.id);
    assert_eq!(response.name, category.name);
    assert_eq!(response.created_at, category.created_at);
}

#[test]
fn test_article_category_clone() {
    let category = ArticleCategory::new("Health".to_string());
    let cloned = category.clone();
    
    assert_eq!(category.id, cloned.id);
    assert_eq!(category.name, cloned.name);
    assert_eq!(category.created_at, cloned.created_at);
    assert_eq!(category.updated_at, cloned.updated_at);
}

#[test]
fn test_article_category_partial_eq() {
    let cat1 = ArticleCategory::new("Entertainment".to_string());
    let cat2 = cat1.clone();
    
    assert_eq!(cat1, cat2);
}
