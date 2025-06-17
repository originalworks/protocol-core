SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOCKER_DIR="$SCRIPT_DIR/../../docker"

echo "Launching Kurtosis local-eth-testnet..."
kurtosis --enclave local-eth-testnet run github.com/ethpandaops/ethereum-package
sleep 20

echo "Creating docker env file..."
rm -f $DOCKER_DIR/.env
cp $DOCKER_DIR/.env.template $DOCKER_DIR/.env
echo "" >> $DOCKER_DIR/.env
echo "RISC0_DEV_MODE=$RISC0_DEV_MODE" >> $DOCKER_DIR/.env
echo "PROVING_SETUP=$PROVING_SETUP" >> $DOCKER_DIR/.env
if [ -n "$BLOB_LIFETIME" ]; then
    echo "BLOB_LIFETIME=$BLOB_LIFETIME" >> $DOCKER_DIR/.env
fi
if [ -n "$DEPLOY_BROKEN_DDEX_SEQUENCER" ]; then
    echo "DEPLOY_BROKEN_DDEX_SEQUENCER=$DEPLOY_BROKEN_DDEX_SEQUENCER" >> $DOCKER_DIR/.env
fi
echo "WS_URL=ws://$(kurtosis port print local-eth-testnet el-1-geth-lighthouse ws | sed 's/127.0.0.1/host.docker.internal/g')" >> $DOCKER_DIR/.env
echo "RPC_URL=http://$(kurtosis port print local-eth-testnet el-1-geth-lighthouse rpc | sed 's/127.0.0.1/host.docker.internal/g')" >> $DOCKER_DIR/.env
echo "BEACON_RPC_URL=$(kurtosis port print local-eth-testnet cl-1-lighthouse-geth http | sed 's/127.0.0.1/host.docker.internal/g')" >> $DOCKER_DIR/.env


echo "Launching DEV MODE Protocol Network..."
docker compose -f $DOCKER_DIR/docker-compose.yml up --build