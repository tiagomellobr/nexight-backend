use crate::application::use_cases::create_article::CreateArticleUseCase;
use crate::application::use_cases::update_article::UpdateArticleUseCase;
use crate::application::use_cases::delete_article::DeleteArticleUseCase;
use crate::application::use_cases::get_article::GetArticleUseCase;
use crate::application::use_cases::list_articles::ListArticlesUseCase;
use crate::domain::entities::article::{CreateArticleDto, UpdateArticleDto};
use crate::infrastructure::web::{ActixWebServer, Response};
use crate::infrastructure::middleware::auth_middleware::AuthMiddleware;
use actix_web::{HttpRequest, HttpResponse};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

pub struct ArticleController {
    create_article_use_case: Arc<CreateArticleUseCase>,
    update_article_use_case: Arc<UpdateArticleUseCase>,
    delete_article_use_case: Arc<DeleteArticleUseCase>,
    get_article_use_case: Arc<GetArticleUseCase>,
    list_articles_use_case: Arc<ListArticlesUseCase>,
    auth_middleware: Arc<AuthMiddleware>,
}

#[derive(Debug, Deserialize)]
struct ListArticlesQuery {
    #[serde(default = "default_page")]
    page: i64,
    #[serde(default = "default_per_page")]
    per_page: i64,
}

fn default_page() -> i64 {
    1
}

fn default_per_page() -> i64 {
    20
}

impl ArticleController {
    pub fn new(
        create_article_use_case: Arc<CreateArticleUseCase>,
        update_article_use_case: Arc<UpdateArticleUseCase>,
        delete_article_use_case: Arc<DeleteArticleUseCase>,
        get_article_use_case: Arc<GetArticleUseCase>,
        list_articles_use_case: Arc<ListArticlesUseCase>,
        auth_middleware: Arc<AuthMiddleware>,
    ) -> Self {
        Self {
            create_article_use_case,
            update_article_use_case,
            delete_article_use_case,
            get_article_use_case,
            list_articles_use_case,
            auth_middleware,
        }
    }

    /// POST /articles - Cria um novo artigo
    /// Requer autenticação JWT
    pub async fn create(&self, req: HttpRequest, body: actix_web::web::Bytes) -> HttpResponse {
        // Verifica autenticação
        let _user_id = match self.auth_middleware.require_auth(&req) {
            Ok(user_id) => user_id,
            Err(response) => return response,
        };

        let request = ActixWebServer::convert_request(&req, body);

        let dto: CreateArticleDto = match request.json() {
            Ok(dto) => dto,
            Err(_) => {
                return ActixWebServer::convert_response(
                    Response::bad_request().json(&serde_json::json!({
                        "error": "Invalid request body"
                    })).unwrap_or_else(|_| Response::internal_error())
                );
            }
        };

        // Valida o DTO
        if let Err(errors) = dto.validate() {
            return ActixWebServer::convert_response(
                Response::bad_request().json(&serde_json::json!({
                    "error": "Validation failed",
                    "details": errors.to_string()
                })).unwrap_or_else(|_| Response::internal_error())
            );
        }

        match self.create_article_use_case.execute(dto).await {
            Ok(article) => {
                ActixWebServer::convert_response(
                    Response::created().json(&article).unwrap_or_else(|_| Response::internal_error())
                )
            }
            Err(e) => {
                log::error!("Failed to create article: {:?}", e);
                ActixWebServer::convert_response(
                    Response::internal_error()
                )
            }
        }
    }

    /// GET /articles/:id - Busca um artigo por ID
    pub async fn get(&self, req: HttpRequest, _body: actix_web::web::Bytes) -> HttpResponse {
        let id_str = req.match_info().get("id").unwrap_or("");
        
        let id = match Uuid::parse_str(id_str) {
            Ok(id) => id,
            Err(_) => {
                return ActixWebServer::convert_response(
                    Response::bad_request().json(&serde_json::json!({
                        "error": "Invalid article ID"
                    })).unwrap_or_else(|_| Response::internal_error())
                );
            }
        };

        match self.get_article_use_case.execute(id).await {
            Ok(Some(article)) => {
                ActixWebServer::convert_response(
                    Response::ok().json(&article).unwrap_or_else(|_| Response::internal_error())
                )
            }
            Ok(None) => {
                ActixWebServer::convert_response(
                    Response::not_found().json(&serde_json::json!({
                        "error": "Article not found"
                    })).unwrap_or_else(|_| Response::internal_error())
                )
            }
            Err(e) => {
                log::error!("Failed to get article: {:?}", e);
                ActixWebServer::convert_response(
                    Response::internal_error()
                )
            }
        }
    }

