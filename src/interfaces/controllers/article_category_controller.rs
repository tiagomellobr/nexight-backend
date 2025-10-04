use crate::application::use_cases::list_article_categories::{
    ListArticleCategoriesError, ListArticleCategoriesUseCase,
};
use crate::infrastructure::web::Response;
use actix_web::{web, HttpRequest, HttpResponse};
use std::sync::Arc;

pub struct ArticleCategoryController {
    list_categories_use_case: Arc<ListArticleCategoriesUseCase>,
}

impl ArticleCategoryController {
    pub fn new(list_categories_use_case: Arc<ListArticleCategoriesUseCase>) -> Self {
        Self {
            list_categories_use_case,
        }
    }

    pub async fn list(&self, _req: HttpRequest, _body: web::Bytes) -> HttpResponse {
        // Execute use case
        match self.list_categories_use_case.execute().await {
            Ok(categories) => {
                let response = Response::ok()
                    .json(&serde_json::json!({
                        "data": categories,
                        "count": categories.len()
                    }))
                    .unwrap_or_else(|_| Response::internal_error());

                crate::infrastructure::web::ActixWebServer::convert_response(response)
            }
            Err(e) => {
                let error_message = match e {
                    ListArticleCategoriesError::RepositoryError(msg) => msg,
                };

                let response = Response::internal_error()
                    .json(&serde_json::json!({
                        "error": error_message
                    }))
                    .unwrap_or_else(|_| Response::internal_error());

                crate::infrastructure::web::ActixWebServer::convert_response(response)
            }
        }
    }
}
