#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT=$SCRIPT_DIR/../../..

echo "" > $PROJECT_ROOT/owen/.env.local
echo "" > $PROJECT_ROOT/validator_node/.env.local

echo "Deploying contracts..."
cd $PROJECT_ROOT/contracts
npm i
npx hardhat compile
npx hardhat run scripts/execute/fixture/deployToKurtosis.ts --network kurtosis_testnet
DDEX_SEQUENCER_ADDRESS=$(cat $PROJECT_ROOT/contracts/tmp.txt | cut -c3-)

echo "Setting DDEX_SEQUENCER_ADDRESS for validator..."
cd $PROJECT_ROOT/validator_node
echo "Building validator node..."
cargo build
echo "DDEX_SEQUENCER_ADDRESS=$DDEX_SEQUENCER_ADDRESS" >> $PROJECT_ROOT/validator_node/.env.local

echo "Setting DDEX_SEQUENCER_ADDRESS for owen..."
cd $PROJECT_ROOT/owen
echo "Building owen..."
cargo build
echo "DDEX_SEQUENCER_ADDRESS=$DDEX_SEQUENCER_ADDRESS" >> $PROJECT_ROOT/owen/.env.local

echo "All builds completed"