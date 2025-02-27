import { ethers, upgrades } from "hardhat";
import { DdexEmitter } from "../../../../typechain-types";
import { getImplementationAddressFromProxy } from "@openzeppelin/upgrades-core";
import { DeploymentOutput } from "../types";
import { DdexEmitterDeploymentInput } from "./DdexEmitter.types";
import { BytesLike } from "ethers";

export async function deployDdexEmitter(
  input: DdexEmitterDeploymentInput
): Promise<DeploymentOutput<DdexEmitter> & {
  imageId: BytesLike
}> {
  const riscZeroGroth16VerifierAddress =
    input._riscZeroGroth16VerifierAddress ||
    process.env.RISC_ZERO_GROTH16_VERIFIER;

  if (!riscZeroGroth16VerifierAddress) {
    throw new Error(`Missing variable: riscZeroGroth16VerifierAddress`);
  }

  const DdexEmitter = await ethers.getContractFactory("DdexEmitter", input.deployer);

  const ddexEmitter = await upgrades.deployProxy(
    DdexEmitter,
    [riscZeroGroth16VerifierAddress, input.ddexSequencerAddress],
    {
      kind: "uups",
    }
  );

  const implementationAddress = await getImplementationAddressFromProxy(
    ethers.provider,
    await ddexEmitter.getAddress()
  );
  if (!implementationAddress) {
    throw new Error("Implementation address not found");
  }

  let contract = (ddexEmitter as unknown as DdexEmitter).connect(input.deployer);
  
  let [imageIdHexString] = await contract.getSupportedBlobImageIds()
  let imageId = ethers.getBytes(imageIdHexString)
  
  if (input.fakeImageId) {
    imageId = ethers.randomBytes(32);
    let blob_current_image_id_target = await contract.BLOB_CURRENT_IMAGE_ID()
    let verifier_current_image_id_target = await contract.VERIFIER_CURRENT_IMAGE_ID()

    await(await contract.setImageId(blob_current_image_id_target, imageId)).wait();
    await(await contract.setImageId(verifier_current_image_id_target, imageId)).wait();
  }

  return {
    contract,
    imageId,
    contractVerificationInput: {
      deployedContractAddress: implementationAddress,
    },
  };
}
