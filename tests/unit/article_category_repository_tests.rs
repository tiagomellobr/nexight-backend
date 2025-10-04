use nexight_backend::domain::entities::article_category::ArticleCategory;
use nexight_backend::domain::repositories::article_category_repository::{ArticleCategoryRepository, ArticleCategoryRepositoryError};
use nexight_backend::infrastructure::repositories::in_memory_article_category_repository::InMemoryArticleCategoryRepository;
use uuid::Uuid;

#[tokio::test]
async fn test_create_category() {
    let repo = InMemoryArticleCategoryRepository::new();
    let category = ArticleCategory::new("Technology".to_string());
    
    let result = repo.create(category.clone()).await;
    assert!(result.is_ok());
    
    let created = result.unwrap();
    assert_eq!(created.name, "Technology");
}

#[tokio::test]
async fn test_create_duplicate_name_fails() {
    let repo = InMemoryArticleCategoryRepository::new();
    let category1 = ArticleCategory::new("Technology".to_string());
    let category2 = ArticleCategory::new("Technology".to_string());
    
    repo.create(category1).await.unwrap();
    let result = repo.create(category2).await;
    
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ArticleCategoryRepositoryError::NameAlreadyExists));
}

#[tokio::test]
async fn test_find_all_empty() {
    let repo = InMemoryArticleCategoryRepository::new();
    let result = repo.find_all().await;
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[tokio::test]
async fn test_find_all_returns_sorted() {
    let repo = InMemoryArticleCategoryRepository::new();
    
    let cat1 = ArticleCategory::new("Zebra".to_string());
    let cat2 = ArticleCategory::new("Apple".to_string());
    let cat3 = ArticleCategory::new("Mango".to_string());
    
    repo.create(cat1).await.unwrap();
    repo.create(cat2).await.unwrap();
    repo.create(cat3).await.unwrap();
    
    let result = repo.find_all().await.unwrap();
    assert_eq!(result.len(), 3);
    assert_eq!(result[0].name, "Apple");
    assert_eq!(result[1].name, "Mango");
    assert_eq!(result[2].name, "Zebra");
}

#[tokio::test]
async fn test_find_by_id_existing() {
    let repo = InMemoryArticleCategoryRepository::new();
    let category = ArticleCategory::new("Sports".to_string());
    let id = category.id;
    
    repo.create(category).await.unwrap();
    let result = repo.find_by_id(id).await;
    
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

#[tokio::test]
async fn test_find_by_id_non_existing() {
    let repo = InMemoryArticleCategoryRepository::new();
    let result = repo.find_by_id(Uuid::new_v4()).await;
    
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[tokio::test]
async fn test_find_by_name_existing() {
    let repo = InMemoryArticleCategoryRepository::new();
    let category = ArticleCategory::new("Business".to_string());
    
    repo.create(category).await.unwrap();
    let result = repo.find_by_name("Business").await;
    
    assert!(result.is_ok());
    let found = result.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "Business");
}

#[tokio::test]
async fn test_find_by_name_non_existing() {
    let repo = InMemoryArticleCategoryRepository::new();
    let result = repo.find_by_name("NonExistent").await;
    
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}
