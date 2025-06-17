#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOCKER_FOLDER=$SCRIPT_DIR/../../docker


# This script will only work with the local setup run with `run_expired_blob_setup.sh`
# It will send a broken non-existing blob to the the BrokenDdexSequencer contract


docker compose -f $DOCKER_FOLDER/docker-compose.yml exec owen sh -c "cd contracts && npx hardhat run scripts/execute/submitBrokenBlob.ts --network kurtosis_testnet"