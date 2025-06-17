#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOCKER_FOLDER=$SCRIPT_DIR/../../docker

# This script will turn off one of three validators
# VALIDATOR variable define which validator to turn off:
#
#   VALIDATOR=1 ./turn_off.sh
#
# this will turn off the validator 1 in its container
# Available validators: 1, 2, 3

# If you omit VALIDATOR variable, it will try to turn off validator 1 by default

VALIDATOR_CONTAINER_NAME="validator_node_${VALIDATOR:-1}"

docker compose -f $DOCKER_FOLDER/docker-compose.yml exec $VALIDATOR_CONTAINER_NAME sh -c "pkill -f validator_node"