# Docker Configuration

Esta pasta contém todos os arquivos relacionados ao Docker para o Nexight Backend.

## Arquivos

- **Dockerfile**: Configuração para build da imagem da aplicação Rust
- **docker-compose.yml**: Configuração completa com PostgreSQL, Redis e aplicação
- **docker-compose.dev.yml**: Configuração simplificada para desenvolvimento (apenas bancos)
- **nginx.conf**: Configuração do Nginx como reverse proxy (opcional)

## Como Usar

### Desenvolvimento Completo

Execute todos os serviços (PostgreSQL, Redis e aplicação):

```bash
cd docker
docker-compose up -d
```

Verificar status:
```bash
docker-compose ps
```

Parar serviços:
```bash
docker-compose down
```

### Apenas Bancos de Dados (Dev)

Se você quer rodar a aplicação localmente mas precisa dos bancos:

```bash
cd docker
docker-compose -f docker-compose.dev.yml up -d
```

Então rode a aplicação localmente:
```bash
cd ..
cargo run
```

### Reconstruir Imagem

Se você fez mudanças no código:

```bash
cd docker
docker-compose up --build -d
```

### Logs

Ver logs da aplicação:
```bash
docker-compose logs -f app
```

Ver logs de todos os serviços:
```bash
docker-compose logs -f
```

### Nginx (Produção)

O Nginx está configurado com profile `production`. Para usá-lo:

```bash
docker-compose --profile production up -d
```

## Portas

- **8005**: API Backend (Rust)
- **5432**: PostgreSQL
- **6379**: Redis
- **80/443**: Nginx (apenas com profile production)

## Variáveis de Ambiente

As variáveis podem ser configuradas no arquivo `.env` na raiz do projeto:

```env
DATABASE_NAME=nexight_db
DATABASE_USER=nexight
DATABASE_PASSWORD=nexight123
DATABASE_PORT=5432
REDIS_PORT=6379
SERVER_PORT=8005
JWT_SECRET=your-secret-key
```
