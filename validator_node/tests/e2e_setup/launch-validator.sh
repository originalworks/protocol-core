#!/bin/bash

cd /protocol-core/validator_node
DDEX_SEQUENCER_ADDRESS=$(cat /protocol-core/contracts/tmp.txt | cut -c3-) cargo run