echo "there was a rebuild attempt!" > /workspace/rebuild-attempt.txt
docker stop $(docker ps -q)
git pull

# recreate .env file
rm -f ./validator_node/.env
cp ./aws/validator-infra/.env ./validator_node/.env
echo >> ./validator_node/.env
aws secretsmanager get-secret-value \
    --secret-id ValidatorSecrets \
    --query SecretString \
    --output text | jq -r 'to_entries[] | "\(.key)=\(.value)"' >> ./validator_node/.env

# docker build --no-cache -t validator-node-image ./validator_node/ 