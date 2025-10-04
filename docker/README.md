# 🐳 Docker Configuration - Nexight Backend

Esta pasta contém todos os arquivos relacionados ao Docker para o Nexight Backend.

## 📁 Arquivos

- **Dockerfile**: Configuração multi-stage para build otimizado da aplicação Rust
- **docker-compose.yml**: Configuração completa com PostgreSQL, Redis e aplicação (produção)
- **docker-compose.dev.yml**: Configuração para desenvolvimento com hot-reload (cargo-watch)
- **nginx.conf**: Configuração do Nginx como reverse proxy (opcional)
- **rebuild.sh**: Script para reconstruir e reiniciar o container facilmente

---

## ⚡ Início Rápido (TL;DR)

### Para Desenvolvimento (com hot-reload):
```bash
cd docker
docker-compose -f docker-compose.dev.yml up
```
✅ Detecta mudanças automaticamente e recompila!

### Para Produção:
```bash
cd docker
docker-compose up -d
```

### Para Atualizar Código no Container de Produção:
```bash
# Da raiz do projeto:
./rebuild.sh

# Ou da pasta docker:
./rebuild.sh
```

---

---

## � Como Atualizar o Código no Container

### 🎯 O Problema
O container `nexight-backend` usa um **Dockerfile multi-stage otimizado** que copia o código apenas durante o build da imagem. Isso significa que quando você altera arquivos `.rs`, essas mudanças **não aparecem no container** até que você reconstrua a imagem.

### ✅ Soluções Disponíveis

### ✅ Soluções Disponíveis

#### 1️⃣ Modo Desenvolvimento com Hot Reload (RECOMENDADO)

O arquivo `docker-compose.dev.yml` inclui um serviço `app-dev` que:
- ✅ Monta seu código como volume
- ✅ Usa `cargo-watch` para detectar mudanças
- ✅ Recompila automaticamente quando você salva arquivos
- ✅ Mantém cache de dependências (builds mais rápidos)

**Como usar:**
```bash
cd docker
docker-compose -f docker-compose.dev.yml up
```

**O que acontece:**
Quando você modificar qualquer arquivo `.rs`, verá no terminal:
```
app-dev | [Running 'cargo run']
app-dev | Compiling nexight-backend...
app-dev | Finished dev [unoptimized + debuginfo] target(s)
app-dev | Running `target/debug/nexight-backend`
```

**Vantagens:**
- ✅ Detecta mudanças automaticamente
- ✅ Não precisa rebuild manual
- ✅ Ideal para desenvolvimento diário
- ✅ Cache de dependências (primeira vez demora, depois é rápido)

**Desvantagens:**
- ⚠️ Primeira compilação pode demorar (baixa dependências)
- ⚠️ Usa mais recursos (monta código como volume)

#### 2️⃣ Script de Rebuild Rápido (Para Produção)

Criamos um script `rebuild.sh` que facilita a reconstrução do container de produção:

#### 2️⃣ Script de Rebuild Rápido (Para Produção)

Criamos um script `rebuild.sh` que facilita a reconstrução do container de produção:

**Como usar:**
```bash
# Da raiz do projeto:
./rebuild.sh

# Ou da pasta docker:
cd docker
./rebuild.sh
```

**Opções disponíveis:**
```bash
./rebuild.sh              # Rebuild normal
./rebuild.sh --no-cache   # Rebuild sem cache (limpa cache antigo)
./rebuild.sh --dev        # Rebuild do ambiente dev
./rebuild.sh --help       # Ver todas opções
```

**O que o script faz:**
1. Para o container atual
2. Remove o container antigo
3. Reconstrói a imagem
4. Inicia o novo container
5. Mostra o status e instruções para ver logs

**Vantagens:**
- ✅ Simples e rápido
- ✅ Build otimizado (multi-stage)
- ✅ Um único comando

**Desvantagens:**
- ⚠️ Precisa rebuild manual após cada mudança
- ⚠️ Build pode demorar alguns minutos

#### 3️⃣ Comandos Docker Manuais (Controle Total)

#### 3️⃣ Comandos Docker Manuais (Controle Total)

Se preferir fazer manualmente sem o script:

