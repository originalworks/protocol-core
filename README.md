# Parser

## Contents

1. ddex_schema - core of the project, it has all structs needed to deserialize xml
2. yaserde & yaserde_derive - dependencies of `ddex_schema` that were customized to support regex validation
3. validation_generator - generator used to generate... validation out of xml. Uses ts.
4. resources - sample ddex messages along with ern xml schema (stripped from descriptions and flattened)
5. runner - binary file to play around with the parser.

## RUN Validation

1. Clone repo
2. `cd host`
3. To run with CPU: `cargo run --release`
4. To run with GPU: `cargo run --release -F cuda`

Prerequisites:
1. [Rust](https://www.rust-lang.org/tools/install) needs to be installed in your system.
2. OpenSSL library. Can be install with  `libssl-dev` on Debian based systems or with `openssl-dev` on Fedora/RedHat. 

## Notes on GPU

To run this project on a GPU, you'll need a graphics card with sufficient memory.

- **Minimum Requirement:** An Nvidia GPU with at least **4GB of VRAM**. However, using a GPU with this minimum memory requires modifying the code as described below.
- **Recommended Configuration:** A GPU with **8GB or more VRAM** for better performance.
- **Optimal Configuration:** High-end GPUs with **24GB or more VRAM** can achieve maximum performance with additional tweaks.

### Adjusting the `segment_limit_po2` Value

The performance and memory usage of the application depend on the value of `.segment_limit_po2` in `/host/src/main.rs`. Here are the suggested adjustments based on your GPU's VRAM:

- **4GB VRAM**
  Change the value of `.segment_limit_po2(19)` to `.segment_limit_po2(18)` in `/host/src/main.rs`.

- **8GB VRAM (default)**
  The default value `.segment_limit_po2(19)` (found on line 36 of `/host/src/main.rs`) should work without any changes.

- **24GB VRAM or higher**
  For optimal performance **comment out the entire line** containing `.segment_limit_po2(19)`.
  Alternatively, you can slightly increase the value to the highest one that your system supports, as determined through testing.

**Notes**

- More VRAM generally allows for better performance and larger `segment_limit_po2` values.
- Experiment with this value to find the optimal configuration for your GPU.

To run on GPU you might need to set these values first:

```bash
export NVCC_APPEND_FLAGS='--gpu-architecture=compute_86 --gpu-code=compute_86,sm_86 --generate-code arch=compute_86,code=sm_86'
```

## Contents

1. core - shared interface between guest and host
2. host - 'main' code that is run by hardware. It creates environment for guest code to be proven and consists of logic what to do with generated receipt.
3. methods - in short, it consists guest code that is proven. Host 'records' execution of guest code. This is where xml parsing happends

## Installation

To run risc0 additional resources are required. Full installation guide can be found at https://dev.risczero.com/api/zkvm/install
