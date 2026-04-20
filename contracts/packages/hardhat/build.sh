#!/bin/bash
set -e

# Paths
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
PACKAGE_DIR="$ROOT_DIR/packages/hardhat"

SRC_CONTRACTS_DIR="$ROOT_DIR/contracts"
SRC_ARTIFACTS_DIR="$ROOT_DIR/artifacts"
SRC_CACHE_DIR="$ROOT_DIR/cache"
SRC_HELPERS_DIR="$ROOT_DIR/helpers"
SRC_SCRIPTS_DIR="$ROOT_DIR/scripts"

DEST_ABI_DIR="$PACKAGE_DIR/abis"
DEST_DIST_DIR="$PACKAGE_DIR/dist"
DEST_SRC_DIR="$PACKAGE_DIR/src"

DEST_TYPECHAIN_DIR="$DEST_SRC_DIR/typechain"
DEST_HELPERS_DIR="$DEST_SRC_DIR/helpers"
DEST_SCRIPTS_DIR="$DEST_SRC_DIR/scripts"

rm -rf "$SRC_ARTIFACTS_DIR" "$SRC_CACHE_DIR"

# Clear package folder
find "$PACKAGE_DIR" -mindepth 1 ! -name 'package.json' ! -name 'build.sh' ! -name 'tsconfig.build.json' -exec rm -rf {} +

npx hardhat compile

mkdir -p "$DEST_ABI_DIR"
mkdir -p "$DEST_SRC_DIR"

# Find and copy JSON files (exclude dbg.json and testing folder)
find "$SRC_ARTIFACTS_DIR/contracts" -type f -name "*.json" \
  ! -name "*.dbg.json" \
  ! -path "*/testing/*" \
  -exec cp {} "$DEST_ABI_DIR" \;

# Add proxy from @oz
find "$SRC_ARTIFACTS_DIR/@openzeppelin/contracts" -type f -name "ERC1967Proxy.json" \
  -exec cp {} "$DEST_ABI_DIR" \;

# Generate typechain directly using copied abis
npx typechain --target=ethers-v6 --out-dir "$DEST_TYPECHAIN_DIR" "$DEST_ABI_DIR/*.json"

# Copy helpers and scripts with perserving structure so no changes in imports are needed
cp -r "$SRC_HELPERS_DIR" "$SRC_SCRIPTS_DIR" "$DEST_SRC_DIR"

tsc -p "$PACKAGE_DIR/tsconfig.build.json"

echo "Done"
