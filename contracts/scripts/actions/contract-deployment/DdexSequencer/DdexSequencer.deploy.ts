import { ethers, upgrades } from "hardhat";
import { DdexSequencer } from "../../../../typechain-types";
import { DdexSequencerDeploymentInput } from "./DdexSequencer.types";
import { DeploymentOutput } from "../types";

export async function deployDdexSequencer(
  input: DdexSequencerDeploymentInput
): Promise<DeploymentOutput<DdexSequencer>> {
  const DdexSequencer = await ethers.getContractFactory("DdexSequencer");

  const ddexSequencer = await upgrades.deployProxy(
    DdexSequencer,
    [
      input.dataProvidersWhitelist,
      input.validatorsWhitelist,
      input.stakeVaultAddress,
    ],
    {
      kind: "uups",
    }
  );

  await ddexSequencer.waitForDeployment();

  return {
    contract: ddexSequencer as unknown as DdexSequencer,
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
