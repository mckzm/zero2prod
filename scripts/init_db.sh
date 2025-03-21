#!/usr/bin/env bash
set -x
set -euo pipefail

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install --version='~0.8' sqlx-cli --no-default-features --features rustls,postgres"
  echo >&2 "to install it."
  exit 1
fi

# Check if a custom parameter has been set, otherwise use default values
DB_PORT="${POSTGRES_PORT:=5432}"
SUPERUSER="${SUPERUSER:=postgres}"
SUPERUSER_PWD="${SUPERUSER_PWD:=opensesame}"
APP_USER="${APP_USER:=zero2prod}"
APP_USER_PWD="${APP_USER_PWD:=shazam}"
APP_DB_NAME="${APP_DB_NAME:=newsletter}"


# IP address/port publishing parameters
# If an IP address is not specified, Docker publishes the port to 0.0.0.0, IOW
# all interfaces, per <https://docs.docker.com/engine/network/#published-ports>
# and <https://docs.docker.com/reference/cli/docker/container/run/#publish>.
# Per the latter, note that Docker manages its own iptables rules.
# Note as well that hosts within the same L2 segment can reach ports published
# to localhost, per issue #45610: <https://github.com/moby/moby/issues/45610>
HOST_ADDR="127.0.0.1"
HOST_PORT="5432"

# Allow to skip Docker if a dockerized Postgres database is already running
SKIP_DOCKER="${SKIP_DOCKER:=""}"
 if [[ -z "${SKIP_DOCKER}" ]]
then
  # Launch postgres using Docker
  CONTAINER_NAME="postgres"
  docker run \
    --env POSTGRES_USER="${SUPERUSER}" \
    --env POSTGRES_PASSWORD="${SUPERUSER_PWD}" \
    --health-cmd="pg_isready -U ${SUPERUSER} || exit 1" \
    --health-interval=1s \
    --health-timeout=5s \
    --health-retries=5 \
    --publish  ${HOST_ADDR}:"${HOST_PORT}":"${DB_PORT}" \
    --detach \
    --name "${CONTAINER_NAME}" \
  postgres -N 1000
  # ^ Increased maximum number of connections for testing purposes

  # Wait for Postgres to be ready to accept connections
  until [ \
    "$(docker inspect -f "{{.State.Health.Status}}" ${CONTAINER_NAME})" == \
    "healthy" \
  ]; do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1
  done

  # Create the application user
  CREATE_QUERY="CREATE USER ${APP_USER} WITH PASSWORD '${APP_USER_PWD}';"
  docker exec -it "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "${CREATE_QUERY}"

  # Grant CREATEDB privileges to the application user
  GRANT_QUERY="ALTER USER ${APP_USER} CREATEDB;"
  docker exec -it "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "${GRANT_QUERY}"
 fi

>&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations now!"

# Create the application database
if [ -d migrations ]; then
  DATABASE_URL="postgres://${APP_USER}:${APP_USER_PWD}@localhost:${DB_PORT}/${APP_DB_NAME}"
  export DATABASE_URL
  sqlx database create
  sqlx migrate run
  >&2 echo "Postgres has been migrated, ready to go!"
else
  >&2 echo "The 'migrations' directory does not exist! Aborting."
fi

