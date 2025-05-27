SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOCKER_DIR="$SCRIPT_DIR/../../docker"
OWEN_DIR="$SCRIPT_DIR/../../../../owen"

echo "Launching Kurtosis local-eth-testnet..."
kurtosis --enclave local-eth-testnet run github.com/ethpandaops/ethereum-package
sleep 20

echo "Creating docker env file..."
rm -f $DOCKER_DIR/.env
cp $DOCKER_DIR/.env.template $DOCKER_DIR/.env
echo "" >> $DOCKER_DIR/.env
echo "WS_URL=ws://$(kurtosis port print local-eth-testnet el-1-geth-lighthouse ws | sed 's/127.0.0.1/host.docker.internal/g')" >> $DOCKER_DIR/.env
echo "RPC_URL=http://$(kurtosis port print local-eth-testnet el-1-geth-lighthouse rpc | sed 's/127.0.0.1/host.docker.internal/g')" >> $DOCKER_DIR/.env
echo "BEACON_RPC_URL=$(kurtosis port print local-eth-testnet cl-1-lighthouse-geth http | sed 's/127.0.0.1/host.docker.internal/g')" >> $DOCKER_DIR/.env


echo "Launching DEV MODE Protocol Network..."
HOST_UID=$(id -u) HOST_GID=$(id -g) docker compose -f $DOCKER_DIR/docker-compose.yml up --build