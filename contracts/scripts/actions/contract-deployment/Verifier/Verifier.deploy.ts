import { ethers, upgrades } from "hardhat";
import { Verifier } from "../../../../typechain-types";

export async function deployVerifier(ddexSequencerAddress: string) {
  const riscZeroGroth16VerifierAddress = process.env.RISC_ZERO_GROTH16_VERIFIER;
  const imageId = process.env.RISC_ZERO_GUEST_IMAGE_ID;
  if (!riscZeroGroth16VerifierAddress || !imageId) {
    throw new Error(
      `Missing env variables: ${{
        riscZeroGroth16VerifierAddress,
        imageId,
      }}`
    );
  }

  const Verifier = await ethers.getContractFactory("Verifier");

  const verifier = await upgrades.deployProxy(
    Verifier,
    [riscZeroGroth16VerifierAddress, ddexSequencerAddress, imageId],
    { kind: "uups" }
  );
  await verifier.waitForDeployment();
  return verifier as unknown as Verifier;
}
