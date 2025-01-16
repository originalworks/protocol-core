import { ethers } from "hardhat";

const NEW_IMAGE_ID = "";
const DDEX_EMITTER_ADDRESS = "";
async function main() {
  const ddexEmitter = await ethers.getContractAt(
    "DdexEmitter",
    DDEX_EMITTER_ADDRESS
  );

  const tx = await ddexEmitter.setImageId(NEW_IMAGE_ID);

  await tx.wait();

  console.log(
    `New ImageID: ${NEW_IMAGE_ID} was set for DdexEmitter contract: ${DDEX_EMITTER_ADDRESS}. Transaction hash: ${tx.hash}`
  );
}
main();
