import { ethers, upgrades } from "hardhat";
import { DdexEmitter } from "../../../../typechain-types";
import { verifyContract } from "../../verifyContract";
import hre from "hardhat";
import { getImplementationAddressFromProxy } from "@openzeppelin/upgrades-core";

export async function deployDdexEmitter(
  ddexSequencerAddress: string,
  _riscZeroGroth16VerifierAddress?: string
) {
  const riscZeroGroth16VerifierAddress =
    _riscZeroGroth16VerifierAddress || process.env.RISC_ZERO_GROTH16_VERIFIER;

  if (!riscZeroGroth16VerifierAddress) {
    throw new Error(`Missing variable: riscZeroGroth16VerifierAddress`);
  }

  const DdexEmitter = await ethers.getContractFactory("DdexEmitter");

  const ddexEmitter = await upgrades.deployProxy(
    DdexEmitter,
    [riscZeroGroth16VerifierAddress, ddexSequencerAddress],
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
  await verifyContract(implementationAddress, hre);

  return ddexEmitter as unknown as DdexEmitter;
}
