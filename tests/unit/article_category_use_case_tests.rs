use nexight_backend::application::use_cases::list_article_categories::ListArticleCategoriesUseCase;
use nexight_backend::domain::entities::article_category::ArticleCategory;
use nexight_backend::domain::repositories::article_category_repository::ArticleCategoryRepository;
use nexight_backend::infrastructure::repositories::in_memory_article_category_repository::InMemoryArticleCategoryRepository;
use std::sync::Arc;

#[tokio::test]
async fn test_list_categories_empty_repository() {
    let repo = Arc::new(InMemoryArticleCategoryRepository::new());
    let use_case = ListArticleCategoriesUseCase::new(repo);
    
    let result = use_case.execute().await;
    
    assert!(result.is_ok());
    let categories = result.unwrap();
    assert_eq!(categories.len(), 0);
}

#[tokio::test]
async fn test_list_categories_with_data() {
    let repo = Arc::new(InMemoryArticleCategoryRepository::new());
    
    // Cria categorias de teste
    let cat1 = ArticleCategory::new("Technology".to_string());
    let cat2 = ArticleCategory::new("Business".to_string());
    let cat3 = ArticleCategory::new("Sports".to_string());
    
    repo.create(cat1).await.unwrap();
    repo.create(cat2).await.unwrap();
    repo.create(cat3).await.unwrap();
    
    let use_case = ListArticleCategoriesUseCase::new(repo);
    let result = use_case.execute().await;
    
    assert!(result.is_ok());
    let categories = result.unwrap();
    assert_eq!(categories.len(), 3);
}

#[tokio::test]
async fn test_list_categories_sorted_alphabetically() {
    let repo = Arc::new(InMemoryArticleCategoryRepository::new());
    
    // Adiciona categorias em ordem não-alfabética
    let cat1 = ArticleCategory::new("Zebra".to_string());
    let cat2 = ArticleCategory::new("Apple".to_string());
    let cat3 = ArticleCategory::new("Mango".to_string());
    
    repo.create(cat1).await.unwrap();
    repo.create(cat2).await.unwrap();
    repo.create(cat3).await.unwrap();
    
    let use_case = ListArticleCategoriesUseCase::new(repo);
    let result = use_case.execute().await.unwrap();
    
    // Verifica que está ordenado alfabeticamente
    assert_eq!(result[0].name, "Apple");
    assert_eq!(result[1].name, "Mango");
    assert_eq!(result[2].name, "Zebra");
}

#[tokio::test]
async fn test_list_categories_returns_correct_fields() {
    let repo = Arc::new(InMemoryArticleCategoryRepository::new());
    
    let category = ArticleCategory::new("Health".to_string());
    let category_id = category.id;
    let category_name = category.name.clone();
    let category_created_at = category.created_at;
    
    repo.create(category).await.unwrap();
    
    let use_case = ListArticleCategoriesUseCase::new(repo);
    let result = use_case.execute().await.unwrap();
    
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, category_id);
    assert_eq!(result[0].name, category_name);
    assert_eq!(result[0].created_at, category_created_at);
}

#[tokio::test]
async fn test_list_categories_multiple_calls_consistent() {
    let repo = Arc::new(InMemoryArticleCategoryRepository::new());
    
    let cat1 = ArticleCategory::new("Category A".to_string());
    let cat2 = ArticleCategory::new("Category B".to_string());
    
    repo.create(cat1).await.unwrap();
    repo.create(cat2).await.unwrap();
    
    let use_case = ListArticleCategoriesUseCase::new(repo);
    
    // Executa múltiplas vezes
    let result1 = use_case.execute().await.unwrap();
    let result2 = use_case.execute().await.unwrap();
    
    assert_eq!(result1.len(), result2.len());
    assert_eq!(result1[0].id, result2[0].id);
    assert_eq!(result1[1].id, result2[1].id);
}

#[tokio::test]
async fn test_list_categories_does_not_expose_internal_fields() {
    let repo = Arc::new(InMemoryArticleCategoryRepository::new());
    
    let category = ArticleCategory::new("Privacy Test".to_string());
    repo.create(category).await.unwrap();
    
    let use_case = ListArticleCategoriesUseCase::new(repo);
    let result = use_case.execute().await.unwrap();
    
    // Verifica que ArticleCategoryResponse não expõe updated_at
    // (isso é garantido pela definição da struct, este teste documenta a intenção)
    assert_eq!(result.len(), 1);
    assert!(!result[0].name.is_empty());
}
