import { ethers } from "hardhat";
import { OwnToken } from "../../../../typechain-types";
import { DeploymentOutput } from "../types";
import { Signer } from "ethers";

export async function deployOwnToken(
  deployer: Signer
): Promise<DeploymentOutput<OwnToken>> {
  const OwnToken = await ethers.getContractFactory("OwnToken");
  const ownToken = await OwnToken.connect(deployer).deploy();
  await ownToken.waitForDeployment();

  return {
    contract: ownToken,
    contractVerificationInput: {
      deployedContractAddress: await ownToken.getAddress(),
    },
  };
}
