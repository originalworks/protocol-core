# Validator Node

## Description

Validator Node is a component of the Protocol written in Rust that leverages the RISC Zero libraries for generating Zero-Knowledge (ZK) proofs.

The Validator Node monitors the DdexSequencer smart contract. When new information about a BLOB being enqueued in the DdexSequencer is detected, the node downloads the BLOB from the Beacon Chain, decodes it, extracts DDEX messages, and validates each message. After validation, the node extracts key data from the messages. The validation and extraction processes run inside RISC Zero guest code (ZK proving system).

The output of the RISC Zero guest code includes both a cryptographic proof and key data extracted from the messages. This output is sent back to the DdexSequencer, where the proof is validated on-chain, and the extracted data is emitted as blockchain events.

## Requirements and Setup

### Hardware Requirements

Producing RISC Zero ZK proofs requires a significant amount of computational power. You can run the system in either CPU mode or GPU mode. While CPU mode is compatible with most Linux PCs, GPU mode requires an Nvidia GPU with at least 4GB of VRAM.

**Minimum Configuration:**

- CPU: 8+ core processor
- RAM: 16GB minimum
- Network: 100Mbps stable connection
- For GPU mode: Nvidia GPU with at least 4GB VRAM (optional but recommended)

**notes on GPU mode:**

To run this project on a GPU, you'll need a graphics card with sufficient memory.

- **Minimum:** at least **4GB of VRAM**. However, using a GPU with this minimum memory requires modifying the code as described below and is very slow.
- **Recommended: 8GB or more VRAM** for better performance.
- **Optimal:** High-end GPUs with **24GB or more VRAM** can achieve maximum performance with additional tweaks.

### Software Requirements:

**For running a binary file:**

- Linux-based operating system
- For GPU mode: CUDA toolkit

## Steps to install the validator on a clean Debian system

1.  Install dependencies: `apt install curl build-essential libssl-dev git pkg-config npm`
2.  [Rust](https://www.rust-lang.org/tools/install) - Can be installed with `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` and shell refreshed with `. "$HOME/.cargo/env"`
3.  [Foundry](https://book.getfoundry.sh/getting-started/installation) - Can be installed with:

```bash
curl -L https://foundry.paradigm.xyz | bash
source $HOME/.bashrc
foundryup
```

4. Install the [RISC Zero Toolchain](https://dev.risczero.com/api/zkvm/quickstart) with:

```bash
curl -L https://risczero.com/install | bash
. "$HOME/.bashrc"
rzup install
```

5. Optional but recommended. To run with GPU you need the latest [CUDA Toolkit](https://developer.nvidia.com/cuda-downloads).
6. If running with NVIDIA GPU you will also need [Risc Zero Local Proving](https://dev.risczero.com/api/generating-proofs/local-proving)

7. Clone the repo with its submodules and enter its folder

```
git clone --recursive https://github.com/originalworks/protocol-core && cd protocol-core
```

7. Compile contracts with

```
cd contracts
npm install hardhat
npx hardhat compile
```

8. Run cargo from validator folder

```bash
cd ../validator_node
```

Run in CPU Mode

```bash
cargo run --release
```

Run in GPU Mode:

```bash
cargo run --release -F cuda
```

### `.env` File Setup:

1. Use a template to create your `.env` file:

```
cd validator_node
cp .env.template .env
```

2. Update variables in your `.env` file:

- `PRIVATE_KEY`: Your private key with funds on Holesky
- `RPC_URL`: Your RPC endpoint URL pointing to Holesky
- `OUTPUT_FILES_DIR`: Path were output files will be saved
- `WS_URL`: Your websocket endpoint pointing to Holesky
- `BEACON_RPC_URL`: Beacon chain API url for Holesky network
- `START_BLOCK`: Block from which your validator will start looking for new BLOBs to process
- `SEGMENT_LIMIT_PO2`: Please see next point
- `ENVIRONMENT`: Used for sentry logging
- `USERNAME`: Used for sentry logging
- `DDEX_SEQUENCER_ADDRESS`: Used to set ddex sequencer address for testing purposes. When unset it defaults to hardcoded protocol sequencer.

### Adjusting the `segment_limit_po2` Value

Before running make sure that your validator node is properly configured for your GPU:

- For GPU with 4GB VRAM set `18` (default).
- For GPU with 8GB VRAM set `19`.
- For GPU with 24GB VRAM or higher for optimal performance set it to `0` (turn it off). Alternatively, you can slightly increase the value to the highest one that your system supports, as determined through testing.
- For CPU Mode (no GPU), its recommened to set `0` to increase perfomance.

## Running a test enviornment locally

You can run `../setup_local.sh` to prepare local environment. After using it remember to run validator with `LOCAL=1` flag.

#### Notes

If you feel that your GPU should perform better, you can set NVCC flags to fine tune performance of your Nvidia GPU:

```bash
COMPUTE_CAP=$(nvidia-smi --query-gpu=compute_cap --format=csv,noheader | awk -F'.' '{print $1$2}')
export NVCC_APPEND_FLAGS="--gpu-architecture=compute_${COMPUTE_CAP} --gpu-code=compute_${COMPUTE_CAP},sm_${COMPUTE_CAP} --generate-code arch=compute_${COMPUTE_CAP},code=sm_${COMPUTE_CAP}"
```

To run risc0 additional resources are required. Full installation guide can be found at https://dev.risczero.com/api/zkvm/install

## Recompile contracts if needed

Run `npx hardhat compile` from the contracts folder

# Run as Docker container:

requirements:

- docker
- nvidia docker toolkit: [link](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html)
- nvidia driver with support of CUDA 12.8 (570.124.06 or higher)

## BUILD

inside `/validator_node`:

```
docker build -t validator-node-image .
```
