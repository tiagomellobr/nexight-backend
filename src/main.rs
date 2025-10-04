use actix_web::{web, App, HttpServer, HttpRequest};
use dotenvy::dotenv;
use env_logger::Env;

mod infrastructure;
mod interfaces;
mod application;
mod domain;

use infrastructure::web::{ActixWebServer, Response};

/// Handler de health check usando nossos tipos abstratos
async fn health_check_handler(_req: HttpRequest, _body: web::Bytes) -> actix_web::HttpResponse {
    // Converte o request do Actix para nosso tipo abstrato (não usado neste caso)
    // let request = ActixWebServer::convert_request(&req, body);
    
    // Cria a resposta usando nossos tipos abstratos
    let response = Response::ok()
        .json(&serde_json::json!({
            "status": "ok",
            "message": "Nexight Backend API is running"
        }))
        .unwrap_or_else(|_| Response::internal_error());
    
    // Converte a resposta abstrata para HttpResponse do Actix
    ActixWebServer::convert_response(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Print inicial para garantir que stdout está funcionando
    eprintln!("Iniciando aplicação Nexight Backend...");
    
    // Carrega variáveis de ambiente
    dotenv().ok();

    // Configura logs
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Obtém configurações do servidor
    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("SERVER_PORT deve ser um número válido");

    log::info!("🚀 Iniciando Nexight Backend API em {}:{}", host, port);

    // Inicia o servidor usando Actix Web diretamente
    // mas com handlers que usam nossos tipos abstratos via adapter
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check_handler))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
