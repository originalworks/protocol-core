import { ethers } from "hardhat";
import { OwnToken } from "../../../../typechain-types";

export async function deployOwnToken(): Promise<OwnToken> {
  const OwnToken = await ethers.getContractFactory("OwnToken");
  const ownToken = await OwnToken.deploy();
  await ownToken.waitForDeployment();

  return ownToken;
}
