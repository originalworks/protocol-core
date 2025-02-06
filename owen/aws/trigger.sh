#!/bin/bash

# Configuration
LAMBDA_NAME="aws_cli_test"
DATE_FILE="to_process.json"
TEMPLATE_FILE="template.json"
PAYLOAD_FILE="event.json"

# Read JSON array of dates
DATES=$(jq -r '.[]' "$DATE_FILE") 


for DATE in $DATES; do
    echo -e "\nProcessing date: $DATE"

    # Replace "HERE" with the current date in the template
    jq --arg date "$DATE" '(.Records[0].s3.object.key |= gsub("HERE"; $date))' "$TEMPLATE_FILE" >| "$PAYLOAD_FILE"

    # Invoke Lambda function and wait for the status code
    RESPONSE=$(aws lambda invoke --function-name "$LAMBDA_NAME" \
        --cli-binary-format raw-in-base64-out \
        --payload "file://$PAYLOAD_FILE" \
        --invocation-type Event --query 'StatusCode' --output text \
        /dev/null
        )

    echo "RESPONSE: $RESPONSE"
    # Check if the status code is 202 (success)
    if [ "$RESPONSE" -eq 202 ]; then
        echo "Lambda invoked successfully for date: $DATE"
    else
        echo "Error: Lambda invocation failed for date: $DATE. Status code: $RESPONSE"
        rm -rf ${PAYLOAD_FILE}
        exit 1 # Exit if the invocation failed
    fi
done

rm -rf ${PAYLOAD_FILE}
echo "That's all folks!"