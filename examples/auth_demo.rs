// Exemplo de uso dos use cases de autenticação

use nexight_backend::application::services::auth_service::AuthService;
use nexight_backend::application::use_cases::register_user::RegisterUserUseCase;
use nexight_backend::application::use_cases::login_user::LoginUserUseCase;
use nexight_backend::domain::entities::user::{CreateUserDto, LoginDto};
use nexight_backend::infrastructure::repositories::in_memory_user_repository::InMemoryUserRepository;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Exemplo de Uso - Sistema de Autenticação JWT\n");

    // 1. Setup: Criar dependências
    println!("📦 Configurando dependências...");
    let repository = Arc::new(InMemoryUserRepository::new());
    let auth_service = Arc::new(AuthService::new(
        "super_secret_key_change_in_production".to_string(),
        24, // Token expira em 24 horas
    ));

    // 2. Criar use cases
    let register_use_case = RegisterUserUseCase::new(repository.clone(), auth_service.clone());
    let login_use_case = LoginUserUseCase::new(repository.clone(), auth_service.clone());

    println!("✅ Dependências configuradas!\n");

    // 3. CENÁRIO 1: Registrar um novo usuário
    println!("=== CENÁRIO 1: Registro de Usuário ===");
    let create_dto = CreateUserDto {
        email: "joao.silva@example.com".to_string(),
        password: "senha_segura_123".to_string(),
        name: "João Silva".to_string(),
    };

    println!("📝 Registrando usuário: {}", create_dto.email);
    match register_use_case.execute(create_dto).await {
        Ok(response) => {
            println!("✅ Usuário registrado com sucesso!");
            println!("   - ID: {}", response.user.id);
            println!("   - Nome: {}", response.user.name);
            println!("   - Email: {}", response.user.email);
            println!("   - Token JWT: {}...", &response.token[..50]);
            
            // Verificar se o token é válido
            match auth_service.verify_token(&response.token) {
                Ok(claims) => {
                    println!("\n🔐 Token verificado com sucesso!");
                    println!("   - User ID: {}", claims.sub);
                    println!("   - Email: {}", claims.email);
                    println!("   - Expira em: {} segundos", claims.exp - claims.iat);
                }
                Err(e) => println!("❌ Erro ao verificar token: {}", e),
            }
        }
        Err(e) => println!("❌ Erro ao registrar: {}", e),
    }

    println!("\n");

    // 4. CENÁRIO 2: Tentar registrar com email duplicado
    println!("=== CENÁRIO 2: Email Duplicado ===");
    let duplicate_dto = CreateUserDto {
        email: "joao.silva@example.com".to_string(), // Mesmo email
        password: "outra_senha_123".to_string(),
        name: "Outro João".to_string(),
    };

    println!("📝 Tentando registrar com email duplicado...");
    match register_use_case.execute(duplicate_dto).await {
        Ok(_) => println!("❌ Não deveria ter permitido!"),
        Err(e) => println!("✅ Erro esperado: {}", e),
    }

    println!("\n");

    // 5. CENÁRIO 3: Login com credenciais corretas
    println!("=== CENÁRIO 3: Login Bem-Sucedido ===");
    let login_dto = LoginDto {
        email: "joao.silva@example.com".to_string(),
        password: "senha_segura_123".to_string(),
    };

    println!("🔑 Fazendo login com: {}", login_dto.email);
    match login_use_case.execute(login_dto).await {
        Ok(response) => {
            println!("✅ Login realizado com sucesso!");
            println!("   - Nome: {}", response.user.name);
            println!("   - Token JWT: {}...", &response.token[..50]);
        }
        Err(e) => println!("❌ Erro no login: {}", e),
    }

    println!("\n");

    // 6. CENÁRIO 4: Login com senha incorreta
    println!("=== CENÁRIO 4: Senha Incorreta ===");
    let wrong_password_dto = LoginDto {
        email: "joao.silva@example.com".to_string(),
        password: "senha_errada".to_string(),
    };

    println!("🔑 Tentando login com senha incorreta...");
    match login_use_case.execute(wrong_password_dto).await {
        Ok(_) => println!("❌ Não deveria ter permitido!"),
        Err(e) => println!("✅ Erro esperado: {}", e),
    }

    println!("\n");

    // 7. CENÁRIO 5: Login com usuário inexistente
    println!("=== CENÁRIO 5: Usuário Inexistente ===");
    let nonexistent_dto = LoginDto {
        email: "naoexiste@example.com".to_string(),
        password: "qualquer_senha".to_string(),
    };

    println!("🔑 Tentando login com usuário inexistente...");
    match login_use_case.execute(nonexistent_dto).await {
        Ok(_) => println!("❌ Não deveria ter permitido!"),
        Err(e) => println!("✅ Erro esperado: {}", e),
    }

    println!("\n");

    // 8. CENÁRIO 6: Validação de dados
    println!("=== CENÁRIO 6: Validações ===");
    
    // Email inválido
    let invalid_email_dto = CreateUserDto {
        email: "email-invalido".to_string(),
        password: "senha_segura_123".to_string(),
        name: "Teste".to_string(),
    };
    
    println!("📝 Tentando registrar com email inválido...");
    match register_use_case.execute(invalid_email_dto).await {
        Ok(_) => println!("❌ Não deveria ter permitido!"),
        Err(e) => println!("✅ Erro de validação: {}", e),
    }

    // Senha muito curta
    let short_password_dto = CreateUserDto {
        email: "novo@example.com".to_string(),
        password: "123".to_string(), // Muito curta
        name: "Teste".to_string(),
    };
    
    println!("📝 Tentando registrar com senha curta...");
    match register_use_case.execute(short_password_dto).await {
        Ok(_) => println!("❌ Não deveria ter permitido!"),
        Err(e) => println!("✅ Erro de validação: {}", e),
    }

    println!("\n");

    // 9. CENÁRIO 7: Múltiplos usuários
    println!("=== CENÁRIO 7: Múltiplos Usuários ===");
    let users = vec![
        ("maria@example.com", "Maria Santos"),
        ("pedro@example.com", "Pedro Oliveira"),
        ("ana@example.com", "Ana Costa"),
    ];

    for (email, name) in users {
        let dto = CreateUserDto {
            email: email.to_string(),
            password: "senha123456".to_string(),
            name: name.to_string(),
        };

        match register_use_case.execute(dto).await {
            Ok(response) => {
                println!("✅ {} registrado(a) - ID: {}", name, response.user.id);
            }
            Err(e) => println!("❌ Erro ao registrar {}: {}", name, e),
        }
    }

    println!("\n🎉 Demonstração concluída!");
    println!("\n💡 Observações:");
    println!("   - Todas as senhas são hasheadas com Argon2");
    println!("   - Tokens JWT são gerados automaticamente no registro e login");
    println!("   - Emails duplicados são rejeitados");
    println!("   - Validações são aplicadas antes de qualquer operação");
    println!("   - Repositório em memória (perfeito para testes!)");

    Ok(())
}