```bash
cd docker

# Parar e remover container
docker-compose stop app
docker-compose rm -f app

# Reconstruir imagem
docker-compose build app

# Iniciar novo container
docker-compose up -d app

# Ver logs
docker-compose logs -f app
```

**Vantagens:**
- ✅ Controle total sobre cada etapa
- ✅ Útil para troubleshooting

**Desvantagens:**
- ⚠️ Mais comandos para executar
- ⚠️ Manual

---

## 📊 Comparação das Opções

| Opção | Vantagens | Desvantagens | Quando Usar |
|-------|-----------|--------------|-------------|
| **🔥 Hot Reload (dev)** | Automático<br>Rápido após 1ª vez<br>Sem rebuild | Primeira vez demora<br>Usa mais recursos | **Desenvolvimento diário** |
| **⚡ rebuild.sh** | Simples<br>Build otimizado<br>Um comando | Manual<br>Demora alguns minutos | **Testar build produção** |
| **🔧 Docker Manual** | Controle total<br>Flexível | Vários comandos<br>Mais complexo | **Troubleshooting** |

---

## 📋 Como Usar - Guia Completo

### 🔥 Modo 1: Desenvolvimento com Hot Reload (Recomendado)

Execute todos os serviços com hot-reload automático:

```bash
cd docker
docker-compose -f docker-compose.dev.yml up
```

A aplicação será reiniciada automaticamente quando você modificar arquivos `.rs`.

**Testar a API:**
```bash
curl http://localhost:8005/health
```

### ⚡ Modo 2: Produção/Testing Completo

Execute todos os serviços (PostgreSQL, Redis e aplicação em modo produção):

```bash
cd docker
docker-compose up -d
```

**Verificar status:**
```bash
docker-compose ps
```

**Parar serviços:**
```bash
docker-compose down
```

**Atualizar código após mudanças:**
```bash
# Da raiz:
../rebuild.sh

# Ou da pasta docker:
./rebuild.sh
```

### 🗄️ Modo 3: Apenas Bancos de Dados

Se você quer rodar a aplicação localmente (`cargo run`) mas precisa dos bancos:

```bash
cd docker
docker-compose -f docker-compose.dev.yml up postgres-dev redis-dev -d
```

Então rode a aplicação localmente:
```bash
cd ..
cargo run
```

---

## � Workflow Recomendado

### Durante o Desenvolvimento Diário:
```bash
# 1. Subir ambiente dev com hot-reload (uma vez)
cd docker
docker-compose -f docker-compose.dev.yml up

# 2. Codificar normalmente em seu editor
# 3. Salvar arquivos .rs
# 4. Aguardar recompilação automática (5-10s)
# 5. Testar em http://localhost:8005
```

### Antes de Commit/Push:
```bash
# 1. Testar build de produção
cd docker
./rebuild.sh

# 2. Verificar se tudo funciona
curl http://localhost:8005/health

# 3. Ver logs se necessário
docker-compose logs -f app

# 4. Parar ambiente de teste
docker-compose down
```

---

---

## 📝 Comandos Úteis

### Ver Status dos Containers

```bash
cd docker

# Status de todos os containers
docker-compose ps

# Status detalhado
docker ps -a | grep nexight
```

### Ver Logs em Tempo Real
### Ver Logs em Tempo Real

```bash
cd docker

# Logs da aplicação (produção)
docker-compose logs -f app

# Logs da aplicação (desenvolvimento)
docker-compose -f docker-compose.dev.yml logs -f app-dev

# Logs de todos os serviços
docker-compose logs -f

# Últimas 100 linhas
docker-compose logs --tail=100 app
```

### Acessar Shell do Container

```bash
# Container de produção
docker exec -it nexight-backend bash

# Container de desenvolvimento
docker exec -it nexight-backend-dev bash

# PostgreSQL
docker exec -it nexight-postgres psql -U nexight -d nexight_db

# Redis
docker exec -it nexight-redis redis-cli
```

### Gerenciar Containers

