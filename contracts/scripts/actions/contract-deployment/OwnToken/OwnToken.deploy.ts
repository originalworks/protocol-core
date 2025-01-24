import { ethers } from "hardhat";
import { OwnToken } from "../../../../typechain-types";
import { DeploymentOutput } from "../types";

export async function deployOwnToken(): Promise<DeploymentOutput<OwnToken>> {
  const OwnToken = await ethers.getContractFactory("OwnToken");
  const ownToken = await OwnToken.deploy();
  await ownToken.waitForDeployment();

  return {
    contract: ownToken,
    contractVerificationInput: {
      deployedContractAddress: await ownToken.getAddress(),
    },
  };
}
