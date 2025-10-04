# Nexight Backend API

Backend API em Rust para o sistema Nexight, construído com Actix-web.

## 🚀 Início Rápido

### Opção 1: Tudo no Docker (Recomendado)
```bash
./docker-compose.sh up -d
```

Verificar status:
```bash
./docker-compose.sh ps
```

Testar API:
```bash
curl http://localhost:8005/health
```

### Opção 2: Desenvolvimento Local (apenas bancos no Docker)
```bash
# Iniciar apenas PostgreSQL e Redis
cd docker
docker-compose -f docker-compose.dev.yml up -d

# Voltar para raiz e rodar aplicação
cd ..
cargo run
```

### Comandos Úteis

**Docker:**
```bash
./docker-compose.sh logs -f        # Ver logs
./docker-compose.sh down           # Parar tudo
./docker-compose.sh up --build -d  # Rebuild
```

**Banco de Dados:**
```bash
diesel migration generate nome     # Criar migração
diesel migration run               # Rodar migrações
diesel migration revert            # Reverter migração
```

**Testes:**
```bash
cargo test                         # Rodar testes
cargo test -- --nocapture          # Com logs
```

### Portas
- **8005**: API Backend
- **5432**: PostgreSQL
- **6379**: Redis

### Estrutura Simplificada
```
nexight-backend/
├── docker/              # Arquivos Docker
├── src/                 # Código fonte
├── migrations/          # Migrações do banco
├── docker-compose.sh    # Script auxiliar
└── README.md           # Esta documentação
```

> 💡 **Dica**: Veja a [documentação Docker](docker/README.md) para mais opções de configuração.

---

## Características

- **Arquitetura Hexagonal**: Organização em camadas bem definidas
- **Clean Architecture**: Separação clara de responsabilidades
- **Domain-Driven Design**: Modelagem baseada no domínio do negócio
- **Test-Driven Development**: Desenvolvimento orientado a testes
- **Princípios SOLID**: Código maintível e extensível
- **Event-Driven Architecture**: Sistema de eventos para comunicação entre contextos

## Tecnologias

- **Rust**: Linguagem principal
- **Actix-web**: Framework web
- **Diesel**: ORM para interação com banco de dados
- **PostgreSQL**: Banco de dados principal
- **Redis**: Cache e sessões
- **Tokio**: Runtime assíncrono
- **Argon2**: Hash de senhas
- **Docker**: Containerização

## Estrutura do Projeto

```
src/
├── application/       # Casos de uso e lógica de aplicação
│   ├── services/      # Serviços de aplicação
│   └── use_cases/     # Casos de uso específicos
├── domain/            # Modelos de domínio e regras de negócio
│   ├── entities/      # Entidades do domínio
│   └── repositories/  # Interfaces de repositórios
├── infrastructure/    # Implementações específicas de infraestrutura
│   ├── database/      # Configurações e conexões de BD
│   └── repositories/  # Implementações concretas dos repositórios
└── interfaces/        # Interfaces de entrada
    ├── controllers/   # Controladores HTTP
    └── middleware/    # Middlewares
```

## Pré-requisitos

- Rust 1.75 ou superior
- Docker e Docker Compose
- PostgreSQL (caso não use Docker)
- Redis (caso não use Docker)

## Configuração

1. **Clone o repositório:**
   ```bash
   git clone <repository-url>
   cd nexight-backend
   ```

2. **Configure as variáveis de ambiente:**
   ```bash
   cp .env.example .env
   # Edite o arquivo .env com suas configurações
   ```

3. **Instale as dependências:**
   ```bash
   cargo build
   ```

## Desenvolvimento

### Usando Docker (Recomendado)

1. **Inicie todos os serviços (PostgreSQL, Redis e aplicação):**
   ```bash
   cd docker
   docker-compose up -d
   ```

   Ou use o script auxiliar da raiz:
   ```bash
   ./docker-compose.sh up -d
   ```

2. **Apenas bancos de dados (para desenvolvimento local):**
   ```bash
   cd docker
   docker-compose -f docker-compose.dev.yml up -d
   ```

   Depois execute a aplicação localmente:
   ```bash
   cargo run
   ```

3. **Ver logs:**
   ```bash
   cd docker
   docker-compose logs -f app
   ```

4. **Parar serviços:**
   ```bash
   cd docker
   docker-compose down
   ```

> 📁 **Nota**: Todos os arquivos Docker estão na pasta `docker/`. Veja `docker/README.md` para mais detalhes.

### Sem Docker

1. **Certifique-se de que PostgreSQL e Redis estão rodando**

2. **Configure as variáveis de ambiente no .env**

3. **Execute as migrações:**
   ```bash
   diesel migration run
   ```

4. **Inicie a aplicação:**
   ```bash
   cargo run
   ```

## Testes

```bash
# Executar todos os testes
cargo test

# Executar testes com logs
cargo test -- --nocapture

# Executar testes de integração
cargo test --test integration_tests
```

## Produção

### Build da aplicação

```bash
cargo build --release
```

### Docker

```bash
# Build e executar todos os serviços
cd docker
docker-compose up --build -d

# Com Nginx (profile production)
docker-compose --profile production up -d

# Apenas build da imagem
docker build -f docker/Dockerfile -t nexight-backend .
```

> 📁 Ver `docker/README.md` para mais opções e configurações.

## API Endpoints

### Autenticação
- `POST /auth/register` - Registrar usuário
- `POST /auth/login` - Login
- `POST /auth/logout` - Logout
- `POST /auth/refresh` - Renovar token

### Usuários
- `GET /users/me` - Perfil do usuário autenticado
- `PUT /users/me` - Atualizar perfil
- `DELETE /users/me` - Deletar conta

### Health Check
- `GET /health` - Status da aplicação

## Configuração de Ambiente

### Variáveis Principais

| Variável | Descrição | Padrão |
|----------|-----------|---------|
| `SERVER_HOST` | Host do servidor | `0.0.0.0` |
| `SERVER_PORT` | Porta do servidor | `8005` |
| `DATABASE_URL` | URL de conexão do PostgreSQL | - |
| `REDIS_URL` | URL de conexão do Redis | - |
| `JWT_SECRET` | Chave secreta para JWT | - |
| `RUST_LOG` | Nível de log | `info` |

## Contribuição

1. Fork o projeto
2. Crie uma branch para sua feature (`git checkout -b feature/nova-feature`)
3. Commit suas mudanças (`git commit -am 'Adiciona nova feature'`)
4. Push para a branch (`git push origin feature/nova-feature`)
5. Abra um Pull Request

## Licença

Este projeto está sob a licença MIT. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.

## Suporte

Para dúvidas ou problemas, abra uma issue no GitHub ou entre em contato com a equipe de desenvolvimento.