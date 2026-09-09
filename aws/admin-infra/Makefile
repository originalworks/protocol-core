STAGE_STACK_NAME=owen-admin-infra-stage
STAGE_ARTIFACTS_BUCKET=owen-artifacts-stage
STAGE_PARAMETERS_FILE=template-config-stage.json

DEV_STACK_NAME=owen-admin-infra-dev
DEV_ARTIFACTS_BUCKET=owen-artifacts-dev
DEV_PARAMETERS_FILE=template-config-dev.json

PROD_STACK_NAME=owen-admin-infra-prod
PROD_ARTIFACTS_BUCKET=owen-artifacts-prod
PROD_PARAMETERS_FILE=template-config-prod.json

TEMPLATE_FILE=template.yml
CAPABILITIES=CAPABILITY_IAM CAPABILITY_AUTO_EXPAND CAPABILITY_NAMED_IAM
REGION=us-east-1

deploy-infra-stage: PARAMETERS=$(shell jq -r '.Parameters | to_entries | map("\(.key)=\(.value)") | join(" ")' $(STAGE_PARAMETERS_FILE))
deploy-infra-stage:
	sam deploy --template-file $(TEMPLATE_FILE) --stack-name $(STAGE_STACK_NAME) --capabilities $(CAPABILITIES) --region $(REGION) --s3-bucket $(STAGE_ARTIFACTS_BUCKET) --parameter-overrides $(PARAMETERS) --no-fail-on-empty-changeset

deploy-infra-dev: PARAMETERS=$(shell jq -r '.Parameters | to_entries | map("\(.key)=\(.value)") | join(" ")' $(DEV_PARAMETERS_FILE))
deploy-infra-dev:
	sam deploy --template-file $(TEMPLATE_FILE) --stack-name $(DEV_STACK_NAME) --capabilities $(CAPABILITIES) --region $(REGION) --s3-bucket $(DEV_ARTIFACTS_BUCKET) --parameter-overrides $(PARAMETERS) --no-fail-on-empty-changeset

deploy-infra-prod: PARAMETERS=$(shell jq -r '.Parameters | to_entries | map("\(.key)=\(.value)") | join(" ")' $(PROD_PARAMETERS_FILE))
deploy-infra-prod:
	sam deploy --template-file $(TEMPLATE_FILE) --stack-name $(PROD_STACK_NAME) --capabilities $(CAPABILITIES) --region $(REGION) --s3-bucket $(PROD_ARTIFACTS_BUCKET) --parameter-overrides $(PARAMETERS) --no-fail-on-empty-changeset

# Usage: make publish-spa-dev
# Requires built admin/dist and AWS credentials. Reads stack outputs for bucket + distribution.
publish-spa-dev:
	$(MAKE) publish-spa ENV=dev STACK=$(DEV_STACK_NAME)

publish-spa-stage:
	$(MAKE) publish-spa ENV=stage STACK=$(STAGE_STACK_NAME)

publish-spa-prod:
	$(MAKE) publish-spa ENV=prod STACK=$(PROD_STACK_NAME)

publish-spa:
	@BUCKET=$$(aws cloudformation describe-stacks --stack-name $(STACK) --region $(REGION) --query "Stacks[0].Outputs[?OutputKey=='SpaBucketName'].OutputValue" --output text); \
	DIST=$$(aws cloudformation describe-stacks --stack-name $(STACK) --region $(REGION) --query "Stacks[0].Outputs[?OutputKey=='CloudFrontDistributionId'].OutputValue" --output text); \
	API=$$(aws cloudformation describe-stacks --stack-name $(STACK) --region $(REGION) --query "Stacks[0].Outputs[?OutputKey=='ApiEndpoint'].OutputValue" --output text); \
	POOL=$$(aws cloudformation describe-stacks --stack-name $(STACK) --region $(REGION) --query "Stacks[0].Outputs[?OutputKey=='UserPoolId'].OutputValue" --output text); \
	CLIENT=$$(aws cloudformation describe-stacks --stack-name $(STACK) --region $(REGION) --query "Stacks[0].Outputs[?OutputKey=='UserPoolClientId'].OutputValue" --output text); \
	DOMAIN=$$(aws cloudformation describe-stacks --stack-name $(STACK) --region $(REGION) --query "Stacks[0].Outputs[?OutputKey=='CognitoDomain'].OutputValue" --output text); \
	CF_URL=$$(aws cloudformation describe-stacks --stack-name $(STACK) --region $(REGION) --query "Stacks[0].Outputs[?OutputKey=='CloudFrontUrl'].OutputValue" --output text); \
	cd ../../admin && \
	VITE_API_BASE_URL="$$API" \
	VITE_COGNITO_DOMAIN="$$DOMAIN" \
	VITE_COGNITO_CLIENT_ID="$$CLIENT" \
	VITE_COGNITO_USER_POOL_ID="$$POOL" \
	VITE_REDIRECT_URI="$$CF_URL/auth-callback" \
	VITE_LOGOUT_URI="$$CF_URL/" \
	npm ci && npm run build && \
	aws s3 sync dist/ "s3://$$BUCKET/" --delete --region $(REGION) && \
	aws cloudfront create-invalidation --distribution-id "$$DIST" --paths "/*" --region $(REGION)
