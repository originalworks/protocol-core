import { ethers, upgrades } from "hardhat";
import { DdexEmitter } from "../../typechain-types";

const DDEX_EMITTER_ADDRESS = '';
const VERIFIER_ROUTER_ADDRESS = '';

async function main() {
  const [deployer] = await ethers.getSigners()
  const ddexEmitterAddress = DDEX_EMITTER_ADDRESS;
  const routerAddress = VERIFIER_ROUTER_ADDRESS
  const DdexEmitterNewImplementation = await ethers.getContractFactory(
    "DdexEmitter",
    deployer
  );
  const emitter = await upgrades.upgradeProxy(
    ddexEmitterAddress,
    DdexEmitterNewImplementation
  ) as unknown as DdexEmitter;
  await emitter.waitForDeployment();

  const tx = await emitter.setVerifierRouter(routerAddress);
  await tx.wait();
  console.log(`DdexEmitter upgraded: ${ddexEmitterAddress}`);
  console.log(`Verifier router configured: ${routerAddress}`);
}

main();
