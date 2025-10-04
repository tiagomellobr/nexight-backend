# Multi-stage build para otimizar o tamanho da imagem
FROM rust:latest AS builder

# Instalar dependências do sistema necessárias
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Criar diretório de trabalho
WORKDIR /app

# Copiar arquivos de configuração primeiro para aproveitar o cache do Docker
COPY Cargo.toml Cargo.lock ./

# Criar src/main.rs dummy para compilar dependências
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Compilar dependências
RUN cargo build --release && rm -rf src

# Copiar código fonte e migrations
COPY src ./src
COPY migrations ./migrations

# Recompilar com o código real
RUN cargo build --release

# Estágio final - imagem mais enxuta
FROM debian:bookworm-slim

# Instalar dependências de runtime
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Criar usuário não-root
RUN useradd -r -s /bin/false nexight

# Criar diretório de trabalho
WORKDIR /app

# Copiar binário do estágio anterior
COPY --from=builder /app/target/release/nexight-backend .

# Mudar proprietário do arquivo
RUN chown nexight:nexight nexight-backend

# Trocar para usuário não-root
USER nexight

# Expor porta
EXPOSE 8005

# Comando para executar a aplicação
ENTRYPOINT ["./nexight-backend"]