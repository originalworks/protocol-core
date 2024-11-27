import { ethers } from "hardhat";
import { FakeGroth16Verifier } from "../../../../typechain-types";

export async function deployFakeGroth16Verifier(): Promise<FakeGroth16Verifier> {
  const FakeGroth16Verifier = await ethers.getContractFactory(
    "FakeGroth16Verifier"
  );
  const fakeGroth16Verifier = await FakeGroth16Verifier.deploy();
  await fakeGroth16Verifier.waitForDeployment();

  return fakeGroth16Verifier;
}
