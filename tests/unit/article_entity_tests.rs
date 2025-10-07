use chrono::Utc;
use nexight_backend::domain::entities::article::{Article, CreateArticleDto, UpdateArticleDto};
use uuid::Uuid;

#[test]
fn test_create_article_with_dto() {
    let feed_id = Uuid::new_v4();
    let dto = CreateArticleDto {
        title: "Test Article".to_string(),
        description: "Test description".to_string(),
        link: "https://example.com/article".to_string(),
        pub_date: Utc::now(),
        media: Some("https://example.com/image.jpg".to_string()),
        content: "Full article content".to_string(),
        creator: "John Doe".to_string(),
        feed_id,
    };

    let article = Article::new(dto);

    assert_eq!(article.title, "Test Article");
    assert_eq!(article.description, "Test description");
    assert_eq!(article.link, "https://example.com/article");
    assert_eq!(article.media, Some("https://example.com/image.jpg".to_string()));
    assert_eq!(article.content, "Full article content");
    assert_eq!(article.creator, "John Doe");
    assert_eq!(article.feed_id, feed_id);
    
    // Verifica valores padrão
    assert_eq!(article.ai_summary, None);
    assert_eq!(article.rate, None);
    assert_eq!(article.keywords, None);
    assert_eq!(article.processing_ai_summary, false);
    assert_eq!(article.processing_rating, false);
    assert_eq!(article.processing_keywords, false);
    assert_eq!(article.category_id, None);
    assert_eq!(article.processing_categorizing, false);
    assert_eq!(article.ai_columnist, None);
    assert_eq!(article.processing_columnist, false);
}

#[test]
fn test_update_article_partial_fields() {
    let dto = CreateArticleDto {
        title: "Original Title".to_string(),
        description: "Original description".to_string(),
        link: "https://example.com/original".to_string(),
        pub_date: Utc::now(),
        media: None,
        content: "Original content".to_string(),
        creator: "Original Author".to_string(),
        feed_id: Uuid::new_v4(),
    };

    let mut article = Article::new(dto);
    let original_created_at = article.created_at;
    let original_id = article.id;

    // Aguarda um pouco para garantir que updated_at será diferente
    std::thread::sleep(std::time::Duration::from_millis(10));

    // Atualiza apenas alguns campos
    let update_dto = UpdateArticleDto {
        title: Some("Updated Title".to_string()),
        description: None,
        link: None,
        pub_date: None,
        media: None,
        content: None,
        creator: None,
        ai_summary: Some("AI Summary".to_string()),
        rate: Some(7),
        keywords: Some("rust, test".to_string()),
        category_id: Some(Uuid::new_v4()),
        ai_columnist: Some("AI Columnist".to_string()),
    };

    article.update(update_dto);

    // Verifica campos atualizados
    assert_eq!(article.title, "Updated Title");
    assert_eq!(article.ai_summary, Some("AI Summary".to_string()));
    assert_eq!(article.rate, Some(7));
    assert_eq!(article.keywords, Some("rust, test".to_string()));
    assert_eq!(article.ai_columnist, Some("AI Columnist".to_string()));
    assert!(article.category_id.is_some());

    // Verifica campos não atualizados
    assert_eq!(article.description, "Original description");
    assert_eq!(article.link, "https://example.com/original");
    assert_eq!(article.content, "Original content");
    assert_eq!(article.creator, "Original Author");

    // Verifica que ID não mudou
    assert_eq!(article.id, original_id);

    // Verifica que created_at não mudou
    assert_eq!(article.created_at, original_created_at);

    // Verifica que updated_at foi atualizado
    assert!(article.updated_at > original_created_at);
}

#[test]
fn test_update_article_all_fields() {
    let dto = CreateArticleDto {
        title: "Original".to_string(),
        description: "Original".to_string(),
        link: "https://example.com/original".to_string(),
        pub_date: Utc::now(),
        media: Some("original.jpg".to_string()),
        content: "Original".to_string(),
        creator: "Original".to_string(),
        feed_id: Uuid::new_v4(),
    };

    let mut article = Article::new(dto);
    let new_pub_date = Utc::now();

    let update_dto = UpdateArticleDto {
        title: Some("New Title".to_string()),
        description: Some("New Description".to_string()),
        link: Some("https://example.com/new".to_string()),
        pub_date: Some(new_pub_date),
        media: Some("new.jpg".to_string()),
        content: Some("New Content".to_string()),
        creator: Some("New Creator".to_string()),
        ai_summary: Some("New Summary".to_string()),
        rate: Some(9),
        keywords: Some("new, keywords".to_string()),
        category_id: Some(Uuid::new_v4()),
        ai_columnist: Some("New Columnist".to_string()),
    };

    article.update(update_dto);

    assert_eq!(article.title, "New Title");
    assert_eq!(article.description, "New Description");
    assert_eq!(article.link, "https://example.com/new");
    assert_eq!(article.pub_date, new_pub_date);
    assert_eq!(article.media, Some("new.jpg".to_string()));
    assert_eq!(article.content, "New Content");
    assert_eq!(article.creator, "New Creator");
    assert_eq!(article.ai_summary, Some("New Summary".to_string()));
    assert_eq!(article.rate, Some(9));
    assert_eq!(article.keywords, Some("new, keywords".to_string()));
    assert!(article.category_id.is_some());
    assert_eq!(article.ai_columnist, Some("New Columnist".to_string()));
}

#[test]
fn test_article_to_response() {
    use nexight_backend::domain::entities::article::ArticleResponse;
    
    let dto = CreateArticleDto {
        title: "Test".to_string(),
        description: "Description".to_string(),
        link: "https://example.com".to_string(),
        pub_date: Utc::now(),
        media: Some("image.jpg".to_string()),
        content: "Content".to_string(),
        creator: "Creator".to_string(),
        feed_id: Uuid::new_v4(),
    };

    let article = Article::new(dto);
    let article_id = article.id;
    let article_feed_id = article.feed_id;

    let response: ArticleResponse = article.into();

    assert_eq!(response.id, article_id);
    assert_eq!(response.title, "Test");
    assert_eq!(response.description, "Description");
    assert_eq!(response.link, "https://example.com");
    assert_eq!(response.media, Some("image.jpg".to_string()));
    assert_eq!(response.content, "Content");
    assert_eq!(response.creator, "Creator");
    assert_eq!(response.feed_id, article_feed_id);
    assert_eq!(response.ai_summary, None);
    assert_eq!(response.rate, None);
    assert_eq!(response.keywords, None);
}

#[test]
fn test_article_clone() {
    let dto = CreateArticleDto {
        title: "Clone Test".to_string(),
        description: "Description".to_string(),
        link: "https://example.com".to_string(),
        pub_date: Utc::now(),
        media: None,
        content: "Content".to_string(),
        creator: "Creator".to_string(),
        feed_id: Uuid::new_v4(),
    };

    let article1 = Article::new(dto);
    let article2 = article1.clone();

    assert_eq!(article1.id, article2.id);
    assert_eq!(article1.title, article2.title);
    assert_eq!(article1.description, article2.description);
    assert_eq!(article1.created_at, article2.created_at);
}

#[test]
fn test_article_equality() {
    let dto = CreateArticleDto {
        title: "Test".to_string(),
        description: "Description".to_string(),
        link: "https://example.com".to_string(),
        pub_date: Utc::now(),
        media: None,
        content: "Content".to_string(),
        creator: "Creator".to_string(),
        feed_id: Uuid::new_v4(),
    };

    let article1 = Article::new(dto.clone());
    let article2 = article1.clone();

    assert_eq!(article1, article2);
}
