STATUS_CODE=$(curl -s -o /dev/null -w "%{http_code}" "http://127.0.0.1:5001/api/v0/id")

if [ "$STATUS_CODE" -ne 405 ]; then
    echo "Local IPFS node not detected. Please run docker-compose -f docker/run-local.yml -d first"
    read -p "Press Enter to continue..."
    exit 1
fi

NODE_VERSION=$(node -v 2>/dev/null | cut -d. -f1 | sed 's/v//')

if [ "$NODE_VERSION" -lt 18 ]; then
    echo "Error: Node.js version needs to be above 18"
    read -p "Press Enter to continue..."
    exit 1
fi  

echo -e "Are you planning to make tests with real proof generation/verification?\n\
[Y] Yes - Guest Image will be recreated and added to contracts and validator. Validator will need to run real proving.\n\
[N] No  - Contracts will be deployed with fake verifier that will allow sending fake proofs made by validator (with flag RISC0_DEV_MODE=1).\n"
read -p "Enter your choice (y/n): " choice 

case "$choice" in 
  y|Y ) 
    PROVING_SETUP=true
    echo "Recreating Guest Image..."
    cd prover
    RUN_BUILD=1 cargo build
    if [ $? -ne 0 ]; then
        cd ..
        echo "Error: Guest Image recreation failed!"
        read -p "Press Enter to continue..."
        exit 1
    fi
    
    echo -e "Guest Image has been recreated.\n"
    cd ..  
    ;;
  n|N ) 
    PROVING_SETUP=false
    echo -e "Skipping Guest Image recreation. \n"
    ;;
  * ) 
    echo "Invalid input. Please enter y or n."
    read -p "Press Enter to continue..."
    exit 1
    ;;
esac

echo "=== (Re)setting up kurtosis ==="

kurtosis clean -a && kurtosis run --enclave local-eth-testnet github.com/ethpandaops/ethereum-package

RPC_URL=http://$(kurtosis port print local-eth-testnet el-1-geth-lighthouse rpc)
WS_URL=ws://$(kurtosis port print local-eth-testnet el-1-geth-lighthouse ws)
BEACON_RPC_URL=$(kurtosis port print local-eth-testnet cl-1-lighthouse-geth http)
PRIVATE_KEY=bcdf20249abf0ed6d944c0288fad489e33f66b3960d9e6229c1cd214ed3bbe31

echo -e "\nSyncing RPC..."

while true; do
    response=$(curl -s -H "Content-type: application/json" -X POST --data '{"jsonrpc":"2.0","method":"eth_syncing","params":[],"id":1}' $RPC_URL)
    
    if echo "$response" | grep -q '"result":false'; then
        echo -e "Synced!\n"
        break
    fi

    sleep 2
done

echo "=== Deploying contracts ==="

cd ./contracts > /dev/null
rm -rf .openzeppelin/unknown-3151908.json

RPC_URL="$RPC_URL" \
PROVING_SETUP="$PROVING_SETUP" \
npx hardhat run scripts/execute/fixture/deployToKurtosis.ts --network "kurtosis_testnet"

if [ $? -ne 0 ]; then
  cd ..
  echo "Error while deploying contracts"
  read -p "Press Enter to continue..."
  exit 1
fi

DDEX_SEQUENCER_ADDRESS=$(cat tmp.txt)

rm -rf tmp.txt
cd ..

echo -e "PRIVATE_KEY=bcdf20249abf0ed6d944c0288fad489e33f66b3960d9e6229c1cd214ed3bbe31\n\
DISABLE_TELEMETRY=true\n\
LOCAL_IPFS=true\n\
IPFS_KUBO_URL=http://localhost:5001\n\
DEFAULT_IPFS_INTERFACE=KUBO\n\
OUTPUT_FILES_DIR=./output_files\n\
ENVIRONMENT=local\n\
RPC_URL=$RPC_URL\n\
DDEX_SEQUENCER_ADDRESS=${DDEX_SEQUENCER_ADDRESS:2}" >| ./owen/.env.local

echo -e "PRIVATE_KEY=bcdf20249abf0ed6d944c0288fad489e33f66b3960d9e6229c1cd214ed3bbe31\n\
DISABLE_TELEMETRY=true\n\
WS_URL=$WS_URL\n\
BEACON_RPC_URL=$BEACON_RPC_URL\n\
ENVIRONMENT=local\n\
RPC_URL=$RPC_URL\n\
START_BLOCK=1\n\
SEGMENT_LIMIT_PO2=18\n\
DDEX_SEQUENCER_ADDRESS=${DDEX_SEQUENCER_ADDRESS:2}" >| ./validator_node/.env.local

echo -e "=== DONE ===\n"
echo "You can use OWEN and VALIDATOR_NODE locally. Remember to run owen & validator with flag LOCAL=1"
if [ "$PROVING_SETUP" == "false" ]; then
    echo "To utilize fake proving remember to run validator with flag RISC0_DEV_MODE=1 or use scripts from Makefile"
fi  




