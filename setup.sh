#!/bin/bash

# Script para configuração inicial do ambiente de desenvolvimento

echo "🚀 Configurando ambiente Nexight Backend..."

# Verificar se o Rust está instalado
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo não encontrado. Instale o Rust primeiro:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Verificar se o Docker está instalado
if ! command -v docker &> /dev/null; then
    echo "❌ Docker não encontrado. Instale o Docker primeiro."
    exit 1
fi

# Verificar se o Docker Compose está instalado
if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose não encontrado. Instale o Docker Compose primeiro."
    exit 1
fi

echo "✅ Dependências verificadas"

# Copiar arquivo de ambiente se não existir
if [ ! -f .env ]; then
    cp .env.example .env
    echo "📄 Arquivo .env criado a partir do .env.example"
    echo "   Edite o arquivo .env com suas configurações"
else
    echo "📄 Arquivo .env já existe"
fi

# Criar diretório de logs
mkdir -p logs
echo "📁 Diretório de logs criado"

# Instalar Diesel CLI se não estiver instalado
if ! command -v diesel &> /dev/null; then
    echo "🔧 Instalando Diesel CLI..."
    cargo install diesel_cli --no-default-features --features postgres
else
    echo "✅ Diesel CLI já está instalado"
fi

# Iniciar serviços Docker para desenvolvimento
echo "🐳 Iniciando serviços Docker..."
docker-compose -f docker-compose.dev.yml up -d

# Aguardar PostgreSQL ficar disponível
echo "⏳ Aguardando PostgreSQL ficar disponível..."
sleep 10

# Executar migrações
echo "🔄 Executando migrações do banco de dados..."
diesel migration run

# Build inicial
echo "🔨 Fazendo build inicial..."
cargo build

echo ""
echo "🎉 Configuração concluída!"
echo ""
echo "Para iniciar o desenvolvimento:"
echo "  cargo run"
echo ""
echo "Para executar testes:"
echo "  cargo test"
echo ""
echo "Para parar os serviços Docker:"
echo "  docker-compose -f docker-compose.dev.yml down"