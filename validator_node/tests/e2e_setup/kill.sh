SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

docker compose -f $SCRIPT_DIR/dev_mode_proving_network.yml down --remove-orphans
kurtosis enclave stop local-eth-testnet
kurtosis clean