#!/usr/bin/env bash
set -x
set -euo pipefail

# Check if a custom parameter has been set, otherwise use default values
DB_PORT="${POSTGRES_PORT:=5432}"
SUPERUSER="${SUPERUSER:=postgres}"
SUPERUSER_PWD="${SUPERUSER_PWD:=password}"

# Launch postgres using Docker
CONTAINER_NAME="postgres"
LISTEN_ADDR="127.0.0.1"
HOST_PORT="5432"
CONTAINER_PORT="5432"

docker run \
  --env POSTGRES_USER=${SUPERUSER} \
  --env POSTGRES_PASSWORD=${SUPERUSER_PWD} \
  --publish  ${LISTEN_ADDR}:"${HOST_PORT}":${CONTAINER_PORT} \
  --detach \
  --name "${CONTAINER_NAME}" \
postgres -N 1000
# ^ Increased maximum number of connections for testing purposes
#--publish ${IP_ADDR}:"${DB_PORT}":5432 \
