REGION=us-east-1
STACK_NAME=owen-infra
ARTIFACTS_BUCKET=owen-artifacts
CAPABILITIES=CAPABILITY_IAM CAPABILITY_AUTO_EXPAND CAPABILITY_NAMED_IAM
TEMPLATE_FILE=template.yml

deploy-infra: PARAMETERS_FILE=template-config.json
deploy-infra: PARAMETERS=$(shell cat $(PARAMETERS_FILE) | jqn 'get("Parameters") | entries | map(x => x[0] + "=" + x[1]) | join(" ")')
deploy-infra: 
	./build-aws-queue-consumer.sh
	sam deploy --template-file $(TEMPLATE_FILE) --stack-name $(STACK_NAME) --capabilities $(CAPABILITIES) --region $(REGION) --s3-bucket $(ARTIFACTS_BUCKET) --parameter-overrides $(PARAMETERS) --no-fail-on-empty-changeset