import { ethers } from "hardhat";
// Targets:
// CURRENT_BLOB = "0x01",
// PREVIOUS_BLOB = "0x02",
// CURRENT_VERIFIER = "0x03",
// PREVIOUS_VERIFIER = "0x04"

const NEW_IMAGE_IDS: string[] = [];
const TARGETS: string[] = [];
const DDEX_EMITTER_ADDRESS = "";


async function main() {
  const ddexEmitter = await ethers.getContractAt(
    "DdexEmitter",
    DDEX_EMITTER_ADDRESS
  );

  const tx = await ddexEmitter.setImageIds(TARGETS.map((target) => ethers.getBytes(target)), NEW_IMAGE_IDS);

  await tx.wait();

  console.log(
    `DdexEmitter contract: ${DDEX_EMITTER_ADDRESS}. Transaction hash: ${tx.hash}`
  );
  console.log(`Changes:\n${TARGETS.map((target, index) => `${target} -> ${NEW_IMAGE_IDS[index]}`).join(",\n")}`)
}
main();
