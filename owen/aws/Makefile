# AWS Lambda Configuration
LAYER_NAME := owen_cli
LAYER_ZIP := ../owen_cli.zip
LAMBDA_FUNCTION := aws_cli_test
RUNTIME := python3.9
REGION := us-east-1
ACCOUNT_ID := $(shell aws sts get-caller-identity --query "Account" --output text)

# Define a wildcard for layer ARN
LAYER_ARN := arn:aws:lambda:$(REGION):$(ACCOUNT_ID):layer/$(LAYER_NAME)

# Path to the zip script directory (one level up)
PARENT_DIR := ..

.PHONY: all zip upload-layer update-lambda clean

# Default target: Zip the layer, upload it, and update the function
all: zip upload-layer update-lambda

# Call the external shell script from its directory to ensure correct paths
zip:
	@echo "📦 Running bundle.sh from $(PARENT_DIR)..."
	@cd $(PARENT_DIR) && bash bundle.sh
	@if [ ! -f "$(LAYER_ZIP)" ]; then echo "❌ Error: ZIP file not found at $(LAYER_ZIP)!" && exit 1; fi
	@echo "✅ Zip file created: $(LAYER_ZIP)"

# Upload a new version of the Lambda Layer
upload-layer: zip
	@echo "🚀 Uploading new version of Lambda layer: $(LAYER_NAME)..."
	$(eval LAYER_VERSION := $(shell aws lambda publish-layer-version \
		--layer-name "$(LAYER_NAME)" \
		--compatible-runtimes "$(RUNTIME)" \
		--zip-file "fileb://$(LAYER_ZIP)" \
		--query 'Version' \
		--output text))
	@echo "✅ Uploaded Layer: $(LAYER_NAME) (Version: $(LAYER_VERSION))"

# Update the Lambda function with the latest layer version
update-lambda:
	@echo "🔄 Fetching current Lambda function layers..."
	@aws lambda get-function-configuration --function-name "$(LAMBDA_FUNCTION)" --query 'Layers[*].Arn' --output json >| ./layers.json
	
	@if [ -z "($LAYER_VERSION)" ]; then \
  		echo "❌ Error: LAYER_VERSION is not set!"; \
  		exit 1; \
	fi
	@export LAYER_VERSION=1
	@echo "Debug: LAYER_VERSION='$(LAYER_VERSION)'"
	$(eval LAYERS_JSON := $(shell cat ./layers.json | jq --arg layer_name "$LAYER_NAME" --arg layer_version "$LAYER_VERSION" \
  'map(if test($layer_name) then sub(":\\d+$"; ":" + $layer_version) else . end)'))
	@echo "Debug: LAYERS_JSON='$(LAYERS_JSON)'"
	# @echo "🚀 Updating Lambda function: $(LAMBDA_FUNCTION) with Layer Version: $(LAYER_VERSION)..."
	# aws lambda update-function-configuration \
	# 	--function-name "$(LAMBDA_FUNCTION)" \
	# 	--layers '$(LAYERS_JSON)'
	# @echo "✅ Lambda function $(LAMBDA_FUNCTION) successfully updated!"
	
	

# Cleanup target
clean:
	@rm -f $(LAYER_ZIP)
	@echo "🧹 Cleaned up temporary files."

	# 