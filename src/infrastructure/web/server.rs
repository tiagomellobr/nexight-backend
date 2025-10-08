use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// Métodos HTTP suportados
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

/// Representa uma requisição HTTP de forma agnóstica ao framework
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Request {
    pub method: HttpMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

impl Request {
    /// Converte o body para JSON deserializado
    #[allow(dead_code)]
    pub fn json<T: for<'de> Deserialize<'de>>(&self) -> Result<T, serde_json::Error> {
        match &self.body {
            Some(bytes) => serde_json::from_slice(bytes),
            None => Err(serde_json::Error::io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "No body present",
            ))),
        }
    }
}

/// Representa uma resposta HTTP de forma agnóstica ao framework
#[derive(Debug, Clone)]
pub struct Response {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

impl Response {
    /// Cria uma nova resposta com status code
    pub fn new(status_code: u16) -> Self {
        Self {
            status_code,
            headers: HashMap::new(),
            body: None,
        }
    }

    /// Resposta 200 OK
    pub fn ok() -> Self {
        Self::new(200)
    }

    /// Resposta 201 Created
    pub fn created() -> Self {
        Self::new(201)
    }

    /// Resposta 204 No Content
    pub fn no_content() -> Self {
        Self::new(204)
    }

    /// Resposta 400 Bad Request
    pub fn bad_request() -> Self {
        Self::new(400)
    }

    /// Resposta 401 Unauthorized
    pub fn unauthorized() -> Self {
        Self::new(401)
    }

    /// Resposta 404 Not Found
    #[allow(dead_code)]
    pub fn not_found() -> Self {
        Self::new(404)
    }

    /// Resposta 409 Conflict
    pub fn conflict() -> Self {
        Self::new(409)
    }

    /// Resposta 500 Internal Server Error
    pub fn internal_error() -> Self {
        Self::new(500)
    }

    /// Define um header
    #[allow(dead_code)]
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Define o body como JSON
    pub fn json<T: Serialize>(mut self, data: &T) -> Result<Self, serde_json::Error> {
        self.body = Some(serde_json::to_vec(data)?);
        self.headers.insert("Content-Type".to_string(), "application/json".to_string());
        Ok(self)
    }

    /// Define o body como texto
    #[allow(dead_code)]
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.body = Some(text.into().into_bytes());
        self.headers.insert("Content-Type".to_string(), "text/plain".to_string());
        self
    }
}

/// Type alias para handlers de rotas
pub type RouteHandler = Arc<dyn Fn(Request) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>> + Send + Sync>;

/// Representa uma rota no servidor
#[allow(dead_code)]
pub struct Route {
    pub method: HttpMethod,
    pub path: String,
    pub handler: RouteHandler,
}

impl Route {
    #[allow(dead_code)]
    pub fn new(method: HttpMethod, path: impl Into<String>, handler: RouteHandler) -> Self {
        Self {
            method,
            path: path.into(),
            handler,
        }
    }
}

/// Trait que define o comportamento de um servidor web
/// Esta abstração permite trocar de framework (actix_web, warp, axum, etc.)
#[async_trait]
#[allow(dead_code)]
pub trait WebServer: Send + Sync {
    /// Adiciona uma rota ao servidor
    fn add_route(&mut self, route: Route);

    /// Inicia o servidor
    async fn start(&self, host: &str, port: u16) -> Result<(), Box<dyn std::error::Error>>;

    /// Para o servidor (para testes)
    async fn stop(&self) -> Result<(), Box<dyn std::error::Error>>;
}
