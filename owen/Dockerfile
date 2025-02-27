# Use Debian 12 as the base
FROM debian:12-slim

# Prevent interactive prompts
ENV DEBIAN_FRONTEND=noninteractive

# 1) Install system packages
RUN apt-get update && apt-get install -y --no-install-recommends \
    git \
    python3.11 python3.11-venv python3.11-dev python3-distutils python3-pip \
    pipx \
    curl \
    pkg-config \
    libssl-dev \
    zip \
    libmagic-dev \
    libtag1-dev \
    imagemagick \
    build-essential \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# 2) Ensure pipx is in PATH and install the Python package iscc_sdk
RUN pipx ensurepath && \
    pipx install iscc_sdk

# 3) Install Rust & Cargo
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
  | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# 4) Install Foundry (forge)
RUN curl -L https://foundry.paradigm.xyz | bash
ENV PATH="/root/.foundry/bin:${PATH}"
RUN foundryup

# 5) Install NVM & Node.js v22
ENV NVM_DIR=/root/.nvm
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.5/install.sh | bash \
    && . "$NVM_DIR/nvm.sh" \
    && nvm install 22 \
    && nvm alias default 22

# Ensure node and npm are on PATH
ENV PATH="${NVM_DIR}/versions/node/v22.0.0/bin:${PATH}"

# 6) Create a working directory and copy your project
WORKDIR /owen
COPY . /owen

# (Optional) Build your Rust project (owen_cli binary) in release mode
RUN cargo build --release --bin main

# (Optional) If you also want to compile with Hardhat during build:
# WORKDIR /owen/../contracts
# RUN npm install --save-dev hardhat && npx hardhat compile
# WORKDIR /owen

# Default entrypoint/command
CMD ["/bin/bash"]

