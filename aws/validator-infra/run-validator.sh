mkdir -p /docker-mnt
docker run \
    --runtime=nvidia \
    --gpus all \
    --privileged \
    -d \
    -it \
    -v /docker-mnt:/docker-mnt \
    -v /var/run/docker.sock:/var/run/docker.sock \
    -v /usr/bin/docker:/usr/bin/docker \
    --rm \
    validator-node-image:latest >> /var/log/validator-run.log 2>&1