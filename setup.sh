#!/bin/sh

set -a
. .env
set +a

envsubst '${LISTEN_PORT}' < nginx.conf.template > nginx.conf

docker-compose up --build -d