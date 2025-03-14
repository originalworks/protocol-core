import { ethers, upgrades } from "hardhat";

const ddexSequencerAddress = "";

async function main() {
  const DdexSequencerNewImplementation = await ethers.getContractFactory(
    "DdexSequencer"
  );

  await upgrades.upgradeProxy(
    ddexSequencerAddress,
    DdexSequencerNewImplementation
  );
}

main();
