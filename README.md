# ORIGINAL WORKS PROTOCOL CORE COMPONENTS

1. [OWEN](https://github.com/originalworks/protocol-core/tree/master/owen). Original Workes Electronic Notification. Software used by Royalty Administrators to publish assets to the Original Works Protocol. 
1. [Parser](https://github.com/originalworks/protocol-core/tree/master/parser)
1. [Validator_node](https://github.com/originalworks/protocol-core/tree/master/validator_node).
1. [Prover](https://github.com/originalworks/protocol-core/tree/master/prover)


## Local setup
To run local setup:
- make sure you got all dependencies installed (node >= 18, npx, forge, cargo, kurtosis)
- run `make local-up`. You will be asked whether to create real of fake proving environment.
    - if you plan to just test owen or validator features, create fake env (it will save time of additional builds and proving)
    - if you plan to run e2e test including generating and validating proof onchain then create real env
- run `owen` with `.env.local` by adding `LOCAL=1` flag
- run `validator_node` with `.env.local` by adding `LOCAL=1` flag. If you run fake proving add also `RISC0_DEV_MODE=1` for time saving.

To shut down local setup:
 - run `make local-down`