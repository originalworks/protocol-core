import { ethers } from "hardhat";
import { DdexSequencer } from "../../../../typechain-types";
import { DdexSequencerDeploymentInput } from "./DdexSequencer.types";
import { DeploymentOutput } from "../types";

export async function deployDdexSequencer(
  input: DdexSequencerDeploymentInput
): Promise<DeploymentOutput<DdexSequencer>> {
  const DdexSequencer = await ethers.getContractFactory("DdexSequencer");
  const ddexSequencer = await DdexSequencer.connect(input.deployer).deploy(
    input.dataProvidersWhitelist,
    input.validatorsWhitelist,
    input.stakeVaultAddress
  );
  await ddexSequencer.waitForDeployment();

  return {
    contract: ddexSequencer,
    contractVerificationInput: {
      deployedContractAddress: await ddexSequencer.getAddress(),
      args: [
        input.dataProvidersWhitelist,
        input.validatorsWhitelist,
        input.stakeVaultAddress,
      ],
    },
  };
}