```bash
cd docker

# Parar todos os containers
docker-compose down
docker-compose -f docker-compose.dev.yml down

# Parar e remover volumes (CUIDADO: apaga dados)
docker-compose down -v

# Reiniciar um serviço específico
docker-compose restart app
docker-compose restart postgres

# Parar apenas um serviço
docker-compose stop app

# Iniciar apenas um serviço
docker-compose start app
```

### Limpar e Rebuild Completo

```bash
cd docker

# Limpar tudo e recomeçar
docker-compose down -v
docker-compose build --no-cache
docker-compose up -d

# Ou usando o script
cd ..
./rebuild.sh --no-cache
```

### Nginx (Reverse Proxy - Produção)

O Nginx está configurado com profile `production`. Para usá-lo:

```bash
cd docker

# Iniciar com Nginx
docker-compose --profile production up -d

# Ver logs do Nginx
docker-compose logs -f nginx
```

---

## 🔍 Troubleshooting

### ❌ Container não atualiza o código

**Problema:** Você modifica arquivos `.rs` mas o container não reflete as mudanças.

**Soluções:**
1. **Melhor:** Use `docker-compose.dev.yml` com hot-reload:
   ```bash
   cd docker
   docker-compose down
   docker-compose -f docker-compose.dev.yml up
   ```

2. **Alternativa:** Execute rebuild:
   ```bash
   ./rebuild.sh
   ```

3. **Manual:** Rebuild completo:
   ```bash
   cd docker
   docker-compose build app && docker-compose up -d app
   ```

### 🐌 Build muito lento

**Problema:** Build demora muito tempo.

**Soluções:**
- **Primeira vez:** É normal, está baixando dependências (pode levar 5-10 min)
- **Usar cache:** Não use `--no-cache` a menos que necessário
- **Modo dev:** Use `docker-compose.dev.yml` que mantém cache em volumes:
  ```bash
  cd docker
  docker-compose -f docker-compose.dev.yml up
  ```
- **Limpar apenas se necessário:** Só use `--no-cache` se houver problemas de cache

### 🔄 Container fica reiniciando constantemente

**Problema:** Container inicia e para repetidamente.

**Diagnóstico:**
```bash
cd docker

# Ver logs para identificar o erro
docker-compose logs app

# Ver últimas 50 linhas
docker-compose logs --tail=50 app

# Verificar status
docker-compose ps
```

**Possíveis causas:**
1. **Erro no código:** Verifique os logs de compilação
2. **Banco não disponível:** Verifique se PostgreSQL está rodando:
   ```bash
   docker-compose ps postgres
   docker-compose logs postgres
   ```
3. **Variáveis de ambiente:** Verifique arquivo `.env`

**Solução:**
```bash
# Parar tudo
docker-compose down

# Ver se há erros de sintaxe
cd ..
cargo check

# Reiniciar do zero
cd docker
docker-compose up -d
```

### 🔌 Erro de conexão com banco de dados

**Problema:** `could not connect to server` ou similar.

**Diagnóstico:**
```bash
cd docker

# Verificar se PostgreSQL está rodando
docker-compose ps postgres

# Ver logs do PostgreSQL
docker-compose logs postgres

# Testar conexão manualmente
docker exec -it nexight-postgres psql -U nexight -d nexight_db
```

**Solução:**
```bash
# Reiniciar PostgreSQL
docker-compose restart postgres

# Ou reiniciar tudo
docker-compose down
docker-compose up -d
```

### 🔐 Erro de autenticação no PostgreSQL

**Problema:** `password authentication failed`.

**Solução:**
```bash
cd docker

# Remover volumes e recriar
docker-compose down -v
docker-compose up -d postgres

# Aguardar PostgreSQL inicializar
sleep 10

# Iniciar aplicação
docker-compose up -d app
```

### 💾 Disco cheio / Limpar espaço

**Problema:** Docker usando muito espaço em disco.

**Solução:**
```bash
# Ver uso de espaço
docker system df

# Limpar containers parados, redes não usadas, etc
docker system prune

# Limpar tudo (CUIDADO: remove volumes também)
docker system prune -a --volumes

# Limpar apenas imagens não usadas
docker image prune -a

# Limpar apenas este projeto
cd docker
docker-compose down -v
docker-compose build --no-cache
```

