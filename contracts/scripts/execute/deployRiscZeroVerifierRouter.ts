import { deployRiscZeroVerifierRouter } from "../actions/contract-deployment/RiscZeroVerifierRouter/RiscZeroVerifierRouter.deploy";
import { verifyContracts } from "../actions/verify/verifyContract";
import hre from "hardhat";

async function main() {
  const [deployer] = await hre.ethers.getSigners();
  const routerDeployment = await deployRiscZeroVerifierRouter(deployer);
  const routerAddress = await routerDeployment.contract.getAddress();
  console.log(`Verifier router: ${routerAddress}`);

  await verifyContracts(hre, [routerDeployment.contractVerificationInput]);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
