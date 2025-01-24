import { ethers, upgrades } from "hardhat";
import { DdexEmitter } from "../../../../typechain-types";
import { getImplementationAddressFromProxy } from "@openzeppelin/upgrades-core";
import { DeploymentOutput } from "../types";
import { DdexEmitterDeploymentInput } from "./DdexEmitter.types";

export async function deployDdexEmitter(
  input: DdexEmitterDeploymentInput
): Promise<DeploymentOutput<DdexEmitter>> {
  const riscZeroGroth16VerifierAddress =
    input._riscZeroGroth16VerifierAddress ||
    process.env.RISC_ZERO_GROTH16_VERIFIER;

  if (!riscZeroGroth16VerifierAddress) {
    throw new Error(`Missing variable: riscZeroGroth16VerifierAddress`);
  }

  const DdexEmitter = await ethers.getContractFactory("DdexEmitter");

  const ddexEmitter = await upgrades.deployProxy(
    DdexEmitter,
    [riscZeroGroth16VerifierAddress, input.ddexSequencerAddress],
    {
      kind: "uups",
    }
  );
  await ddexEmitter.waitForDeployment();
  const implementationAddress = await getImplementationAddressFromProxy(
    ethers.provider,
    await ddexEmitter.getAddress()
  );
  if (!implementationAddress) {
    throw new Error("Implementation address not found");
  }

  return {
    contract: (ddexEmitter as unknown as DdexEmitter).connect(input.deployer),
    contractVerificationInput: {
      deployedContractAddress: implementationAddress,
    },
  };
}
