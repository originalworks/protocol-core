import { ethers, upgrades } from "hardhat";
import { required } from "../utils/required";

const DDEX_EMITTER_ADDRESS = process.env.DDEX_EMITTER_ADDRESS;
const VERIFIER_ROUTER_ADDRESS = process.env.RISC_ZERO_VERIFIER_ROUTER;

async function main() {
  const ddexEmitterAddress = required(
    DDEX_EMITTER_ADDRESS,
    "DDEX_EMITTER_ADDRESS"
  );
  const routerAddress = required(
    VERIFIER_ROUTER_ADDRESS,
    "RISC_ZERO_VERIFIER_ROUTER"
  );
  const DdexEmitterNewImplementation = await ethers.getContractFactory(
    "DdexEmitter"
  );
  const emitter = await upgrades.upgradeProxy(
    ddexEmitterAddress,
    DdexEmitterNewImplementation
  );
  await emitter.waitForDeployment();

  const tx = await emitter.setVerifierRouter(routerAddress);
  await tx.wait();
  console.log(`DdexEmitter upgraded: ${ddexEmitterAddress}`);
  console.log(`Verifier router configured: ${routerAddress}`);
}

main();
