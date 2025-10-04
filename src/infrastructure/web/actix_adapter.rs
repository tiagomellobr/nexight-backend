use actix_web::{web, HttpRequest, HttpResponse};
use async_trait::async_trait;

use super::server::{HttpMethod, Request, Response, Route, WebServer};

/// Adapter para usar Actix Web como implementação do WebServer
/// 
/// Nota: Esta implementação é simplificada. Para o Actix Web,
/// as rotas devem ser configuradas diretamente no main.rs usando
/// os métodos de conversão públicos fornecidos.
pub struct ActixWebServer;

impl ActixWebServer {
    pub fn new() -> Self {
        Self
    }

    /// Converte uma requisição do Actix Web para nosso tipo abstrato
    /// 
    /// Esta função não é usada atualmente mas está disponível caso
    /// precisemos processar requisições de forma agnóstica ao framework
    #[allow(dead_code)]
    pub fn convert_request(req: &HttpRequest, body: web::Bytes) -> Request {
        let method = match req.method().as_str() {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            "PATCH" => HttpMethod::PATCH,
            "HEAD" => HttpMethod::HEAD,
            "OPTIONS" => HttpMethod::OPTIONS,
            _ => HttpMethod::GET, // fallback
        };

        let mut headers = std::collections::HashMap::new();
        for (key, value) in req.headers() {
            if let Ok(val_str) = value.to_str() {
                headers.insert(key.as_str().to_string(), val_str.to_string());
            }
        }

        let mut query_params = std::collections::HashMap::new();
        for (key, value) in req.query_string().split('&').filter_map(|s| {
            let mut parts = s.split('=');
            Some((parts.next()?, parts.next().unwrap_or("")))
        }) {
            query_params.insert(key.to_string(), value.to_string());
        }

        Request {
            method,
            path: req.path().to_string(),
            headers,
            query_params,
            body: if body.is_empty() {
                None
            } else {
                Some(body.to_vec())
            },
        }
    }

    /// Converte nosso Response abstrato para HttpResponse do Actix
    pub fn convert_response(response: Response) -> HttpResponse {
        let mut builder = HttpResponse::build(
            actix_web::http::StatusCode::from_u16(response.status_code)
                .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
        );

        for (key, value) in response.headers {
            builder.insert_header((key, value));
        }

        match response.body {
            Some(body) => builder.body(body),
            None => builder.finish(),
        }
    }
}

impl Default for ActixWebServer {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl WebServer for ActixWebServer {
    fn add_route(&mut self, _route: Route) {
        // Para o Actix, as rotas são configuradas diretamente no HttpServer
        // Esta implementação não suporta adicionar rotas dinamicamente via trait
        log::warn!("add_route não é suportado pelo ActixWebServer via trait. Use os métodos convert_request/convert_response.");
    }

    async fn start(&self, _host: &str, _port: u16) -> Result<(), Box<dyn std::error::Error>> {
        // O start real deve ser feito via ActixWebServer::run()
        // Este método existe apenas para satisfazer a trait
        Err("Use ActixWebServer::run() ao invés de start()".into())
    }

    async fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Actix não possui um método explícito de stop no runtime
        // Em produção, isso seria controlado por signals
        Ok(())
    }
}