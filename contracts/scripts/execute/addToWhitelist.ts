import { KMSClient } from "@aws-sdk/client-kms";
import { KMSSigner } from "@rumblefishdev/eth-signer-kms";
import { ethers } from "hardhat";

const WHITELIST_ADDRESS = "";
const ADDRESS_TO_ADD = "";

async function main() {
  // const kmsKeyId = process.env.KMS_KEY_ID_DEV!;
  // const kmsKeyId = process.env.KMS_KEY_ID_PROD!;
  // const kmsClient = new KMSClient();
  // const kmsSigner = await KMSSigner.create(
  //   ethers.provider as any,
  //   kmsKeyId,
  //   kmsClient,
  // );

  // console.log(kmsSigner.address);

  const whitelist = await ethers.getContractAt("Whitelist", WHITELIST_ADDRESS);
  console.log(await whitelist.isWhitelisted(ADDRESS_TO_ADD));

  // console.log(
  //   await whitelist.hasRole(
  //     await whitelist.MODERATOR_ROLE(),
  //     kmsSigner.address,
  //   ),
  // );

  // const tx = await whitelist.connect(kmsSigner).addToWhitelist(ADDRESS_TO_ADD);
  // await tx.wait();
}

main();
