#!/bin/bash

cd /protocol-core/owen
DDEX_SEQUENCER_ADDRESS=$(cat /protocol-core/contracts/tmp.txt | cut -c3-) cargo run