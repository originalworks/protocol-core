import { ethers, upgrades } from "hardhat";

const ddexEmitterAddress = "";

async function main() {
  const DdexEmitterNewImplementation = await ethers.getContractFactory(
    "DdexEmitter"
  );
  await upgrades.upgradeProxy(ddexEmitterAddress, DdexEmitterNewImplementation);
}

main();
