#!/bin/bash

# Exit on any error
set -e

# Define the relative path to the contracts directory
CONTRACTS_DIR="../contracts"

# Ensure the contracts directory exists
if [ ! -d "$CONTRACTS_DIR" ]; then
  echo "Error: Contracts directory $CONTRACTS_DIR does not exist."
  exit 1
fi

# Navigate to the contracts directory
echo "Navigating to $CONTRACTS_DIR..."
cd "$CONTRACTS_DIR"

# Run the Hardhat compile command
echo "Running 'npx hardhat compile' in $CONTRACTS_DIR..."
npx hardhat compile
echo "Hardhat compile completed. Proceeding with other tasks..."

# Return to the owen folder
echo "Returning to the /owen directory..."
cd -

cargo build --release --bin owen_cli
mkdir -p ./owen_cli
yes | cp -rf ./target/release/owen_cli ./owen_cli/owen_cli
zip -r ./owen_cli.zip ./owen_cli/owen_cli
rm -rf ./owen_cli