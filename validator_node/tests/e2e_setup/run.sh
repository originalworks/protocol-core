SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "Launching Kurtosis local-eth-testnet..."
kurtosis --enclave local-eth-testnet run github.com/ethpandaops/ethereum-package
sleep 20

echo "Creating env files..."
rm -f .env
cp .env.template .env
echo "" >> .env
echo "WS_URL=ws://$(kurtosis port print local-eth-testnet el-1-geth-lighthouse ws | sed 's/127.0.0.1/host.docker.internal/g')" >> .env
echo "RPC_URL=http://$(kurtosis port print local-eth-testnet el-1-geth-lighthouse rpc | sed 's/127.0.0.1/host.docker.internal/g')" >> .env
echo "BEACON_RPC_URL=$(kurtosis port print local-eth-testnet cl-1-lighthouse-geth http | sed 's/127.0.0.1/host.docker.internal/g')" >> .env

echo "Launching DEV MODE Protocol Network..."
HOST_UID=$(id -u) HOST_GID=$(id -g) docker compose -f $SCRIPT_DIR/dev_mode_proving_network.yml up --build