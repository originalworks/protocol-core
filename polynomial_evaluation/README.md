### Evaluation of BLS polynomial created from BLOB (EIP4844)

This is WIP. Use with caution.

## Precalculated values:

- `ROOTS_OF_UNITY_BRP_BYTES: [[u8; 32]; 4096]` Roots of unity bit reversal permutation for number 4096
- `INVERSE_WIDTH_BYTES: [u8; 32]` Reversed `Scalar` for number 4096

Both values can be used to create Scalars with `bls12_381::Scalar::from_bytes()`.

## Usecase:

This software focuses on transforming a BLOB into a polynomial and performing evaluation on it. When this evaluation is executed inside RISC Zero guest code:

1. The resulting polynomial data becomes part of the guest code's output.
2. This output can then be verified on-chain, ensuring the processed BLOB corresponds to the original commitment in the queue.

By incorporating this transformation and calculation step into the workflow, the software creates a reliable and verifiable link between the BLOB commitment and the proof.
