import { ethers } from "hardhat";
import { RiscZeroVerifierRouter } from "../../../../typechain-types";
import { DeploymentOutput } from "../types";
import { Signer } from "ethers";

export async function deployRiscZeroVerifierRouter(
  admin: Signer
): Promise<DeploymentOutput<RiscZeroVerifierRouter>> {
  const Router = await ethers.getContractFactory(
    "RiscZeroVerifierRouter",
    admin
  );
  const router = await Router.connect(admin).deploy(await admin.getAddress());
  await router.waitForDeployment();

  return {
    contract: router,
    contractVerificationInput: {
      deployedContractAddress: await router.getAddress(),
      args: [await admin.getAddress()],
    },
  };
}
