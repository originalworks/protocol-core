# WIP - dummy file
echo "there was a rebuild attempt!" > /workspace/rebuild-attempt.txt
git pull
docker stop $(docker ps -q)
docker build --no-cache -t validator-node-image ./validator_node/ 