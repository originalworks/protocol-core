import * as dotenv from "dotenv";
dotenv.config();

import "@openzeppelin/hardhat-upgrades";
import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import "@nomicfoundation/hardhat-foundry";
import "@rumblefishdev/hardhat-kms-signer";

const config: HardhatUserConfig = {
  solidity: {
    version: "0.8.24",
    settings: { evmVersion: "cancun" },
  },
  networks: {
    holesky: {
      url: `${process.env.RPC_URL}`,
      kmsKeyId: `${process.env.KMS_KEY_ID_DEV}`,
    },
    kurtosis_testnet: {
      url: `${process.env.RPC_URL}`,
      accounts: [
        "0xbcdf20249abf0ed6d944c0288fad489e33f66b3960d9e6229c1cd214ed3bbe31",
      ],
    },
  },
  etherscan: {
    apiKey: process.env.ETHERSCAN_API_KEY,
  },
};

export default config;
