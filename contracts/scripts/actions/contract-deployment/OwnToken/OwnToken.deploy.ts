import { ethers } from "hardhat";
import { OwnToken } from "../../../../typechain-types";
import { verifyContract } from "../../verifyContract";
import hre from "hardhat";

export async function deployOwnToken(): Promise<OwnToken> {
  const OwnToken = await ethers.getContractFactory("OwnToken");
  const ownToken = await OwnToken.deploy();
  await ownToken.waitForDeployment();

  await verifyContract(await ownToken.getAddress(), hre);

  return ownToken;
}
