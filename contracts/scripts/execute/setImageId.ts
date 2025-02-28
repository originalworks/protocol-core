import { ethers } from "hardhat";
// Targets:
// CURRENT_BLOB = "0x01",
// PREVIOUS_BLOB = "0x02",
// CURRENT_VERIFIER = "0x03",
// PREVIOUS_VERIFIER = "0x04"

const NEW_IMAGE_ID = "";
const DDEX_EMITTER_ADDRESS = "";
const TARGET = ""


async function main() {
  const ddexEmitter = await ethers.getContractAt(
    "DdexEmitter",
    DDEX_EMITTER_ADDRESS
  );

  const tx = await ddexEmitter.setImageId(ethers.getBytes(TARGET), NEW_IMAGE_ID);

  await tx.wait();

  console.log(
    `New ImageID: ${NEW_IMAGE_ID} on target ${TARGET} was set for DdexEmitter contract: ${DDEX_EMITTER_ADDRESS}. Transaction hash: ${tx.hash}`
  );
}
main();
