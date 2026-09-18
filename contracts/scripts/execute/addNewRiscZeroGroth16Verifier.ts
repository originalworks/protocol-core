import hre, { ethers } from "hardhat";
import fs from "node:fs";
import path from "node:path";
import { deployRiscZeroGroth16Verifier } from "../actions/contract-deployment/RiscZeroGroth16Verifier/RiscZeroGroth16Verifier.deploy";
import { verifyContracts } from "../actions/verify/verifyContract";

const DDEX_EMITTER_ADDRESS = '0xDe804E8fc13883C447092d05F7968f86D1fD6847'

const imageIdFromGeneratedSource = () => {
  const imageIdPath = path.resolve(__dirname, "../../contracts/ImageID.sol");
  const source = fs.readFileSync(imageIdPath, "utf8");
  const match = source.match(
    /DDEX_GUEST_ID\s*=\s*bytes32\(\s*(0x[a-fA-F0-9]{64})\s*\)/
  );

  if (!match) {
    throw new Error(`Could not find a valid DDEX_GUEST_ID in ${imageIdPath}`);
  }

  return match[1];
}

async function main() {
  const newImageId = imageIdFromGeneratedSource();
  const [deployer] = await ethers.getSigners();
  const emitter = await ethers.getContractAt("DdexEmitter", DDEX_EMITTER_ADDRESS);

  const [previousBlobImageId] = await emitter.getSupportedBlobImageIds();
  const [previousVerifierImageId] = await emitter.getSupportedVerifierImageIds();
  
  if (previousBlobImageId !== previousVerifierImageId) {
    throw new Error(
      `Blob and verifier image IDs differ: ${previousBlobImageId} != ${previousVerifierImageId}`
    );
  }

  if (previousVerifierImageId.toLowerCase() === newImageId.toLowerCase()) {
    console.log(
      `Image ID ${newImageId} is already current on the emitter; nothing to update.`
    );
    return;
  }

  const previousVerifierAddress = await emitter.riscZeroGroth16Verifiers(previousBlobImageId);
  if (previousVerifierAddress === ethers.ZeroAddress) {
    throw new Error(`No verifier registered for previous image ID ${previousBlobImageId}`);
  }

  console.log(`Deployer: ${await deployer.getAddress()}`);
  console.log(`Emitter: ${DDEX_EMITTER_ADDRESS}`);
  console.log(`Previous image ID: ${previousBlobImageId}`);
  console.log(`New image ID: ${newImageId}`);
  console.log(`Previous verifier: ${previousVerifierAddress}`);

  const deployment = await deployRiscZeroGroth16Verifier(deployer);
  const newVerifierAddress = await deployment.contract.getAddress();
  console.log(`New verifier: ${newVerifierAddress}`);

  await verifyContracts(hre, [deployment.contractVerificationInput]);

  const targets = [
    await emitter.BLOB_PREVIOUS_IMAGE_ID(),
    await emitter.VERIFIER_PREVIOUS_IMAGE_ID(),
    await emitter.BLOB_CURRENT_IMAGE_ID(),
    await emitter.VERIFIER_CURRENT_IMAGE_ID(),
  ];
  const imageIds = [previousBlobImageId, previousBlobImageId, newImageId, newImageId];
  const verifiers = [
    previousVerifierAddress,
    previousVerifierAddress,
    newVerifierAddress,
    newVerifierAddress,
  ];

  const tx = await emitter.setImageIds(targets, imageIds, verifiers);
  await tx.wait();
  console.log(`DdexEmitter update transaction: ${tx.hash}`);

  const [currentBlob, previousBlob] = await emitter.getSupportedBlobImageIds();
  const [currentVerifier, previousVerifier] =
    await emitter.getSupportedVerifierImageIds();
  console.log({ currentBlob, previousBlob, currentVerifier, previousVerifier });
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
