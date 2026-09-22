import { ethers } from "hardhat";

const VERIFIER_ROUTER_ADDRESS = '' ;
const VERIFIER_ADDRESS = '';

async function main() {
  const [deployer] = await ethers.getSigners();
  const verifierRouterAddress = VERIFIER_ROUTER_ADDRESS

  const router = await ethers.getContractAt("RiscZeroVerifierRouter", verifierRouterAddress, deployer);
  const verifierAddress = VERIFIER_ADDRESS;

  const verifier = await ethers.getContractAt(
    "RiscZeroGroth16Verifier",
    verifierAddress,
    deployer
  );
  const selector = await verifier.SELECTOR();
  await (
    await router.addVerifier(
      selector,
      verifierAddress, 
    )
  ).wait();

  console.log(
    `Registered verifier ${verifierAddress} for selector ${selector}`
  );
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
