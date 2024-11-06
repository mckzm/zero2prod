#!/usr/bin/env bash
set -x
set -euo pipefail

# Check if a custom parameter has been set, otherwise use default values
DB_PORT="${POSTGRES_PORT:=5432}"
SUPERUSER="${SUPERUSER:=postgres}"
SUPERUSER_PWD="${SUPERUSER_PWD:=password}"

# Launch postgres using Docker
CONTAINER_NAME="postgres"

# IP address/port publishing parameters
# If an IP address is not specified, Docker publishes the port to 0.0.0.0, IOW
# all interfaces, per <https://docs.docker.com/engine/network/#published-ports>
# and <https://docs.docker.com/reference/cli/docker/container/run/#publish>.
# Per the latter, note that Docker manages its own iptables rules.
# Note as well that hosts within the same L2 segment can reach ports published
# to localhost, per issue #45610: <https://github.com/moby/moby/issues/45610>
HOST_ADDR="127.0.0.1"
HOST_PORT="5432"

docker run \
  --env POSTGRES_USER="${SUPERUSER}" \
  --env POSTGRES_PASSWORD="${SUPERUSER_PWD}" \
  --publish  ${HOST_ADDR}:"${HOST_PORT}":"${DB_PORT}" \
  --detach \
  --name "${CONTAINER_NAME}" \
postgres -N 1000
# ^ Increased maximum number of connections for testing purposes
#--publish ${IP_ADDR}:"${DB_PORT}":5432 \
