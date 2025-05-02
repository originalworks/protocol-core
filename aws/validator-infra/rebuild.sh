echo "Starting docker image rebuild..."
docker stop $(docker ps -q)
cd /workspace/protocol-core
git pull

# recreate .env file
secret_name=$(jq -r '.Parameters.ValidatorSecretsName' /workspace/protocol-core/aws/validator-infra/template-config.json)
rm -f /workspace/.env
cp /workspace/protocol-core/aws/validator-infra/validator.env /workspace/protocol-core/validator_node/.env
echo >> /workspace/protocol-core/validator_node/.env
aws secretsmanager get-secret-value \
    --secret-id $secret_name \
    --query SecretString \
    --output text | jq -r 'to_entries[] | "\(.key)=\(.value)"' >> /workspace/protocol-core/validator_node/.env

docker build --no-cache -t validator-node-image /workspace/protocol-core/validator_node/ 