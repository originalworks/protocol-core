# Parser

## Contents
1. ddex_schema - core of the project, it has all structs needed to deserialize xml
2. yaserde & yaserde_derive - dependencies of `ddex_schema` that were customized to support regex validation
3. validation_generator - generator used to generate... validation out of xml. Uses ts.
4. resources - sample ddex messages along with ern xml schema (stripped from descriptions and flattened)
5. runner - binary file to play around with the parser.

## Installation
1. Clone repo
2. `cd parser`
3. To run with CPU: `cargo run --release`
4. To run with GPU: `cargo run --release -F cuda`

Note on GPU: Running on GPU requires a GPU with a lot of memory. We have tested it working on Nvidia cards with 8GB of RAM

On line 23 of `/prover/host/src/main.rs` you have the value `.segment_limit_po2(19)`, which should work for cards with 8GB of RAM. For higher end cards (24GB) you can comment to entire line to improve performance.

## Contents
1. core - shared interface between guest and host
2. host - 'main' code that is run by hardware. It creates environment for guest code to be proven and consists of logic what to do with generated receipt.
3. methods - in short, it consists guest code that is proven. Host 'records' execution of guest code. This is where xml parsing happends

## Installation

To run risc0 additional resources are required. Full installation guide can be found at https://dev.risczero.com/api/zkvm/install
