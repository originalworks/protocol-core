import { ethers } from "hardhat";
import { FakeVerifier } from "../../../../typechain-types";
import { Signer } from "ethers";

export async function deployFakeVerifier(
  deployer: Signer
): Promise<FakeVerifier> {
  const FakeVerifier = await ethers.getContractFactory("FakeVerifier");
  const fakeVerifier = await FakeVerifier.connect(deployer).deploy();
  await fakeVerifier.waitForDeployment();

  return fakeVerifier;
}