    /// GET /articles - Lista artigos com paginação
    pub async fn list(&self, req: HttpRequest, _body: actix_web::web::Bytes) -> HttpResponse {
        let query = actix_web::web::Query::<ListArticlesQuery>::from_query(req.query_string())
            .unwrap_or(actix_web::web::Query(ListArticlesQuery {
                page: 1,
                per_page: 20,
            }));

        match self.list_articles_use_case.execute(query.page, query.per_page).await {
            Ok(paginated) => {
                ActixWebServer::convert_response(
                    Response::ok().json(&paginated).unwrap_or_else(|_| Response::internal_error())
                )
            }
            Err(e) => {
                log::error!("Failed to list articles: {:?}", e);
                ActixWebServer::convert_response(
                    Response::bad_request().json(&serde_json::json!({
                        "error": format!("{:?}", e)
                    })).unwrap_or_else(|_| Response::internal_error())
                )
            }
        }
    }

    /// PUT /articles/:id - Atualiza um artigo (requer autenticação)
    pub async fn update(&self, req: HttpRequest, body: actix_web::web::Bytes) -> HttpResponse {
        // Verifica autenticação
        if let Err(response) = self.auth_middleware.require_auth(&req) {
            log::warn!("Unauthorized attempt to update article");
            return response;
        }

        let id_str = req.match_info().get("id").unwrap_or("");
        
        let id = match Uuid::parse_str(id_str) {
            Ok(id) => id,
            Err(_) => {
                return ActixWebServer::convert_response(
                    Response::bad_request().json(&serde_json::json!({
                        "error": "Invalid article ID"
                    })).unwrap_or_else(|_| Response::internal_error())
                );
            }
        };

        let request = ActixWebServer::convert_request(&req, body);

        let dto: UpdateArticleDto = match request.json() {
            Ok(dto) => dto,
            Err(_) => {
                return ActixWebServer::convert_response(
                    Response::bad_request().json(&serde_json::json!({
                        "error": "Invalid request body"
                    })).unwrap_or_else(|_| Response::internal_error())
                );
            }
        };

        match self.update_article_use_case.execute(id, dto).await {
            Ok(article) => {
                ActixWebServer::convert_response(
                    Response::ok().json(&article).unwrap_or_else(|_| Response::internal_error())
                )
            }
            Err(e) => {
                log::error!("Failed to update article: {:?}", e);
                let response = match e {
                    crate::domain::repositories::article_repository::ArticleRepositoryError::NotFound => {
                        Response::not_found().json(&serde_json::json!({
                            "error": "Article not found"
                        })).unwrap_or_else(|_| Response::internal_error())
                    }
                    _ => Response::internal_error()
                };
                ActixWebServer::convert_response(response)
            }
        }
    }

    /// DELETE /articles/:id - Remove um artigo (requer autenticação)
    pub async fn delete(&self, req: HttpRequest, _body: actix_web::web::Bytes) -> HttpResponse {
        // Verifica autenticação
        if let Err(response) = self.auth_middleware.require_auth(&req) {
            log::warn!("Unauthorized attempt to delete article");
            return response;
        }

        let id_str = req.match_info().get("id").unwrap_or("");
        
        let id = match Uuid::parse_str(id_str) {
            Ok(id) => id,
            Err(_) => {
                return ActixWebServer::convert_response(
                    Response::bad_request().json(&serde_json::json!({
                        "error": "Invalid article ID"
                    })).unwrap_or_else(|_| Response::internal_error())
                );
            }
        };

        match self.delete_article_use_case.execute(id).await {
            Ok(_) => {
                ActixWebServer::convert_response(
                    Response::no_content()
                )
            }
            Err(e) => {
                log::error!("Failed to delete article: {:?}", e);
                let response = match e {
                    crate::domain::repositories::article_repository::ArticleRepositoryError::NotFound => {
                        Response::not_found().json(&serde_json::json!({
                            "error": "Article not found"
                        })).unwrap_or_else(|_| Response::internal_error())
                    }
                    _ => Response::internal_error()
                };
                ActixWebServer::convert_response(response)
            }
        }
    }
}
