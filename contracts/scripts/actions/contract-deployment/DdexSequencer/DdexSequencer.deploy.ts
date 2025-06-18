import { ethers, upgrades } from "hardhat";
import { DdexSequencer } from "../../../../typechain-types";
import { DdexSequencerDeploymentInput } from "./DdexSequencer.types";
import { DeploymentOutput } from "../types";
import { getImplementationAddressFromProxy } from "@openzeppelin/upgrades-core";

export async function deployDdexSequencer(
  input: DdexSequencerDeploymentInput
): Promise<DeploymentOutput<DdexSequencer>> {
  let DdexSequencer;

  if (input.brokenDdexSequencer) {
    console.log("Broken DdexSequencer will be deployed!");
    DdexSequencer = await ethers.getContractFactory("BrokenDdexSequencer");
  } else {
    DdexSequencer = await ethers.getContractFactory("DdexSequencer");
  }

  const ddexSequencer = (await upgrades.deployProxy(
    DdexSequencer,
    [
      input.dataProvidersWhitelist,
      input.validatorsWhitelist,
      input.stakeVaultAddress,
    ],
    {
      kind: "uups",
    }
  )) as unknown as DdexSequencer;

  await ddexSequencer.waitForDeployment();

  const implementationAddress = await getImplementationAddressFromProxy(
    ethers.provider,
    await ddexSequencer.getAddress()
  );
  if (!implementationAddress) {
    throw new Error("Implementation address not found");
  }

  const setHeadProcessingTimeInBlocksTx =
    await ddexSequencer.setHeadProcessingTimeInBlocks(
      input.headProcessingTimeInBlocks
    );
  await setHeadProcessingTimeInBlocksTx.wait();

  const setBlobLifetimeTx = await ddexSequencer.setBlobLifetime(
    input.blobLifetime
  );
  await setBlobLifetimeTx.wait();

  return {
    contract: ddexSequencer,
    contractVerificationInput: {
      deployedContractAddress: implementationAddress,
    },
  };
}
