use nexight_backend::application::use_cases::list_article_categories::ListArticleCategoriesUseCase;
use nexight_backend::domain::entities::article_category::ArticleCategory;
use nexight_backend::domain::repositories::article_category_repository::ArticleCategoryRepository;
use nexight_backend::infrastructure::repositories::in_memory_article_category_repository::InMemoryArticleCategoryRepository;
use std::sync::Arc;

fn create_test_setup() -> (ListArticleCategoriesUseCase, Arc<InMemoryArticleCategoryRepository>) {
    let repository = Arc::new(InMemoryArticleCategoryRepository::new());
    let use_case = ListArticleCategoriesUseCase::new(repository.clone());

    (use_case, repository)
}

async fn create_test_category(repository: &InMemoryArticleCategoryRepository, name: &str) -> ArticleCategory {
    let category = ArticleCategory::new(name.to_string());
    repository.create(category.clone()).await.unwrap();
    category
}

#[tokio::test]
async fn test_list_categories_empty() {
    let (use_case, _) = create_test_setup();

    let result = use_case.execute().await;
    assert!(result.is_ok());

    let categories = result.unwrap();
    assert_eq!(categories.len(), 0);
}

#[tokio::test]
async fn test_list_categories_single_category() {
    let (use_case, repository) = create_test_setup();

    let created_category = create_test_category(&repository, "Technology").await;

    let result = use_case.execute().await;
    assert!(result.is_ok());

    let categories = result.unwrap();
    assert_eq!(categories.len(), 1);
    assert_eq!(categories[0].id, created_category.id);
    assert_eq!(categories[0].name, "Technology");
}

#[tokio::test]
async fn test_list_categories_multiple_categories() {
    let (use_case, repository) = create_test_setup();

    create_test_category(&repository, "Technology").await;
    create_test_category(&repository, "Business").await;
    create_test_category(&repository, "Sports").await;

    let result = use_case.execute().await;
    assert!(result.is_ok());

    let categories = result.unwrap();
    assert_eq!(categories.len(), 3);
}

#[tokio::test]
async fn test_list_categories_returns_sorted_alphabetically() {
    let (use_case, repository) = create_test_setup();

    // Adiciona categorias em ordem não-alfabética
    create_test_category(&repository, "Zebra").await;
    create_test_category(&repository, "Apple").await;
    create_test_category(&repository, "Mango").await;
    create_test_category(&repository, "Banana").await;

    let result = use_case.execute().await;
    assert!(result.is_ok());

    let categories = result.unwrap();
    assert_eq!(categories.len(), 4);
    
    // Verifica ordenação alfabética
    assert_eq!(categories[0].name, "Apple");
    assert_eq!(categories[1].name, "Banana");
    assert_eq!(categories[2].name, "Mango");
    assert_eq!(categories[3].name, "Zebra");
}

#[tokio::test]
async fn test_list_categories_response_has_correct_fields() {
    let (use_case, repository) = create_test_setup();

    let created_category = create_test_category(&repository, "Health").await;

    let result = use_case.execute().await;
    assert!(result.is_ok());

    let categories = result.unwrap();
    assert_eq!(categories.len(), 1);
    
    let response = &categories[0];
    assert_eq!(response.id, created_category.id);
    assert_eq!(response.name, created_category.name);
    assert_eq!(response.created_at, created_category.created_at);
}

#[tokio::test]
async fn test_list_categories_idempotent() {
    let (use_case, repository) = create_test_setup();

    create_test_category(&repository, "Category A").await;
    create_test_category(&repository, "Category B").await;

    // Executa múltiplas vezes para verificar consistência
    let result1 = use_case.execute().await.unwrap();
    let result2 = use_case.execute().await.unwrap();
    let result3 = use_case.execute().await.unwrap();

    assert_eq!(result1.len(), result2.len());
    assert_eq!(result2.len(), result3.len());
    
    assert_eq!(result1[0].id, result2[0].id);
    assert_eq!(result1[0].id, result3[0].id);
}

#[tokio::test]
async fn test_list_categories_with_special_characters() {
    let (use_case, repository) = create_test_setup();

    create_test_category(&repository, "Tech & Innovation").await;
    create_test_category(&repository, "Business/Finance").await;
    create_test_category(&repository, "Sports-Health").await;

    let result = use_case.execute().await;
    assert!(result.is_ok());

    let categories = result.unwrap();
    assert_eq!(categories.len(), 3);
}

#[tokio::test]
async fn test_list_categories_with_unicode() {
    let (use_case, repository) = create_test_setup();

    create_test_category(&repository, "Tecnologia").await;
    create_test_category(&repository, "Negócios").await;
    create_test_category(&repository, "Saúde").await;

    let result = use_case.execute().await;
    assert!(result.is_ok());

    let categories = result.unwrap();
    assert_eq!(categories.len(), 3);
    
    // Verifica que os nomes estão preservados corretamente
    let names: Vec<String> = categories.iter().map(|c| c.name.clone()).collect();
    assert!(names.contains(&"Tecnologia".to_string()));
    assert!(names.contains(&"Negócios".to_string()));
    assert!(names.contains(&"Saúde".to_string()));
}

#[tokio::test]
async fn test_list_categories_after_adding_more() {
    let (use_case, repository) = create_test_setup();

    // Adiciona algumas categorias
    create_test_category(&repository, "Category 1").await;
    create_test_category(&repository, "Category 2").await;

    let result1 = use_case.execute().await.unwrap();
    assert_eq!(result1.len(), 2);

    // Adiciona mais categorias
    create_test_category(&repository, "Category 3").await;
    create_test_category(&repository, "Category 4").await;

    let result2 = use_case.execute().await.unwrap();
    assert_eq!(result2.len(), 4);
}

#[tokio::test]
async fn test_list_categories_large_number() {
    let (use_case, repository) = create_test_setup();

    // Adiciona muitas categorias
    for i in 1..=100 {
        create_test_category(&repository, &format!("Category {}", i)).await;
    }

    let result = use_case.execute().await;
    assert!(result.is_ok());

    let categories = result.unwrap();
    assert_eq!(categories.len(), 100);
}
