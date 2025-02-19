#!/bin/bash

# Set variables
LAYER_NAME="owen_cli"
LAYER_ZIP="../owen_cli.zip"
LAMBDA_FUNCTION="aws_cli_test"
RUNTIME="python3.9"
CONTRACTS_DIR="../../contracts"
OWEN_CLI_DIR="../owen_cli"
OWEN_CLI_BINARY="../target/release/main"
OUTPUT_ZIP="../owen_cli.zip"

echo "== Compiling contracts =="
cd "$CONTRACTS_DIR"
echo "Running 'npx hardhat compile' in $CONTRACTS_DIR..."
npx hardhat compile
if [ $? -ne 0 ]; then
  echo "Error: Hardhat compilation failed."
  exit 1
fi
echo "== Contracts compiles =="

cd ../owen > /dev/null

cargo build --release --bin main
if [ $? -ne 0 ]; then
  echo "Error: Rust binary build failed."
  exit 1
fi

cd aws > /dev/null

# Check if the binary was successfully built
if [ ! -f "$OWEN_CLI_BINARY" ]; then
  echo "Error: owen_cli binary was not found after build."
  exit 1
fi

# Prepare and package the binary
echo "== Packaging the owen_cli binary... =="
mkdir -p "$OWEN_CLI_DIR"
cp "$OWEN_CLI_BINARY" "$OWEN_CLI_DIR/owen_cli"
zip -r "$OUTPUT_ZIP" "$OWEN_CLI_DIR/owen_cli"
rm -rf "$OWEN_CLI_DIR"

# Step 1: Upload a new version of the Lambda Layer
echo "== Uploading new version of Lambda layer: $LAYER_NAME... =="
LAYER_VERSION=$(aws lambda publish-layer-version \
    --layer-name "$LAYER_NAME" \
    --compatible-runtimes "$RUNTIME" \
    --zip-file "fileb://$LAYER_ZIP" \
    --query 'Version' \
    --output text)

if [ -z "$LAYER_VERSION" ]; then
    echo "❌ Failed to upload Lambda layer!"
    exit 1
fi

echo "✅ Uploaded Layer: $LAYER_NAME (Version: $LAYER_VERSION)"

# Step 3: Extract existing layers and update with the new version
EXISTING_LAYERS=$(aws lambda get-function-configuration \
    --function-name "$LAMBDA_FUNCTION" \
    --query 'Layers[*].Arn' \
    --output json)

if [ -z "$EXISTING_LAYERS" ]; then
    echo "❌ Failed to get existing layers!"
    exit 1
fi

LAYERS_JSON=$(echo "$EXISTING_LAYERS" | jq --arg layer_name "$LAYER_NAME" --arg layer_version "$LAYER_VERSION" \
  'map(if test($layer_name) then sub(":\\d+$"; ":" + $layer_version) else . end)')

if [ -z "$LAYERS_JSON" ]; then
    echo "❌ Failed to create payload!"
    exit 1
fi


# # Step 4: Update the Lambda function with the new layer version
echo "Updating Lambda function: $LAMBDA_FUNCTION to use Layer Version: $LAYER_VERSION..."
RESULT=$(aws lambda update-function-configuration \
    --function-name "$LAMBDA_FUNCTION" \
    --layers "$LAYERS_JSON" \
    2>&1)

if [ $? -ne 0 ]; then
    # The command failed, print the output
    echo "Error: $RESULT"
fi

echo "✅ Lambda function $LAMBDA_FUNCTION successfully updated with the latest layer version!"