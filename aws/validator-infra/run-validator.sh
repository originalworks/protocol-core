mkdir -p /docker-mnt
VALIDATOR_CONTAINER_ID=$(docker run \
    --runtime=nvidia \
    --gpus all \
    --privileged \
    -d \
    -it \
    -v /docker-mnt:/docker-mnt \
    -v /var/run/docker.sock:/var/run/docker.sock \
    -v /usr/bin/docker:/usr/bin/docker \
    --rm \
    validator-node-image:latest)

docker logs -f $VALIDATOR_CONTAINER_ID  >> /var/log/validator-run.log 2>&1 &