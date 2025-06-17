SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOCKER_DIR="$SCRIPT_DIR/../../docker"
CONTRACTS_DIR="$SCRIPT_DIR/../../../contracts"


rm -f $CONTRACTS_DIR/.openzeppelin/unknown-3151908.json
docker compose -f $DOCKER_DIR/docker-compose.yml down --remove-orphans
kurtosis enclave stop local-eth-testnet
kurtosis clean