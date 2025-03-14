import { ethers, upgrades } from "hardhat";
import { DdexSequencer } from "../../../../typechain-types";
import { DdexSequencerDeploymentInput } from "./DdexSequencer.types";
import { DeploymentOutput } from "../types";
import { getImplementationAddressFromProxy } from "@openzeppelin/upgrades-core";

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

  const implementationAddress = await getImplementationAddressFromProxy(
    ethers.provider,
    await ddexSequencer.getAddress()
  );
  if (!implementationAddress) {
    throw new Error("Implementation address not found");
  }

  return {
    contract: ddexSequencer as unknown as DdexSequencer,
    contractVerificationInput: {
      deployedContractAddress: implementationAddress,
    },
  };
}
