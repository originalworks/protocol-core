#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOCKER_FOLDER=$SCRIPT_DIR/../../docker

# This script will turn on one of three validators
# VALIDATOR variable define which validator to turn on:
#
#   VALIDATOR=1 ./turn_on.sh
#
# this will run the validator 1 in its container
# Available validators: 1, 2, 3

# If you omit VALIDATOR variable, it will run validator 1 by default

VALIDATOR_CONTAINER_NAME="validator_node_${VALIDATOR:-1}"

HOST_UID=$(id -u) HOST_GID=$(id -g) docker compose -f $DOCKER_FOLDER/docker-compose.yml exec $VALIDATOR_CONTAINER_NAME sh -c "cd validator_node && cargo run"