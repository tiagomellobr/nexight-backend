#!/bin/bash

# Script simplificado para rebuild do container na raiz do projeto

cd "$(dirname "$0")/docker"
exec ./rebuild.sh "$@"
