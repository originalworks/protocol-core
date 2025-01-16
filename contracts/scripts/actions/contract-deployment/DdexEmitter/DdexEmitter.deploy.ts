import { ethers, upgrades } from "hardhat";
import { DdexEmitter } from "../../../../typechain-types";

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
    { kind: "uups" }
  );
  await ddexEmitter.waitForDeployment();
  return ddexEmitter as unknown as DdexEmitter;
}
