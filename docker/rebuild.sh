#!/bin/bash

# Script para reconstruir e reiniciar o container nexight-backend

set -e

echo "🔄 Reconstruindo container nexight-backend..."

# Cores para output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Diretório do script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Opções
FORCE_REBUILD=false
NO_CACHE=false
COMPOSE_FILE="docker-compose.yml"

# Parse argumentos
while [[ $# -gt 0 ]]; do
    case $1 in
        -f|--force)
            FORCE_REBUILD=true
            shift
            ;;
        --no-cache)
            NO_CACHE=true
            shift
            ;;
        --dev)
            COMPOSE_FILE="docker-compose.dev.yml"
            shift
            ;;
        -h|--help)
            echo "Uso: $0 [opções]"
            echo ""
            echo "Opções:"
            echo "  -f, --force     Força rebuild mesmo sem mudanças"
            echo "  --no-cache      Rebuild sem usar cache do Docker"
            echo "  --dev           Usa docker-compose.dev.yml"
            echo "  -h, --help      Mostra esta mensagem"
            exit 0
            ;;
        *)
            echo "Opção desconhecida: $1"
            echo "Use -h ou --help para ver as opções disponíveis"
            exit 1
            ;;
    esac
done

cd "$SCRIPT_DIR"

echo -e "${BLUE}📦 Parando container atual...${NC}"
docker-compose -f "$COMPOSE_FILE" stop app 2>/dev/null || true
docker-compose -f "$COMPOSE_FILE" rm -f app 2>/dev/null || true

BUILD_ARGS=""
if [ "$NO_CACHE" = true ]; then
    BUILD_ARGS="--no-cache"
    echo -e "${YELLOW}⚠️  Build sem cache (pode demorar mais)${NC}"
fi

echo -e "${BLUE}🔨 Reconstruindo imagem...${NC}"
docker-compose -f "$COMPOSE_FILE" build $BUILD_ARGS app

echo -e "${BLUE}🚀 Iniciando container...${NC}"
docker-compose -f "$COMPOSE_FILE" up -d app

echo -e "${BLUE}📋 Aguardando container iniciar...${NC}"
sleep 3

echo -e "${BLUE}📊 Status do container:${NC}"
docker-compose -f "$COMPOSE_FILE" ps app

echo ""
echo -e "${GREEN}✅ Container reconstruído e reiniciado com sucesso!${NC}"
echo ""
echo -e "${BLUE}Para ver os logs:${NC}"
echo "  docker-compose -f $COMPOSE_FILE logs -f app"
echo ""
echo -e "${BLUE}Para verificar o status:${NC}"
echo "  docker-compose -f $COMPOSE_FILE ps"
