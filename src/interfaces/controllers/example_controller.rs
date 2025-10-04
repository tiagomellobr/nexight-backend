use actix_web::{web, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::infrastructure::web::{ActixWebServer, Response};

#[derive(Debug, Serialize, Deserialize)]
pub struct EchoRequest {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct EchoResponse {
    pub original: String,
    pub reversed: String,
    pub length: usize,
}

/// Exemplo de handler que demonstra o uso completo do adapter
/// 
/// Este handler:
/// 1. Converte a requisição do Actix para nosso tipo abstrato
/// 2. Faz parsing do JSON do body
/// 3. Processa os dados (lógica de negócio)
/// 4. Cria uma resposta usando nossos tipos abstratos
/// 5. Converte a resposta para HttpResponse do Actix
pub async fn echo_handler(req: HttpRequest, body: web::Bytes) -> actix_web::HttpResponse {
    // 1. Converte a requisição
    let request = ActixWebServer::convert_request(&req, body);
    
    // 2. Parse do JSON
    let echo_req: EchoRequest = match request.json() {
        Ok(data) => data,
        Err(e) => {
            log::error!("Failed to parse JSON: {}", e);
            let response = Response::bad_request()
                .json(&serde_json::json!({
                    "error": "Invalid JSON",
                    "details": e.to_string()
                }))
                .unwrap_or_else(|_| Response::internal_error());
            return ActixWebServer::convert_response(response);
        }
    };
    
    // 3. Lógica de negócio (totalmente desacoplada do framework)
    let reversed: String = echo_req.message.chars().rev().collect();
    let length = echo_req.message.len();
    
    let echo_resp = EchoResponse {
        original: echo_req.message,
        reversed,
        length,
    };
    
    // 4. Cria a resposta usando tipos abstratos
    let response = match Response::ok().json(&echo_resp) {
        Ok(r) => r,
        Err(e) => {
            log::error!("Failed to serialize response: {}", e);
            Response::internal_error()
                .json(&serde_json::json!({
                    "error": "Internal server error"
                }))
                .unwrap_or_else(|_| Response::internal_error())
        }
    };
    
    // 5. Converte para HttpResponse do Actix
    ActixWebServer::convert_response(response)
}

/// Handler que demonstra acesso a headers e query params
pub async fn info_handler(req: HttpRequest, body: web::Bytes) -> actix_web::HttpResponse {
    let request = ActixWebServer::convert_request(&req, body);
    
    // Acessa informações da requisição de forma agnóstica
    let info = serde_json::json!({
        "method": format!("{:?}", request.method),
        "path": request.path,
        "headers_count": request.headers.len(),
        "query_params": request.query_params,
        "user_agent": request.headers.get("user-agent").cloned(),
    });
    
    let response = Response::ok()
        .json(&info)
        .unwrap_or_else(|_| Response::internal_error());
    
    ActixWebServer::convert_response(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_web::test]
    async fn test_echo_handler() {
        let req = test::TestRequest::default().to_http_request();
        let payload = web::Bytes::from(r#"{"message":"hello"}"#);
        
        let resp = echo_handler(req, payload).await;
        
        assert_eq!(resp.status(), 200);
    }
}
