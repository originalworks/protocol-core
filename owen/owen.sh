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

# This variable will hold either "sudo" or "" after we check_sudo()
SUDO_PREFIX=""

#############################################
# Functions to check for sudo               #
#############################################
check_sudo() {
  if command -v sudo &> /dev/null; then
    SUDO_PREFIX="sudo"
  else
    SUDO_PREFIX=""
  fi
}

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

# Check if Node.js v22 is installed and npx is available
check_node_version() {
  REQUIRED_VERSION="22"  # Ensure this is set properly
  if command -v node &> /dev/null; then
    NODE_VERSION=$(node -v | sed 's/v//')
    if [[ "$NODE_VERSION" == "$REQUIRED_VERSION"* ]]; then
      printf "  ${GREEN}✔${RESET} Node.js v$REQUIRED_VERSION.x detected (current: v$NODE_VERSION)\n"

      # Check for npx
      if command -v npx &> /dev/null; then
        printf "  ${GREEN}✔${RESET} npx is available\n"
      else
        printf "  ${RED}✘${RESET} npx is missing despite Node.js v$REQUIRED_VERSION.x installation\n"
        missing_deps+=("npx")
      fi

    else
      printf "  ${RED}✘${RESET} Node.js v$REQUIRED_VERSION.x not found (current: v$NODE_VERSION)\n"
      missing_deps+=("nodejs_v$REQUIRED_VERSION")
    fi
  else
    printf "  ${RED}✘${RESET} Node.js not installed\n"
    missing_deps+=("nodejs_v$REQUIRED_VERSION")
  fi
}


# Check if a Debian package is installed by verifying with dpkg -s.
check_debian_package() {
  # Temporarily turn off exit-on-error while checking,
  # to avoid immediate script exit on dpkg returning non-zero
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

# Check if a Python package is installed by checking pipx output.
check_iscc_python_package() {
  set +e
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
      echo "    ${SUDO_PREFIX} apt update && ${SUDO_PREFIX} apt install -y python3.11 python3.11-venv python3.11-dev pip pipx"
      ;;
    cargo)
      echo "  - Install Rust and Cargo with:"
      echo "    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
      echo "    source \$HOME/.cargo/env"
      ;;
    git)
      echo "  - Install Git with:"
      echo "    ${SUDO_PREFIX} apt update && ${SUDO_PREFIX} apt install -y git"
      ;;
    forge)
      echo "  - Install Foundry with:"
      echo "    curl -L https://foundry.paradigm.xyz | bash"
      echo "    source \$HOME/.bashrc"
      echo "    foundryup"
      ;;
    nodejs_v22)
      echo "  - Install Node.js v22.x with nvm:"
      echo "    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.5/install.sh | bash"
      echo "    export NVM_DIR=\"\$HOME/.nvm\""
      echo "    [ -s \"\$NVM_DIR/nvm.sh\" ] && \. \"\$NVM_DIR/nvm.sh\""
      echo "    nvm install 22"
      echo "    nvm use 22"
      echo "    nvm alias default 22"
      ;;
    curl)
      echo "  - Install curl with:"
      echo "    ${SUDO_PREFIX} apt update && ${SUDO_PREFIX} apt install -y curl"
      ;;
    pkg-config)
      echo "  - Install pkg-config with:"
      echo "    ${SUDO_PREFIX} apt update && ${SUDO_PREFIX} apt install -y pkg-config"
      ;;
    openssl)
      echo "  - Install OpenSSL development headers with:"
      echo "    ${SUDO_PREFIX} apt update && ${SUDO_PREFIX} apt install -y libssl-dev"
      ;;
    zip)
      echo "  - Install zip with:"
      echo "    ${SUDO_PREFIX} apt update && ${SUDO_PREFIX} apt install -y zip"
      ;;
    libmagic-dev)
      echo "  - Install libmagic-dev with:"
      echo "    ${SUDO_PREFIX} apt update && ${SUDO_PREFIX} apt install -y libmagic-dev"
      ;;
    libtag1-dev)
      echo "  - Install libtag1-dev with:"
      echo "    ${SUDO_PREFIX} apt update && ${SUDO_PREFIX} apt install -y libtag1-dev"
      ;;
    imagemagick)
      echo "  - Install ImageMagick with:"
      echo "    ${SUDO_PREFIX} apt update && ${SUDO_PREFIX} apt install -y imagemagick"
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
# Automatic Installation Helper Functions   #
#############################################

# Attempt to install a missing system dependency automatically
install_dependency() {
  dep="$1"
  local install_cmd=""

  case "$dep" in
    python3)
      install_cmd="${SUDO_PREFIX} apt update && ${SUDO_PREFIX} apt install -y python3.11 python3.11-venv python3.11-dev pip pipx"
      ;;
    git)
      install_cmd="${SUDO_PREFIX} apt update && ${SUDO_PREFIX} apt install -y git"
      ;;
    forge)
      # Foundry installation steps
      install_cmd="curl -L https://foundry.paradigm.xyz | bash && source \$HOME/.bashrc && foundryup"
      ;;
    curl)
      install_cmd="${SUDO_PREFIX} apt update && ${SUDO_PREFIX} apt install -y curl"
      ;;
    cargo)
      install_cmd="curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && source $HOME/.cargo/env"
      ;;
    pkg-config)
      install_cmd="${SUDO_PREFIX} apt update && ${SUDO_PREFIX} apt install -y pkg-config"
      ;;
    openssl)
      install_cmd="${SUDO_PREFIX} apt update && ${SUDO_PREFIX} apt install -y libssl-dev"
      ;;
    nodejs_v22)
      install_cmd="curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.5/install.sh | bash && \
      export NVM_DIR=\"\$HOME/.nvm\" && \
      [ -s \"\$NVM_DIR/nvm.sh\" ] && \. \"\$NVM_DIR/nvm.sh\" && \
      nvm install 22 && \
      nvm use 22 && \
      nvm alias default 22 && \
      source \$HOME/.nvm/nvm.sh"

      ;;
    zip)
      install_cmd="${SUDO_PREFIX} apt update && ${SUDO_PREFIX} apt install -y zip"
      ;;
    libmagic-dev)
      install_cmd="${SUDO_PREFIX} apt update && ${SUDO_PREFIX} apt install -y libmagic-dev"
      ;;
    libtag1-dev)
      install_cmd="${SUDO_PREFIX} apt update && ${SUDO_PREFIX} apt install -y libtag1-dev"
      ;;
    imagemagick)
      install_cmd="${SUDO_PREFIX} apt update && ${SUDO_PREFIX} apt install -y imagemagick"
      ;;
    *)
      echo "  * Sorry, I don't know how to automatically install '$dep'. Please install it manually."
      return
      ;;
  esac

  echo "Installing system dependency '$dep' via:"
  echo "  $install_cmd"
  eval "$install_cmd"
}

