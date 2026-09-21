import { ethers, upgrades } from "hardhat";
import { required } from "../utils/required";
import { DdexEmitter } from "../../typechain-types";

const DDEX_EMITTER_ADDRESS = '';
const VERIFIER_ROUTER_ADDRESS = '';

async function main() {
  const [deployer] = await ethers.getSigners()
  const ddexEmitterAddress = required(
    DDEX_EMITTER_ADDRESS,
    "DDEX_EMITTER_ADDRESS"
  );
  const routerAddress = required(
    VERIFIER_ROUTER_ADDRESS,
    "RISC_ZERO_VERIFIER_ROUTER"
  );
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
