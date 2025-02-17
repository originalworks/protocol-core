#!/bin/bash

# Exit on any error unless specifically handled
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

# Arrays to store missing items
missing_deps=()
missing_python_packages=()

#############################################
# Functions to check commands/dependencies  #
#############################################

# Check if a given command-line tool is installed (by name).
# If not found, add it to missing_deps.
check_command() {
  if command -v "$1" &> /dev/null; then
    printf "  ${GREEN}✔${RESET} %s\\n" "$1"
  else
    printf "  ${RED}✘${RESET} %s\\n" "$1"
    missing_deps+=("$1")
  fi
}

# Check if a Debian package is installed by verifying with dpkg -s.
check_debian_package() {
  # Turn off exit-on-error while checking, to avoid an immediate script exit on dpkg returning non-zero
  set +e
  dpkg -s "$1" &> /dev/null
  local dpkg_status=$?
  set -e

  if [ $dpkg_status -eq 0 ]; then
    printf "  ${GREEN}✔${RESET} %s (Debian package)\\n" "$1"
  else
    printf "  ${RED}✘${RESET} %s (Debian package)\\n" "$1"
    missing_deps+=("$1")
  fi
}

# Check if a Python package is installed by trying to import it.
check_iscc_python_package() {
  set +e
  # Try detecting iscc_sdk via pipx
  pipx list | grep idk 2> /dev/null
  local status=$?
  set -e

  if [ $status -eq 0 ]; then
    printf "  ${GREEN}✔${RESET} Python iscc_sdk package '%s'\\n" "$1"
  else
    printf "  ${RED}✘${RESET} Python iscc_sdk package '%s'\\n" "$1"
    missing_python_packages+=("$1")
  fi
}

#############################################
# Print instructions for missing items      #
#############################################

# Print installation instructions for system commands/libraries
print_install_instructions() {
  case "$1" in
    python3)
      echo "  - Install Python 3.11 with:"
      echo "    sudo apt update && sudo apt install -y python3.11 python3.11-venv python3.11-dev pip pipx"
      ;;
    cargo)
      echo "  - Install Rust and Cargo with:"
      echo "    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
      echo "    source \$HOME/.cargo/env"
      ;;
    git)
      echo "  - Install Git with:"
      echo "    sudo apt update && sudo apt install -y git"
      ;;
    forge)
      echo "  - Install Foundry with:"
      echo "    curl -L https://foundry.paradigm.xyz | bash"
      echo "    source \$HOME/.bashrc"
      echo "    foundryup"
      ;;
    npm)
      echo "  - Install Node.js (includes npm) with:"
      echo "    sudo apt update && sudo apt install -y nodejs npm"
      ;;
    curl)
      echo "  - Install curl with:"
      echo "    sudo apt update && sudo apt install -y curl"
      ;;
    pkg-config)
      echo "  - Install pkg-config with:"
      echo "    sudo apt update && sudo apt install -y pkg-config"
      ;;
    openssl)
      echo "  - Install OpenSSL development headers with:"
      echo "    sudo apt update && sudo apt install -y libssl-dev"
      ;;
    npx)
      echo "  - Install Node.js (includes npx) with:"
      echo "    sudo apt update && sudo apt install -y nodejs npm"
      ;;
    zip)
      echo "  - Install zip with:"
      echo "    sudo apt update && sudo apt install -y zip"
      ;;
    libmagic-dev)
      echo "  - Install libmagic-dev with:"
      echo "    sudo apt update && sudo apt install -y libmagic-dev"
      ;;
    libtag1-dev)
      echo "  - Install libtag1-dev with:"
      echo "    sudo apt update && sudo apt install -y libtag1-dev"
      ;;
    *)
      echo "  - Install $1 using your package manager."
      ;;
  esac
}

# Print installation instructions for missing Python packages
print_python_package_instructions() {
  for pkg in "${missing_python_packages[@]}"; do
    echo "  - Install Python package '$pkg' with:"
    echo "    pipx install $pkg"
    echo "    pipx ensurepath"
    echo "    source \$HOME/.bashrc"
  done
}

#############################################
# 1) Check all needed dependencies          #
#############################################

echo "Checking system dependencies..."
echo "System Dependencies:"

# List of required commands
required_commands=(
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

# Check commands
for cmd in "${required_commands[@]}"; do
  check_command "$cmd"
done

# Check Debian packages
check_debian_package "libmagic-dev"
check_debian_package "libtag1-dev"

echo ""
echo "Checking Python packages..."

# Check if pipx is installed before checking Python packages
check_command "pipx"

if [ "${#missing_deps[@]}" -eq 0 ]; then
  check_iscc_python_package
else
  echo ""
  echo "pipx is missing. Please install pipx first and re-run the script:"
  print_install_instructions "pipx"
  exit 1
fi

# If any system dependencies are missing, print instructions and exit
if [ "${#missing_deps[@]}" -ne 0 ]; then
  echo ""
  echo "Some system dependencies are missing. Please install them and re-run the script:"
  for dep in "${missing_deps[@]}"; do
    print_install_instructions "$dep"
  done
  echo ""
  echo "Exiting now..."
  exit 1
fi

# If any Python packages are missing, print instructions and exit
if [ "${#missing_python_packages[@]}" -ne 0 ]; then
  echo ""
  echo "Some Python packages are missing. Please install them and re-run the script:"
  print_python_package_instructions
  echo ""
  echo "Exiting now..."
  exit 1
fi

echo "All dependencies are installed."
echo ""

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
# 3) Decide: ZIP for Lambda or cargo run
#####################################

if [ "$1" == "--lambda" ]; then
  # Build the Rust binary
  echo "Building the owen_cli binary for Lambda packaging..."
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
  echo "Packaging the owen_cli binary into $OUTPUT_ZIP ..."
  mkdir -p "$OWEN_CLI_DIR"
  cp "$OWEN_CLI_BINARY" "$OWEN_CLI_DIR/owen_cli"
  zip -r "$OUTPUT_ZIP" "$OWEN_CLI_DIR/owen_cli" > /dev/null
  rm -rf "$OWEN_CLI_DIR"

  echo "Packaging completed: $OUTPUT_ZIP"
  echo "You may upload this file as a Lambda layer to your AWS account."
else
  # If the first argument is anything else (e.g., 'foldername'), run cargo with that argument
  if [ -n "$1" ]; then
    echo "Running 'cargo run --bin main $1' ..."
    cargo run --bin main "$1"
  else
    echo "No argument supplied. Usage:"
    echo "  $0 --lambda          # Build and package the binary for AWS Lambda"
    echo "  $0 <foldername>      # Run 'cargo run <foldername>'"
  fi
fi
