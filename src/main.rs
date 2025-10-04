use actix_web::{web, App, HttpServer, HttpRequest};
use dotenvy::dotenv;
use env_logger::Env;
use std::sync::Arc;

mod infrastructure;
mod interfaces;
mod application;
mod domain;

use infrastructure::web::{ActixWebServer, Response};
use infrastructure::database::{establish_connection_pool, run_migrations};
use infrastructure::repositories::diesel_user_repository::DieselUserRepository;
use application::services::auth_service::AuthService;
use application::use_cases::register_user::RegisterUserUseCase;
use application::use_cases::login_user::LoginUserUseCase;
use interfaces::controllers::auth_controller::AuthController;

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

    // Estabelece pool de conexões com banco de dados
    let db_pool = establish_connection_pool();
    
    // Executa migrações
    run_migrations(&db_pool);

    // Cria repositórios
    let user_repository = Arc::new(DieselUserRepository::new(db_pool.clone()));

    // Cria serviços
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string());
    let token_expiration_hours = std::env::var("TOKEN_EXPIRATION_HOURS")
        .unwrap_or_else(|_| "24".to_string())
        .parse::<i64>()
        .unwrap_or(24);
    
    let auth_service = Arc::new(AuthService::new(jwt_secret, token_expiration_hours));

    // Cria use cases
    let register_use_case = Arc::new(RegisterUserUseCase::new(
        user_repository.clone(),
        auth_service.clone(),
    ));
    let login_use_case = Arc::new(LoginUserUseCase::new(
        user_repository.clone(),
        auth_service.clone(),
    ));

    // Cria controllers
    let auth_controller = Arc::new(AuthController::new(
        register_use_case,
        login_use_case,
    ));

    // Obtém configurações do servidor
    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("SERVER_PORT deve ser um número válido");

    log::info!("🚀 Iniciando Nexight Backend API em {}:{}", host, port);

    // Inicia o servidor usando Actix Web diretamente
    // mas com handlers que usam nossos tipos abstratos via adapter
    HttpServer::new(move || {
        let auth_ctrl = auth_controller.clone();
        
        App::new()
            .route("/health", web::get().to(health_check_handler))
            .route("/auth/register", web::post().to({
                let ctrl = auth_ctrl.clone();
                move |req: HttpRequest, body: web::Bytes| {
                    let controller = ctrl.clone();
                    async move { controller.register(req, body).await }
                }
            }))
            .route("/auth/login", web::post().to({
                let ctrl = auth_ctrl.clone();
                move |req: HttpRequest, body: web::Bytes| {
                    let controller = ctrl.clone();
                    async move { controller.login(req, body).await }
                }
            }))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
