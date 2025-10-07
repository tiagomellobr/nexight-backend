use chrono::Utc;
use nexight_backend::domain::entities::article::{Article, CreateArticleDto, UpdateArticleDto};
use nexight_backend::domain::repositories::article_repository::{ArticleRepository, ArticleRepositoryError};
use nexight_backend::infrastructure::repositories::in_memory_article_repository::InMemoryArticleRepository;
use uuid::Uuid;

// Helper para criar um DTO de artigo de teste
fn create_test_article_dto(title: &str) -> CreateArticleDto {
    CreateArticleDto {
        title: title.to_string(),
        description: format!("Description for {}", title),
        link: format!("https://example.com/{}", title.to_lowercase().replace(' ', "-")),
        pub_date: Utc::now(),
        media: Some("https://example.com/image.jpg".to_string()),
        content: format!("Full content for {}", title),
        creator: "Test Author".to_string(),
        feed_id: Uuid::new_v4(),
    }
}

#[tokio::test]
async fn test_create_article() {
    let repo = InMemoryArticleRepository::new();
    let dto = create_test_article_dto("Test Article");
    let article = Article::new(dto);
    
    let result = repo.create(article.clone()).await;
    assert!(result.is_ok());
    
    let created = result.unwrap();
    assert_eq!(created.title, "Test Article");
    assert_eq!(created.creator, "Test Author");
    assert_eq!(created.processing_ai_summary, false);
}

#[tokio::test]
async fn test_create_multiple_articles() {
    let repo = InMemoryArticleRepository::new();
    
    for i in 1..=5 {
        let dto = create_test_article_dto(&format!("Article {}", i));
        let article = Article::new(dto);
        let result = repo.create(article).await;
        assert!(result.is_ok());
    }
    
    let count = repo.count().await.unwrap();
    assert_eq!(count, 5);
}

#[tokio::test]
async fn test_find_by_id_existing() {
    let repo = InMemoryArticleRepository::new();
    let dto = create_test_article_dto("Findable Article");
    let article = Article::new(dto);
    let id = article.id;
    
    repo.create(article).await.unwrap();
    let result = repo.find_by_id(id).await;
    
    assert!(result.is_ok());
    let found = result.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().title, "Findable Article");
}

#[tokio::test]
async fn test_find_by_id_non_existing() {
    let repo = InMemoryArticleRepository::new();
    let result = repo.find_by_id(Uuid::new_v4()).await;
    
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[tokio::test]
async fn test_update_article() {
    let repo = InMemoryArticleRepository::new();
    let dto = create_test_article_dto("Original Title");
    let mut article = Article::new(dto);
    let id = article.id;
    
    repo.create(article.clone()).await.unwrap();
    
    // Atualiza o artigo
    let update_dto = UpdateArticleDto {
        title: Some("Updated Title".to_string()),
        description: Some("Updated description".to_string()),
        ai_summary: Some("AI generated summary".to_string()),
        rate: Some(5),
        keywords: Some("rust, testing, article".to_string()),
        link: None,
        pub_date: None,
        media: None,
        content: None,
        creator: None,
        category_id: None,
        ai_columnist: None,
    };
    
    article.update(update_dto);
    let result = repo.update(article).await;
    
    assert!(result.is_ok());
    let updated = result.unwrap();
    assert_eq!(updated.title, "Updated Title");
    assert_eq!(updated.description, "Updated description");
    assert_eq!(updated.ai_summary, Some("AI generated summary".to_string()));
    assert_eq!(updated.rate, Some(5));
    assert_eq!(updated.keywords, Some("rust, testing, article".to_string()));
    
    // Verifica se realmente foi atualizado no repositório
    let found = repo.find_by_id(id).await.unwrap().unwrap();
    assert_eq!(found.title, "Updated Title");
}

#[tokio::test]
async fn test_update_non_existing_article() {
    let repo = InMemoryArticleRepository::new();
    let dto = create_test_article_dto("Non Existing");
    let article = Article::new(dto);
    
    let result = repo.update(article).await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ArticleRepositoryError::NotFound));
}

#[tokio::test]
async fn test_delete_article() {
    let repo = InMemoryArticleRepository::new();
    let dto = create_test_article_dto("To Be Deleted");
    let article = Article::new(dto);
    let id = article.id;
    
    repo.create(article).await.unwrap();
    
    // Verifica que existe
    assert!(repo.find_by_id(id).await.unwrap().is_some());
    
    // Deleta
    let result = repo.delete(id).await;
    assert!(result.is_ok());
    
    // Verifica que não existe mais
    assert!(repo.find_by_id(id).await.unwrap().is_none());
    
    let count = repo.count().await.unwrap();
    assert_eq!(count, 0);
}

#[tokio::test]
async fn test_delete_non_existing_article() {
    let repo = InMemoryArticleRepository::new();
    let result = repo.delete(Uuid::new_v4()).await;
    
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ArticleRepositoryError::NotFound));
}

#[tokio::test]
async fn test_list_articles_empty() {
    let repo = InMemoryArticleRepository::new();
    let result = repo.list(1, 10).await;
    
    assert!(result.is_ok());
    let paginated = result.unwrap();
    assert_eq!(paginated.articles.len(), 0);
    assert_eq!(paginated.total, 0);
    assert_eq!(paginated.page, 1);
    assert_eq!(paginated.per_page, 10);
    assert_eq!(paginated.total_pages, 0);
}

