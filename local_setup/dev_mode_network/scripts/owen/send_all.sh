#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOCKER_FOLDER=$SCRIPT_DIR/../../docker


docker compose -f $DOCKER_FOLDER/docker-compose.yml exec owen sh -c "cd owen && cargo run /protocol-core/local_setup/blobs/blob_one"
docker compose -f $DOCKER_FOLDER/docker-compose.yml exec owen sh -c "cd owen && cargo run /protocol-core/local_setup/blobs/blob_two"
docker compose -f $DOCKER_FOLDER/docker-compose.yml exec owen sh -c "cd owen && cargo run /protocol-core/local_setup/blobs/blob_three"
docker compose -f $DOCKER_FOLDER/docker-compose.yml exec owen sh -c "cd owen && cargo run /protocol-core/local_setup/blobs/blob_four"
docker compose -f $DOCKER_FOLDER/docker-compose.yml exec owen sh -c "cd owen && cargo run /protocol-core/local_setup/blobs/blob_five"
docker compose -f $DOCKER_FOLDER/docker-compose.yml exec owen sh -c "cd owen && cargo run /protocol-core/local_setup/blobs/blob_six"
docker compose -f $DOCKER_FOLDER/docker-compose.yml exec owen sh -c "cd owen && cargo run /protocol-core/local_setup/blobs/blob_seven"
docker compose -f $DOCKER_FOLDER/docker-compose.yml exec owen sh -c "cd owen && cargo run /protocol-core/local_setup/blobs/blob_eight"
docker compose -f $DOCKER_FOLDER/docker-compose.yml exec owen sh -c "cd owen && cargo run /protocol-core/local_setup/blobs/blob_nine"
docker compose -f $DOCKER_FOLDER/docker-compose.yml exec owen sh -c "cd owen && cargo run /protocol-core/local_setup/blobs/blob_ten"
docker compose -f $DOCKER_FOLDER/docker-compose.yml exec owen sh -c "cd owen && cargo run /protocol-core/local_setup/blobs/blob_eleven"
docker compose -f $DOCKER_FOLDER/docker-compose.yml exec owen sh -c "cd owen && cargo run /protocol-core/local_setup/blobs/blob_twelve"
docker compose -f $DOCKER_FOLDER/docker-compose.yml exec owen sh -c "cd owen && cargo run /protocol-core/local_setup/blobs/blob_thirteen"