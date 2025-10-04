use crate::application::use_cases::login_user::{LoginUserError, LoginUserUseCase};
use crate::application::use_cases::register_user::{RegisterUserError, RegisterUserUseCase};
use crate::domain::entities::user::{CreateUserDto, LoginDto};
use crate::infrastructure::web::Response;
use actix_web::{web, HttpRequest, HttpResponse};
use std::sync::Arc;

pub struct AuthController {
    register_use_case: Arc<RegisterUserUseCase>,
    login_use_case: Arc<LoginUserUseCase>,
}

impl AuthController {
    pub fn new(
        register_use_case: Arc<RegisterUserUseCase>,
        login_use_case: Arc<LoginUserUseCase>,
    ) -> Self {
        Self {
            register_use_case,
            login_use_case,
        }
    }

    pub async fn register(
        &self,
        _req: HttpRequest,
        body: web::Bytes,
    ) -> HttpResponse {
        // Parse request body
        let dto: CreateUserDto = match serde_json::from_slice(&body) {
            Ok(dto) => dto,
            Err(e) => {
                let response = Response::bad_request()
                    .json(&serde_json::json!({
                        "error": "Invalid request body",
                        "details": e.to_string()
                    }))
                    .unwrap_or_else(|_| Response::internal_error());
                
                return crate::infrastructure::web::ActixWebServer::convert_response(response);
            }
        };

        // Execute use case
        match self.register_use_case.execute(dto).await {
            Ok(auth_response) => {
                let response = Response::created()
                    .json(&auth_response)
                    .unwrap_or_else(|_| Response::internal_error());
                
                crate::infrastructure::web::ActixWebServer::convert_response(response)
            }
            Err(e) => {
                let (status_code, error_message) = match e {
                    RegisterUserError::ValidationError(msg) => (400, msg),
                    RegisterUserError::EmailAlreadyExists => {
                        (409, "Email already in use".to_string())
                    }
                    RegisterUserError::PasswordHashError(msg) => (500, msg),
                    RegisterUserError::RepositoryError(msg) => (500, msg),
                    RegisterUserError::TokenError(msg) => (500, msg),
                };

                let response = if status_code >= 500 {
                    Response::internal_error()
                        .json(&serde_json::json!({
                            "error": error_message
                        }))
                        .unwrap_or_else(|_| Response::internal_error())
                } else if status_code == 409 {
                    Response::conflict()
                        .json(&serde_json::json!({
                            "error": error_message
                        }))
                        .unwrap_or_else(|_| Response::internal_error())
                } else {
                    Response::bad_request()
                        .json(&serde_json::json!({
                            "error": error_message
                        }))
                        .unwrap_or_else(|_| Response::internal_error())
                };

                crate::infrastructure::web::ActixWebServer::convert_response(response)
            }
        }
    }

    pub async fn login(
        &self,
        _req: HttpRequest,
        body: web::Bytes,
    ) -> HttpResponse {
        // Parse request body
        let dto: LoginDto = match serde_json::from_slice(&body) {
            Ok(dto) => dto,
            Err(e) => {
                let response = Response::bad_request()
                    .json(&serde_json::json!({
                        "error": "Invalid request body",
                        "details": e.to_string()
                    }))
                    .unwrap_or_else(|_| Response::internal_error());
                
                return crate::infrastructure::web::ActixWebServer::convert_response(response);
            }
        };

        // Execute use case
        match self.login_use_case.execute(dto).await {
            Ok(auth_response) => {
                let response = Response::ok()
                    .json(&auth_response)
                    .unwrap_or_else(|_| Response::internal_error());
                
                crate::infrastructure::web::ActixWebServer::convert_response(response)
            }
            Err(e) => {
                let (status_code, error_message) = match e {
                    LoginUserError::ValidationError(msg) => (400, msg),
                    LoginUserError::InvalidCredentials => {
                        (401, "Invalid email or password".to_string())
                    }
                    LoginUserError::TokenError(msg) => (500, msg),
                    LoginUserError::RepositoryError(msg) => (500, msg),
                };

                let response = if status_code >= 500 {
                    Response::internal_error()
                        .json(&serde_json::json!({
                            "error": error_message
                        }))
                        .unwrap_or_else(|_| Response::internal_error())
                } else if status_code == 401 {
                    Response::unauthorized()
                        .json(&serde_json::json!({
                            "error": error_message
                        }))
                        .unwrap_or_else(|_| Response::internal_error())
                } else {
                    Response::bad_request()
                        .json(&serde_json::json!({
                            "error": error_message
                        }))
                        .unwrap_or_else(|_| Response::internal_error())
                };

                crate::infrastructure::web::ActixWebServer::convert_response(response)
            }
        }
    }
}
