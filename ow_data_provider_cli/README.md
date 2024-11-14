# WIP - Work in progress

This is still very much pre-alfa early stage sketch-draft prototype!

Current status is reported on issues [v0.1](https://github.com/originalworks/protocol/issues/73) and [v0.2](https://github.com/originalworks/protocol/issues/74)

You can currenlty use the CLI to send `.xml` files as a BLOB using [EIP4844](https://www.eip4844.com/) transaction.

# HOW TO USE CLI WITH LOCAL TESTNET

### 1. Install and run kurtosis testnet:

1. Install `kurtosis` by following the instruction from [HERE](https://docs.kurtosis.com/install)
2. Run the testnet with `kurtosis --enclave local-eth-testnet run github.com/ethpandaops/ethereum-package`
3. Run the following command to view the RPC of the testnet network you just created.

```bash
$ kurtosis port print local-eth-testnet el-1-geth-lighthouse rpc

127.0.0.1:32769
```
Prefix it with `http://` and save it for later steps

### 2. Clone this repository and enter its directory

```bash
git clone https://github.com/originalworks/protocol-core && cd protocol-core
```

### 3. Run IPFS local node:

```bash
docker compose -f ./docker/run-local.yml up ipfs -d
```

### 4. Create .env file 1

1. Copy env file the sample and change the `RPC_URL` and optionally the `PRIVATE_KEY` values:
   ```bash
   cp ./ow_data_provider_cli/.env.sample ./ow_data_provider_cli/.env
   ```
2. Edit the new `.env` file and that change the value of the `RPC_URL` to the RPC value (prefixed with http) that you obtained previously.

### 5. Create .env file 2

1. Copy env file the sample and change the `RPC_URL` and optionally the `PRIVATE_KEY` values:

```bash
cp ./contracts/.env.sample ./contracts/.env
```

2. Edit the new `.env` file and that change the value of the `RPC_URL` to the RPC value (prefixed with http) that you obtained previously.

### 6. Compile and deploy contracts

1. Inside the `/contracts` folder run
   ```bash
   npm i
   npx hardhat compile
   npx hardhat run scripts/execute/deployLocal.ts --network kurtosis_testnet
   ```
This last command may take some time to complete. Once it's done, it will return output with the addresses of validators, data providers and smart contracts. Similar to this:

```
{
  token: '0x8F0342A7060e76dfc7F6e9dEbfAD9b9eC919952c',
  deployer: '0x8943545177806ED17B9F23F0a21ee5948eCaa776',
  validator: '0xE25583099BA105D9ec0A67f5Ae86D90e50036425',
  validator2: '0x614561D2d143621E126e87831AEF287678B442b8',
  dataProvider: '0xf93Ee4Cf8c6c40b329b0c0626F28333c132CF241',
  dataProvider2: '0x802dCbE1B1A97554B4F50DB5119E37E8e7336417',
  ddexSequencer: '0x00c042C4D5D913277CE16611a2ce6e9003554aD5',
  ownToken: '0x8F0342A7060e76dfc7F6e9dEbfAD9b9eC919952c',
  dataProvidersWhitelist: '0xb4B46bdAA835F8E4b4d8e208B6559cD267851051',
  validatorsWhitelist: '0x422A3492e218383753D8006C7Bfa97815B44373F'
}
```

Copy `ddexSequencer` address without the `0x` prefix and paste the value in the file `ow_data_provider_cli/src/constants.rs` for the `DDEX_SEQUENCER_ADDRESS`. (THIS SEEMS TO HAPPEN AUTOMATICALLY, SO NO LONGER NEEDED?).


### 7. Run the tests

```bash
$ cd ow_data_provider_cli && cargo run ./tests
```

As an output you should receive the list of processed messges from the `./tests` directory with the images files that was pined and the receipt of the transaction. Similar to this

```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/ow_data_provider_cli ./tests`
PROCESSED DDEX MESSAGE:
Source files: image: ./tests/asset_one/image-asset.png; XML: ./tests/asset_one/test.xml
Image file ./tests/asset_one/image-asset.png was pined to IPFS under CID: QmZ9zbXsBffafmAJSKtRXh6EZfChc1rgNR6JEJc92ZmWkS
CID: QmZ9zbXsBffafmAJSKtRXh6EZfChc1rgNR6JEJc92ZmWkS was included in the output file: ./output_files/1.xml
----------
PROCESSED DDEX MESSAGE:
Source files: image: ./tests/asset_two/random-image.avif; XML: ./tests/asset_two/name.xml
Image file ./tests/asset_two/random-image.avif was pined to IPFS under CID: QmXvR6x7tF6RcgPD51zmS2Y1pjNVquWynreC63iRpTTRsd
CID: QmXvR6x7tF6RcgPD51zmS2Y1pjNVquWynreC63iRpTTRsd was included in the output file: ./output_files/4.xml
----------
sending tx...
TransactionReceipt { inner: Eip4844(ReceiptWithBloom { receipt: Receipt { status: Eip658(true), cumulative_gas_used: 101145, logs: [Log { inner: Log { address: 0x00c042c4d5d913277ce16611a2ce6e9003554ad5, data: LogData { topics: [0x6a6fc970009454e3172a2ec189981caa44b6d81bfc3a7ba62cf8367df4aecf75], data: 0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000030a4e5158df6997b07c5781ed92225804d23da8ce76fefca0a0d5624cbb4e0779cad21e842c254216297b571bcd8b9534100000000000000000000000000000000 } }, block_hash: Some(0xe2b754bb58d6f27c95b9a42ddf10be75d60255a3e459bd5de9d6c2e9bc22c487), block_number: Some(462), block_timestamp: None, transaction_hash: Some(0xc73b1a70f9d446ab04433b9bcc3ba95849ea9f35e35cd3ece9cc7d8ab2ddcc33), transaction_index: Some(0), log_index: Some(0), removed: false }] }, logs_bloom: 0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000800000000000000000000000000000000000002000080000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000 }), transaction_hash: 0xc73b1a70f9d446ab04433b9bcc3ba95849ea9f35e35cd3ece9cc7d8ab2ddcc33, transaction_index: Some(0), block_hash: Some(0xe2b754bb58d6f27c95b9a42ddf10be75d60255a3e459bd5de9d6c2e9bc22c487), block_number: Some(462), gas_used: 101145, effective_gas_price: 100000000, blob_gas_used: Some(131072), blob_gas_price: Some(1), from: 0x802dcbe1b1a97554b4f50db5119e37e8e7336417, to: Some(0x00c042c4d5d913277ce16611a2ce6e9003554ad5), contract_address: None, state_root: None, authorization_list: None }

```

### 8 Run Cargo command with a location with xml files

```bash
cargo run --manifest-path ./ow_data_provider_cli/Cargo.toml ./dir-with-xml-files
```