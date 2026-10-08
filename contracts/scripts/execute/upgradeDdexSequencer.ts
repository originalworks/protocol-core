import { KMSClient } from "@aws-sdk/client-kms";
import { KMSSigner } from "@rumblefishdev/eth-signer-kms";
import hre, { ethers, upgrades } from "hardhat";
import { verifyContract } from "../actions/verify/verifyContract";
// import { verifyContract } from "../actions/verifyContract";

const ddexSequencerAddress = "0x75AbeCf07C26368F0f4AA0b0d3637A732E25467e";

async function main() {
  const kmsKeyId = process.env.KMS_KEY_ID_DEV!;

  const kmsClient = new KMSClient();
  const kmsSigner = await KMSSigner.create(
    ethers.provider as any,
    kmsKeyId,
    kmsClient,
  );
  await verifyContract("0x38f53a26C273e31fAB6cf8C4066809F3De1F09e6", hre);

  // const DdexSequencerNewImplementation = await ethers.getContractFactory(
  //   "DdexSequencer",
  //   kmsSigner,
  // );

  // await upgrades.upgradeProxy(
  //   ddexSequencerAddress,
  //   DdexSequencerNewImplementation,
  // );
}

main();
