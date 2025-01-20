import { ethers } from "hardhat";
import { DdexSequencer } from "../../../../typechain-types";
import { DdexSequencerDeploymentInput } from "./DdexSequencer.types";
import { verifyContract } from "../../verifyContract";
import hre from "hardhat";

export async function deployDdexSequencer(
  input: DdexSequencerDeploymentInput
): Promise<DdexSequencer> {
  const DdexSequencer = await ethers.getContractFactory("DdexSequencer");
  const ddexSequencer = await DdexSequencer.deploy(
    input.dataProvidersWhitelist,
    input.validatorsWhitelist,
    input.stakeVaultAddress
  );
  await ddexSequencer.waitForDeployment();

  await verifyContract(await ddexSequencer.getAddress(), hre, [
    input.dataProvidersWhitelist,
    input.validatorsWhitelist,
    input.stakeVaultAddress,
  ]);

  return ddexSequencer;
}