#[tokio::test]
async fn test_list_articles_with_pagination() {
    let repo = InMemoryArticleRepository::new();
    
    // Cria 25 artigos
    for i in 1..=25 {
        let dto = create_test_article_dto(&format!("Article {}", i));
        let article = Article::new(dto);
        repo.create(article).await.unwrap();
        // Pequeno delay para garantir ordem de pub_date
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
    }
    
    // Testa primeira página (10 items)
    let page1 = repo.list(1, 10).await.unwrap();
    assert_eq!(page1.articles.len(), 10);
    assert_eq!(page1.total, 25);
    assert_eq!(page1.page, 1);
    assert_eq!(page1.per_page, 10);
    assert_eq!(page1.total_pages, 3);
    
    // Testa segunda página (10 items)
    let page2 = repo.list(2, 10).await.unwrap();
    assert_eq!(page2.articles.len(), 10);
    assert_eq!(page2.total, 25);
    assert_eq!(page2.page, 2);
    
    // Testa terceira página (5 items restantes)
    let page3 = repo.list(3, 10).await.unwrap();
    assert_eq!(page3.articles.len(), 5);
    assert_eq!(page3.total, 25);
    assert_eq!(page3.page, 3);
}

#[tokio::test]
async fn test_list_articles_different_page_sizes() {
    let repo = InMemoryArticleRepository::new();
    
    // Cria 15 artigos
    for i in 1..=15 {
        let dto = create_test_article_dto(&format!("Article {}", i));
        let article = Article::new(dto);
        repo.create(article).await.unwrap();
    }
    
    // Testa com 5 por página
    let result = repo.list(1, 5).await.unwrap();
    assert_eq!(result.articles.len(), 5);
    assert_eq!(result.total_pages, 3);
    
    // Testa com 20 por página (todos de uma vez)
    let result = repo.list(1, 20).await.unwrap();
    assert_eq!(result.articles.len(), 15);
    assert_eq!(result.total_pages, 1);
}

#[tokio::test]
async fn test_list_articles_invalid_pagination() {
    let repo = InMemoryArticleRepository::new();
    
    // Página inválida (0)
    let result = repo.list(0, 10).await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ArticleRepositoryError::InvalidPagination));
    
    // Per page inválido (0)
    let result = repo.list(1, 0).await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ArticleRepositoryError::InvalidPagination));
    
    // Ambos negativos
    let result = repo.list(-1, -10).await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ArticleRepositoryError::InvalidPagination));
}

#[tokio::test]
async fn test_list_articles_beyond_last_page() {
    let repo = InMemoryArticleRepository::new();
    
    // Cria 5 artigos
    for i in 1..=5 {
        let dto = create_test_article_dto(&format!("Article {}", i));
        let article = Article::new(dto);
        repo.create(article).await.unwrap();
    }
    
    // Tenta pegar página 10 (não existe)
    let result = repo.list(10, 10).await.unwrap();
    assert_eq!(result.articles.len(), 0);
    assert_eq!(result.total, 5);
    assert_eq!(result.page, 10);
}

#[tokio::test]
async fn test_count_articles() {
    let repo = InMemoryArticleRepository::new();
    
    // Inicialmente vazio
    assert_eq!(repo.count().await.unwrap(), 0);
    
    // Adiciona 3 artigos
    for i in 1..=3 {
        let dto = create_test_article_dto(&format!("Article {}", i));
        let article = Article::new(dto);
        repo.create(article).await.unwrap();
    }
    
    assert_eq!(repo.count().await.unwrap(), 3);
    
    // Remove 1
    let dto = create_test_article_dto("To Delete");
    let article = Article::new(dto);
    let id = article.id;
    repo.create(article).await.unwrap();
    
    assert_eq!(repo.count().await.unwrap(), 4);
    
    repo.delete(id).await.unwrap();
    assert_eq!(repo.count().await.unwrap(), 3);
}

#[tokio::test]
async fn test_article_update_method() {
    let dto = create_test_article_dto("Original");
    let mut article = Article::new(dto);
    
    let original_id = article.id;
    let original_created_at = article.created_at;
    
    // Atualiza apenas alguns campos
    let update_dto = UpdateArticleDto {
        title: Some("New Title".to_string()),
        ai_summary: Some("Summary".to_string()),
        rate: Some(8),
        description: None,
        link: None,
        pub_date: None,
        media: None,
        content: None,
        creator: None,
        keywords: None,
        category_id: None,
        ai_columnist: None,
    };
    
    article.update(update_dto);
    
    // Verifica que campos atualizados mudaram
    assert_eq!(article.title, "New Title");
    assert_eq!(article.ai_summary, Some("Summary".to_string()));
    assert_eq!(article.rate, Some(8));
    
    // Verifica que campos não atualizados permaneceram
    assert_eq!(article.description, "Description for Original");
    assert_eq!(article.creator, "Test Author");
    
    // Verifica que ID não mudou
    assert_eq!(article.id, original_id);
    
    // Verifica que created_at não mudou
    assert_eq!(article.created_at, original_created_at);
    
    // Verifica que updated_at foi atualizado
    assert!(article.updated_at > original_created_at);
}
