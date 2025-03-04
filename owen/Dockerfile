# Base stage: Python dependencies
FROM python:3.11-slim AS python-base
RUN pip install --no-cache-dir pipx && pipx install iscc_sdk

# Base stage: Rust & Cargo
FROM rust:latest AS rust-base
RUN rustup default stable  # Explicitly set Rust stable version

# Base stage: Foundry (Solidity development)
FROM debian:12 AS foundry-base
RUN apt-get update && apt-get install -y curl git
RUN curl -L https://foundry.paradigm.xyz | bash
RUN /root/.foundry/bin/foundryup
ENV PATH="/root/.foundry/bin:${PATH}"

# Base stage: Node.js & Hardhat
FROM node:22 AS node-base
RUN npm install -g hardhat

# Base stage: Clone GitHub Repository (protocol-core)
FROM debian:12 AS clone-repo
RUN apt-get update && apt-get install -y git
WORKDIR /protocol-core
RUN git clone --recursive --depth 1 https://github.com/originalworks/protocol-core . \
    && git submodule update --init --recursive  # Ensure submodules are initialized immediately

# Final image: Only what’s needed
FROM debian:12-slim AS final
WORKDIR /owen

# Install system dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    git \
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

# Copy dependencies from previous stages
COPY --from=python-base /root/.local /root/.local
COPY --from=rust-base /usr/local/cargo /usr/local/cargo
COPY --from=foundry-base /root/.foundry /root/.foundry
COPY --from=node-base /usr/local /usr/local
COPY --from=clone-repo /protocol-core /protocol-core

# Ensure paths are set correctly
ENV PATH="/root/.local/bin:/usr/local/cargo/bin:/root/.foundry/bin:/usr/local/bin:${PATH}"

# Set default Rust toolchain in the final image
RUN rustup default stable

# Run Hardhat compile inside /protocol-core/contracts
WORKDIR /protocol-core/contracts
RUN npm install && npx hardhat compile

# Install w3cli
RUN npm install @web3-storage/w3cli

# Ensure the build runs inside `/protocol-core/owen`
WORKDIR /protocol-core/owen

# Install s3fs
RUN apt-get update && apt-get install -y --no-install-recommends \
    s3fs \
 && rm -rf /var/lib/apt/lists/*

# Copy .env file
COPY .env .env

# Make sure the mountpoint directory exists
RUN mkdir -p /protocol-core/owen/input

# Make the script executable
RUN chmod +x /protocol-core/owen/mount-s3.sh

# Set as entrypoint (assuming your script is in /protocol-core/owen)
ENTRYPOINT ["/protocol-core/owen/mount-s3.sh"]

# Build Rust project
RUN cargo build --release --bin main

# Default entrypoint/command
# CMD ["/bin/bash"]
