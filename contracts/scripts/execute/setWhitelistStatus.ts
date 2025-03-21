import { ethers } from "hardhat";

const SEQUENCER_ADDRTESs = "0x75AbeCf07C26368F0f4AA0b0d3637A732E25467e";

async function main() {
  const sequencer = await ethers.getContractAt("DdexSequencer", SEQUENCER_ADDRTESs);

  const tx = await sequencer.setWhitelistingStatus(true);
  await tx.wait();
  console.log("Done")
}

main();