### 🚫 Porta já em uso

**Problema:** `port is already allocated`.

**Solução:**
```bash
# Descobrir o que está usando a porta (ex: 8005)
lsof -i :8005

# Parar o processo
kill -9 <PID>

# Ou mudar a porta no .env
echo "SERVER_PORT=8006" >> .env

# Reiniciar
cd docker
docker-compose down
docker-compose up -d
```

### 🔧 Problemas com cargo-watch no modo dev

**Problema:** cargo-watch não detecta mudanças ou fica travado.

**Solução:**
```bash
cd docker

# Parar ambiente dev
docker-compose -f docker-compose.dev.yml down

# Limpar volumes de cache
docker-compose -f docker-compose.dev.yml down -v

# Reiniciar
docker-compose -f docker-compose.dev.yml up --build
```

### 📦 Erro ao compilar dependências

**Problema:** Erro ao compilar crates do Rust.

**Solução:**
```bash
# Limpar cache e rebuild sem cache
./rebuild.sh --no-cache

# Ou manualmente
cd docker
docker-compose down
docker-compose build --no-cache app
docker-compose up -d app
```

---

## 🌐 Portas Utilizadas

---

## 🌐 Portas Utilizadas

| Serviço | Porta | Descrição |
|---------|-------|-----------|
| **Backend API** | `8005` | API REST Rust (Actix-web) |
| **PostgreSQL** | `5432` | Banco de dados principal |
| **Redis** | `6379` | Cache e sessões |
| **Nginx** | `80/443` | Reverse proxy (apenas com `--profile production`) |

---

## ⚙️ Variáveis de Ambiente

As variáveis podem ser configuradas no arquivo `.env` na raiz do projeto:

```env
# Banco de Dados
DATABASE_NAME=nexight_db
DATABASE_USER=nexight
DATABASE_PASSWORD=nexight123
DATABASE_PORT=5432

# Redis
REDIS_PORT=6379

# Servidor
SERVER_HOST=0.0.0.0
SERVER_PORT=8005

# Segurança
JWT_SECRET=your-super-secret-jwt-key

# Logging
RUST_LOG=info  # ou: debug, warn, error
```

**Criar arquivo .env:**
```bash
# Na raiz do projeto
cat > .env << EOF
DATABASE_NAME=nexight_db
DATABASE_USER=nexight
DATABASE_PASSWORD=nexight123
DATABASE_PORT=5432
REDIS_PORT=6379
SERVER_PORT=8005
JWT_SECRET=$(openssl rand -hex 32)
RUST_LOG=debug
EOF
```

---

## 📚 Recursos Adicionais

### Endpoints da API

```bash
# Health check
curl http://localhost:8005/health

# Criar usuário (exemplo)
curl -X POST http://localhost:8005/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","password":"password123"}'

# Login (exemplo)
curl -X POST http://localhost:8005/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","password":"password123"}'
```

### Comandos Docker Úteis

```bash
# Ver uso de recursos
docker stats

# Ver logs de sistema
docker events

# Inspecionar container
docker inspect nexight-backend

# Ver redes
docker network ls
docker network inspect nexight-network

# Ver volumes
docker volume ls
docker volume inspect nexight_postgres_data
```

### Migrações do Banco de Dados

```bash
# Criar nova migração
diesel migration generate nome_da_migracao

# Executar migrações
diesel migration run

# Reverter última migração
diesel migration revert

# Ver status das migrações
diesel migration list
```

---

## ❓ FAQ (Perguntas Frequentes)

**P: Por que o container não atualiza automaticamente quando mudo o código?**  
R: O Dockerfile usa build multi-stage que copia o código durante o build. Use `docker-compose.dev.yml` com hot-reload ou execute `./rebuild.sh` após mudanças.

**P: Qual modo devo usar para desenvolvimento diário?**  
R: Use `docker-compose -f docker-compose.dev.yml up` para hot-reload automático. É a melhor experiência de desenvolvimento.

**P: Como faço para testar a versão de produção?**  
R: Use `docker-compose up -d` (sem o `-f docker-compose.dev.yml`). Para atualizar código, use `./rebuild.sh`.

