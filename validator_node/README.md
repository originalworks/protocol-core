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

**If you want to compile the binary by yourself and test it locally:**

1. [Rust](https://www.rust-lang.org/tools/install) needs to be installed in your system.
2. OpenSSL library. Can be install with `libssl-dev` on Debian based systems or with `openssl-dev` on Fedora/RedHat.
3. [RISC Zero Toolchain](https://dev.risczero.com/api/zkvm/quickstart).
4. To run with GPU you need the latest [CUDA Toolkit](https://developer.nvidia.com/cuda-downloads).
5. [Risc Zero Local Proving](https://dev.risczero.com/api/generating-proofs/local-proving)
6. Docker environment

**Network Requirements:**

- RPC url for Holesky network
- websocket url for Holesky network
- Beacon chain API url for Holesky network (available on chainstack for free)
- Private key with funds on Holesky

## How to Run It

Clone the Repository

```
git clone https://github.com/originalworks/protocol-core
cd validatore_node
```

### Adjusting the `segment_limit_po2` Value

Before running make sure that your validator node is properly configured for your GPU:

- For GPU with 4GB VRAM change the value of `.segment_limit_po2(19)` to `.segment_limit_po2(18)` in `/validator_node/src/main.rs`.
- For GPU with 8GB VRAM (default) The default value `.segment_limit_po2(19)` should work without any changes.
- For GPU with 24GB VRAM or higher For optimal performance comment out the entire line containing `.segment_limit_po2(19)`. Alternatively, you can slightly increase the value to the highest one that your system supports, as determined through testing.
- For CPU Mode (no GPU), its recommened to remove the ``.segment_limit_po2(19)` line entirely to increase perfomance.

#### Run in CPU Mode

```bash
cargo run --release
```

### Run in GPU Mode:

```bash
cargo run --release -F cuda
```

#### Notes

On some systems with GPU you might need to set these values first:

```bash
export NVCC_APPEND_FLAGS='--gpu-architecture=compute_86 --gpu-code=compute_86,sm_86 --generate-code arch=compute_86,code=sm_86'
```

To run risc0 additional resources are required. Full installation guide can be found at https://dev.risczero.com/api/zkvm/install
