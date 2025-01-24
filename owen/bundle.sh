#!/bin/bash

# Exit on any error
set -e

# Define the relative paths
CONTRACTS_DIR="../contracts"
OWEN_CLI_DIR="./owen_cli"
OWEN_CLI_BINARY="./target/release/owen_cli"
OUTPUT_ZIP="./owen_cli.zip"

# Function to check if a required command is installed
check_dependency() {
  if ! command -v "$1" &> /dev/null; then
    echo "Error: $1 is not installed. Please install $1 before running this script."
    exit 1
  fi
}

# Ensure the required tools are installed
echo "Checking dependencies..."
check_dependency cargo
check_dependency npx
check_dependency forge
echo "All dependencies are installed."

# Ensure the contracts directory exists
if [ ! -d "$CONTRACTS_DIR" ]; then
  echo "Error: Contracts directory $CONTRACTS_DIR does not exist."
  exit 1
fi

# Navigate to the contracts directory and update submodules
echo "Navigating to $CONTRACTS_DIR..."
cd "$CONTRACTS_DIR"

echo "Initializing and updating Git submodules..."
git submodule update --init --recursive
if [ $? -ne 0 ]; then
  echo "Error: Failed to initialize or update Git submodules."
  exit 1
fi
echo "Git submodules initialized and updated successfully."

# Check if Hardhat is installed locally
if [ ! -f "./node_modules/.bin/hardhat" ]; then
  echo "Error: Hardhat is not installed locally in $CONTRACTS_DIR. Installing Hardhat..."
  npm install --save-dev hardhat
  if [ $? -ne 0 ]; then
    echo "Error: Failed to install Hardhat."
    exit 1
  fi
fi

# Run Hardhat compile
echo "Running 'npx hardhat compile' in $CONTRACTS_DIR..."
npx hardhat compile
if [ $? -ne 0 ]; then
  echo "Error: Hardhat compilation failed."
  exit 1
fi
echo "Hardhat compile completed."

# Return to the /owen directory
echo "Returning to the /owen directory..."
cd - > /dev/null

# Build the Rust binary
echo "Building the owen_cli binary..."
cargo build --release --bin owen_cli
if [ $? -ne 0 ]; then
  echo "Error: Rust binary build failed."
  exit 1
fi

# Check if the binary was successfully built
if [ ! -f "$OWEN_CLI_BINARY" ]; then
  echo "Error: owen_cli binary was not found after build."
  exit 1
fi

# Prepare and package the binary
echo "Packaging the owen_cli binary..."
mkdir -p "$OWEN_CLI_DIR"
cp "$OWEN_CLI_BINARY" "$OWEN_CLI_DIR/owen_cli"
zip -r "$OUTPUT_ZIP" "$OWEN_CLI_DIR/owen_cli" > /dev/null
rm -rf "$OWEN_CLI_DIR"

echo "Packaging completed: $OUTPUT_ZIP"
echo "You may upload this file as a Lambda layer to your AWS account."
