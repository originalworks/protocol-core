#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

export PROVING_SETUP=true
export RISC0_DEV_MODE=0

$SCRIPT_DIR/network/launch.sh