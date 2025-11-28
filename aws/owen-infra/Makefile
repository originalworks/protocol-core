STAGE_STACK_NAME=owen-infra-stage
STAGE_ARTIFACTS_BUCKET=owen-artifacts-stage
STAGE_PARAMETERS_FILE=template-config-stage.json

DEV_STACK_NAME=owen-infra-dev
DEV_ARTIFACTS_BUCKET=owen-artifacts-dev
DEV_PARAMETERS_FILE=template-config-dev.json

PROD_STACK_NAME=owen-infra-prod
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
