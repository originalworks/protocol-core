#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOCKER_FOLDER=$SCRIPT_DIR/../../docker

# You can run this script with BLOB_FOLDER variable set to specify the path to the blob folder.
# If not set, it defaults to /protocol-core/local_setup/blobs/blob_one

# IMPORTANT: 
# script will run OWEN inside the owen container 
# where absolute path to project root is /protocol-core

# example:
# BLOB_FOLDER=/protocol-core/local_setup/blobs/blob_one ./send_blob.sh


HOST_UID=$(id -u) HOST_GID=$(id -g) docker compose -f $DOCKER_FOLDER/docker-compose.yml exec owen sh -c "cd owen && cargo run ${BLOB_FOLDER:-/protocol-core/local_setup/blobs/blob_one}"