# WIP - dummy file
echo "there was a rebuild attempt!" > /workspace/rebuild-attempt.txt
docker stop $(docker ps -q)
git pull
rm ./validator_node/.env
cp ./aws/validator_infra/.env ./validator_node/.env

# docker build --no-cache -t validator-node-image ./validator_node/ 