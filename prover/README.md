# Prover
This folder contains risc0 guest code and its i/o interface to be used in validator software.

## Deterministic build
Guest code compiles to ELF binary and calculates its checksum (image id). According to the official risc0 docs there are many factors (os, hardware, dependencies) that may change ELF binary and checksum despite having same code inside the guest. To bypass this, customized build script is used that moves compiled ELF with its image ID to `src` folder of the `methods` package and generates dedicated `lib.rs`. This way, generated files can be kept in the repository and stay consistent between the builds on different machines.

## Build script
Build script is skipped by default. If you need to change guest code and/or interface change `RUN_BUILD` in `.env` to `1` and then run `cargo build`.
`RUN_BUILD_1 cargo build` will also work.

## WARNING
Before commiting new build be sure that you know what you're doing, it has serious consequences!
