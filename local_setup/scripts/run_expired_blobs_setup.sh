#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

export PROVING_SETUP=false
export RISC0_DEV_MODE=1 
export DEPLOY_BROKEN_DDEX_SEQUENCER=true
export BLOB_LIFETIME=2

$SCRIPT_DIR/network/launch.sh