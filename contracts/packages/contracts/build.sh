#!/bin/bash
set -e

# Paths
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
PACKAGE_DIR="$ROOT_DIR/packages/contracts"

SRC_ARTIFACTS_DIR="$ROOT_DIR/artifacts"
SRC_CACHE_DIR="$ROOT_DIR/cache"

DEST_ABI_DIR="$PACKAGE_DIR/abis"

rm -rf "$SRC_ARTIFACTS_DIR" "$SRC_CACHE_DIR"

find "$PACKAGE_DIR" -mindepth 1 ! -name 'package.json' ! -name 'build.sh' -exec rm -rf {} +

npx hardhat compile

mkdir -p "$DEST_ABI_DIR"

# Find and copy JSON files (exclude dbg.json and testing folder)
find "$SRC_ARTIFACTS_DIR/contracts" -type f -name "*.json" \
  ! -name "*.dbg.json" \
  ! -path "*/testing/*" \
  -exec cp {} "$DEST_ABI_DIR" \;

# Add proxy from @oz
find "$SRC_ARTIFACTS_DIR/@openzeppelin/contracts" -type f -name "ERC1967Proxy.json" \
  -exec cp {} "$DEST_ABI_DIR" \;

cp -r $ROOT_DIR/contracts/* "$PACKAGE_DIR/"

echo "Done"

