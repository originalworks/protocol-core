import { ethers, upgrades } from "hardhat";
import { DdexEmitter } from "../../../../typechain-types";

export async function deployDdexEmitter(
  ddexSequencerAddress: string,
  _riscZeroGroth16VerifierAddress?: string
) {
  const riscZeroGroth16VerifierAddress =
    _riscZeroGroth16VerifierAddress || process.env.RISC_ZERO_GROTH16_VERIFIER;

  const imageId = process.env.RISC_ZERO_GUEST_IMAGE_ID;
  if (!riscZeroGroth16VerifierAddress || !imageId) {
    throw new Error(
      `Missing variables: ${{
        riscZeroGroth16VerifierAddress,
        imageId,
      }}`
    );
  }

  const DdexEmitter = await ethers.getContractFactory("DdexEmitter");

  const ddexEmitter = await upgrades.deployProxy(
    DdexEmitter,
    [riscZeroGroth16VerifierAddress, ddexSequencerAddress, imageId],
    { kind: "uups" }
  );
  await ddexEmitter.waitForDeployment();
  return ddexEmitter as unknown as DdexEmitter;
}
