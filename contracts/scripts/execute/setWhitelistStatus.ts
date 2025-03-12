import { ethers } from "hardhat";

const SEQUENCER_ADDRTESs = "0xc69d6CA304AAaB8368cC0104D08011f6C70041F1";

async function main() {
  const sequencer = await ethers.getContractAt("DdexSequencer", SEQUENCER_ADDRTESs);

  const tx = await sequencer.setWhitelistingStatus(true);
  await tx.wait();
  console.log("Done")
}

main();
