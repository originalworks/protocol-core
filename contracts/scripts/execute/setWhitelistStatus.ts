import { ethers } from "hardhat";

const SEQUENCER_ADDRTESs = "";

async function main() {
  const sequencer = await ethers.getContractAt("DdexSequencer", SEQUENCER_ADDRTESs);

  const tx = await sequencer.setWhitelistingStatus(true);
  await tx.wait();
  console.log("Done")
}

main();
