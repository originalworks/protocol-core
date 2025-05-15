#!/bin/bash

cd contracts
npm i
npx hardhat compile
npx hardhat run scripts/execute/fixture/deployToKurtosis.ts --network kurtosis_testnet

cd ../validator_node
cargo build

cd ../owen
cargo build