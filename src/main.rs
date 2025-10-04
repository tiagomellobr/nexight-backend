use actix_web::{web, App, HttpResponse, HttpServer, Result};
use dotenvy::dotenv;
use env_logger::Env;

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "message": "Nexight Backend API is running"
    })))
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

    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
