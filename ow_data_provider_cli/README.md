# WIP!!!

This is still very much pre-alfa early stage sketch-draft prototype!!!

To send `.xml` files as a BLOB using EIP4844 transaction:

1. Create `.env` file in the project root with `RPC_URL` and `PRIVATE_KEY` values
2. Run command:
   ```
   cargo run ./dir-with-xml-files
   ```

### TODO:

- ~~pack multiple messages into BLOB~~
- ~~prepare and send transaction to DDEX MESSAGE SEQUENCER~~
- upload media files to IPFS and include CID in DDEX message before encoding it into BLOB
- either upload audio files to private ipfs node or only generate CID of audio and then include the CID in the DDEX message
- generate ISCC of audio file and include it
- manage stake
- write tests!

# HOW TO USE WITH LOCAL TESTNET

### 1. Install and run kurtosis testnet:

1. Install `kurtosis` by following the instruction from [HERE](https://docs.kurtosis.com/install)
2. Run the testnet with `kurtosis --enclave local-eth-testnet run github.com/ethpandaops/ethereum-package`
3. Run `kurtosis port print local-eth-testnet el-1-geth-lighthouse rpc` - this is the RPC of the testnet network. Prefix it with `http://` and save it for later steps

### 2. Run IPFS local node:

Run `docker compose -f run-local.yml up ipfs` inside the `/docker` directory

### 3. Compile and deploy contracts

1. Run `npm i` and `npx hardhat compile` inside `/contracts` folder
2. Use `/contracts/.env.example` file to create `.env` file. Inside the file change the value of the `RPC_URL` to the RPC value (prefixed with http) that you obtained in step 1.3
3. Run `npx hardhat run scripts/execute/deployLocal.ts --network kurtosis_testnet` inside `/contracts` folder - this may take some time to complete. Once it's done, it will return output with the addresses of validators, data providers and smart contracts. Copy `ddexSequencer` address without the `0x` prefix and paste the value in the file `ow_data_provider_cli/src/constants.rs` for the `DDEX_SEQUENCER_ADDRESS`

### 4. Run the CLI

1. Use `/ow_data_provider_cli/.env.example` file to create `.env` file. Inside the file change the value of the `RPC_URL` to the RPC value (prefixed with http) that you obtained in step 1.3
2. Inside the `/ow_data_provider_cli` directory run `cargo run ./tests` - as an output you should receive the list of processed messges from the `./tests` directory with the images files that was pined and the receipt of the transaction.