# Attempt to install a missing Python package automatically
install_python_pkg() {
  local pkg="$1"
  local run_cmd="pipx install $pkg && pipx ensurepath && source \$HOME/.bashrc"
  echo "Installing Python package '$pkg' via:"
  echo "  $run_cmd"
  eval "$run_cmd"
}

#############################################
# 1) Check for sudo & needed dependencies   #
#############################################

# First decide if we have 'sudo' or not
check_sudo

echo "Checking system dependencies..."
echo "System Dependencies:"

# List of required commands
required_commands=(
  git
  python3
  curl
  cargo
  forge
  pkg-config
  openssl
  zip
)

# Check commands
for cmd in "${required_commands[@]}"; do
  check_command "$cmd"
done

check_node_version

# Check Debian packages
check_debian_package "libmagic-dev"
check_debian_package "libtag1-dev"
check_debian_package "imagemagick"

echo ""
echo "Checking Python packages..."

# Check if pipx is installed before checking Python packages
check_command "pipx"

if [ "${#missing_deps[@]}" -eq 0 ]; then
  check_iscc_python_package "iscc_sdk"
else
  echo "pipx is missing; skipping check for iscc_sdk."
  # check_iscc_python_package "iscc_sdk"
fi

# Summarize missing items
all_missing_count=$(( ${#missing_deps[@]} + ${#missing_python_packages[@]} ))

if [ $all_missing_count -eq 0 ]; then
  echo "All dependencies are installed."
  echo ""
else
  echo ""
  echo "Some dependencies are missing."
  # Ask user if we should install them automatically
  read -r -p "Do you want me to try installing these dependencies for you? [Y/n] " user_choice
  # If the user just presses Enter, default to 'Y':
  user_choice=${user_choice:-Y}

  if [[ "$user_choice" =~ ^[Yy]$ ]]; then
    echo "Attempting to install missing dependencies..."
    for dep in "${missing_deps[@]}"; do
      install_dependency "$dep" || true
    done
    for pkg in "${missing_python_packages[@]}"; do
      install_python_pkg "$pkg" || true
    done

    # Source Cargo environment if it was installed or updated
    if [ -f "$HOME/.cargo/env" ]; then
      echo "Sourcing $HOME/.cargo/env to refresh environment..."
      # shellcheck source=/dev/null
      source "$HOME/.cargo/env"
    fi

    # Source .bashrc so changes to PATH (etc.) take effect
    if [ -f "$HOME/.bashrc" ]; then
      echo "Sourcing $HOME/.bashrc to refresh the environment..."
      # shellcheck source=/dev/null
      source "$HOME/.bashrc"
    else
      echo "No ~/.bashrc found; skipping source step."
    fi

    echo ""
    echo "Finished installing dependencies."
    echo "Please re-run this script to verify if everything installed properly."
    echo "Exiting now..."
    exit 0
  else
    echo ""
    echo "Please install the missing dependencies manually using the instructions below:"
    # Print instructions
    for dep in "${missing_deps[@]}"; do
      print_install_instructions "$dep"
    done
    if [ "${#missing_python_packages[@]}" -ne 0 ]; then
      print_python_package_instructions
    fi
    echo ""
    exit 1
  fi
fi

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
echo "Git submodules initialized and updated successfully."

# Check if Hardhat is installed locally
if [ ! -f "./node_modules/.bin/hardhat" ]; then
  echo "Hardhat is not installed locally in $CONTRACTS_DIR. Installing Hardhat..."
  npm install --save-dev hardhat
fi

# Run Hardhat compile
echo "Running 'npx hardhat compile' in $CONTRACTS_DIR..."
npx hardhat compile
echo "Hardhat compile completed."

# Return to the /owen directory
echo "Returning to the /owen directory..."
cd - > /dev/null

#####################################
# 3) Decide: ZIP for Lambda or cargo run
#####################################

# If no arguments, just display instructions
if [ -z "$1" ]; then
  echo "All dependencies are installed."
  echo "No argument supplied. Please use one of the following commands:"
  echo "  owen --lambda       Build and package the binary for AWS Lambda"
  echo "  owen foldername     Start Owen, indicating the folder of messages"
  exit 0
fi

if [ "$1" == "--lambda" ]; then
  # Build the Rust binary
  echo "Building the owen_cli binary for Lambda packaging..."
  cargo build --release --bin main

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
  echo "Running 'cargo run --bin main $1' ..."
  cargo run --bin main "$1"
fi