**P: O rebuild é muito lento, o que fazer?**  
R: Para desenvolvimento, use `docker-compose.dev.yml` que mantém cache. Evite `--no-cache` a menos que tenha problemas de cache.

**P: Posso usar hot-reload no docker-compose.yml principal?**  
R: Sim! Descomente as linhas de volume no serviço `app` do `docker-compose.yml` e modifique o Dockerfile para usar cargo-watch.

**P: Como vejo os logs em tempo real?**  
R: Use `docker-compose logs -f app` (produção) ou `docker-compose -f docker-compose.dev.yml logs -f app-dev` (dev).

**P: Como limpo tudo e começo do zero?**  
R: Execute `docker-compose down -v` (remove volumes também) e depois `./rebuild.sh --no-cache`.

**P: Onde ficam os dados do PostgreSQL?**  
R: Em um volume Docker chamado `nexight_postgres_data`. Use `docker volume inspect nexight_postgres_data` para ver detalhes.

**P: Como acesso o banco de dados diretamente?**  
R: `docker exec -it nexight-postgres psql -U nexight -d nexight_db`

**P: Posso rodar múltiplas instâncias?**  
R: Sim, mas você precisa mudar as portas no `.env` para evitar conflitos.

---

## 📖 Estrutura dos Arquivos Docker

### Dockerfile (Multi-stage Build)
```dockerfile
# Estágio 1: Builder - compila a aplicação
FROM rust:latest AS builder
# ... instala dependências, compila código ...

# Estágio 2: Runtime - imagem final enxuta
FROM debian:bookworm-slim
# ... copia apenas o binário compilado ...
```

**Vantagens:**
- ✅ Imagem final pequena (~100MB vs ~2GB)
- ✅ Apenas runtime dependencies
- ✅ Mais seguro (sem ferramentas de build)

### docker-compose.yml (Produção)
- PostgreSQL com healthcheck
- Redis com persistência
- App Rust (build otimizado)
- Nginx opcional (profile production)

### docker-compose.dev.yml (Desenvolvimento)
- PostgreSQL e Redis (apenas serviços)
- App com volumes montados
- cargo-watch para hot-reload
- Volumes separados para cache

---

## 🎯 Checklist de Uso

### Setup Inicial
- [ ] Criar arquivo `.env` com configurações
- [ ] Verificar que Docker e Docker Compose estão instalados
- [ ] Escolher modo de desenvolvimento (dev ou produção)

### Desenvolvimento Diário
- [ ] Iniciar com `docker-compose -f docker-compose.dev.yml up`
- [ ] Codificar normalmente
- [ ] Testar mudanças automaticamente recompiladas
- [ ] Ver logs em tempo real se necessário

### Antes de Commit
- [ ] Testar build de produção com `./rebuild.sh`
- [ ] Verificar que tudo funciona
- [ ] Rodar testes (se houver)
- [ ] Verificar logs para erros

### Troubleshooting
- [ ] Ver logs: `docker-compose logs -f app`
- [ ] Verificar status: `docker-compose ps`
- [ ] Se necessário, rebuild sem cache: `./rebuild.sh --no-cache`
- [ ] Em último caso, limpar tudo: `docker-compose down -v`

---

## 🤝 Contribuindo

Se você encontrar problemas ou tiver sugestões para melhorar a configuração Docker:

1. Verifique se o problema já existe nos logs
2. Tente as soluções do Troubleshooting
3. Se o problema persistir, documente:
   - Passos para reproduzir
   - Logs relevantes
   - Sistema operacional
   - Versões do Docker/Docker Compose

---

## 📝 Notas Importantes

- **Produção:** Use sempre `docker-compose.yml` para builds otimizados
- **Desenvolvimento:** Use `docker-compose.dev.yml` para melhor experiência
- **Cache:** Volumes de cache aceleram muito os builds subsequentes
- **Segurança:** Nunca commite o arquivo `.env` com senhas reais
- **Performance:** Primeira compilação sempre demora, seja paciente
- **Logs:** Sempre verifique os logs quando algo der errado

---

**Documentação criada em:** 4 de outubro de 2025  
**Última atualização:** 4 de outubro de 2025


