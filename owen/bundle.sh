#!/bin/bash

# Exit on any error except for the dependency checks
set -e

# Define the relative paths
CONTRACTS_DIR="../contracts"
OWEN_CLI_DIR="./owen_cli"
OWEN_CLI_BINARY="./target/release/main"
OUTPUT_ZIP="./owen_cli.zip"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
RESET='\033[0m'

# Array to store missing dependencies
missing_deps=()

# Function that checks if a given command is installed
# and prints a green check or red cross
check_dependency() {
  if command -v "$1" &> /dev/null; then
    printf "  ${GREEN}✔${RESET} %s\n" "$1"
  else
    printf "  ${RED}✘${RESET} %s\n" "$1"
    missing_deps+=("$1")
  fi
}

# Function that prints installation instructions for a given missing dependency
print_install_instructions() {
  case "$1" in
    python3)
      echo "  - Install Python 3.11 with:"
      echo "    sudo apt install -y python3.11 python3.11-venv python3.11-dev pip"
      ;;
    cargo)
      echo "  - Install Rust and Cargo with:"
      echo "    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
      echo "    source \$HOME/.cargo/env"
      ;;
    git)
      echo "  - Install Git with:"
      echo "    sudo apt install git -y"
      ;;
    forge)
      echo "  - Install Foundry with:"
      echo "    curl -L https://foundry.paradigm.xyz | bash"
      echo "    source $HOME/.bashrc"
      echo "    foundryup"
      ;;
    npm)
      echo "  - Install Node.js (includes npm) with:"
      echo "    sudo apt install nodejs npm -y"
      ;;
    curl)
      echo "  - Install curl with:"
      echo "    sudo apt install curl -y"
      ;;
    pkg-config)
      echo "  - Install pkg-config with:"
      echo "    sudo apt install pkg-config -y"
      ;;
    openssl)
      echo "  - Install OpenSSL development headers with:"
      echo "    sudo apt install libssl-dev -y"
      ;;
    npx)
      echo "  - Install Node.js (includes npx) with:"
      echo "    sudo apt install nodejs npm -y"
      ;;
    zip)
      echo "  - Install zip with:"
      echo "    sudo apt install zip -y"
      ;;
    *)
      echo "  - Install $1 using your package manager."
      ;;
  esac
}

echo "Checking dependencies..."

# List of all required dependencies
required_deps=(
  git
  python3
  curl
  cargo
  npm
  npx
  forge
  pkg-config
  openssl
  zip
)

# Display the dependency check table
echo "Dependencies:"
for dep in "${required_deps[@]}"; do
  check_dependency "$dep"
done

# If any dependencies are missing, print instructions and exit
if [ "${#missing_deps[@]}" -ne 0 ]; then
  echo ""
  echo "Some dependencies are missing. Please install them and re-run the script:"
  for dep in "${missing_deps[@]}"; do
    print_install_instructions "$dep"
  done
  echo ""
  echo "Exiting now..."
  exit 1
fi

echo "All dependencies are installed."

###############################
# 2) HARDHAT / RUST WORKFLOW  #
###############################

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
  echo "Hardhat is not installed locally in $CONTRACTS_DIR. Installing Hardhat..."
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


#####################################
# 3) Prepare and package the binary #
#####################################
# Build the Rust binary
echo "Building the owen_cli binary..."
cargo build --release --bin main
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
