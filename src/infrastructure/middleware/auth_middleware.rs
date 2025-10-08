use crate::application::services::auth_service::AuthService;
use actix_web::{HttpRequest, HttpResponse};
use std::sync::Arc;

pub struct AuthMiddleware {
    auth_service: Arc<AuthService>,
}

impl AuthMiddleware {
    pub fn new(auth_service: Arc<AuthService>) -> Self {
        Self { auth_service }
    }

    /// Extrai o token do header Authorization
    pub fn extract_token(req: &HttpRequest) -> Option<String> {
        req.headers()
            .get("Authorization")?
            .to_str()
            .ok()?
            .strip_prefix("Bearer ")
            .map(|s| s.to_string())
    }

    /// Verifica se o token JWT é válido e extrai o user_id
    pub fn verify_token(&self, token: &str) -> Result<uuid::Uuid, AuthError> {
        let claims = self.auth_service
            .verify_token(token)
            .map_err(|_| AuthError::InvalidToken)?;
        
        // Extrai o user_id do subject (sub) do claims
        uuid::Uuid::parse_str(&claims.sub)
            .map_err(|_| AuthError::InvalidToken)
    }

    /// Middleware que exige autenticação e retorna o user_id ou erro HTTP
    pub fn require_auth(&self, req: &HttpRequest) -> Result<uuid::Uuid, HttpResponse> {
        // Extrai o token do header
        let token = Self::extract_token(req)
            .ok_or_else(|| {
                HttpResponse::Unauthorized()
                    .json(serde_json::json!({
                        "error": "Missing or invalid Authorization header"
                    }))
            })?;

        // Verifica o token e retorna o user_id
        self.verify_token(&token)
            .map_err(|err| {
                HttpResponse::Unauthorized()
                    .json(serde_json::json!({
                        "error": match err {
                            AuthError::InvalidToken => "Invalid or expired token",
                        }
                    }))
            })
    }
}

#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
}
