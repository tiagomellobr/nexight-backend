#!/bin/bash
# Script auxiliar para executar docker-compose da pasta docker/

cd "$(dirname "$0")/docker" && docker-compose "$@"
