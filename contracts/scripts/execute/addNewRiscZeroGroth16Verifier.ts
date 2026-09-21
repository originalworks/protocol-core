import hre, { ethers } from "hardhat";
import fs from "node:fs";
import path from "node:path";
import { deployRiscZeroGroth16Verifier } from "../actions/contract-deployment/RiscZeroGroth16Verifier/RiscZeroGroth16Verifier.deploy";
import { verifyContracts } from "../actions/verify/verifyContract";
import { required } from "../utils/required";

const DDEX_EMITTER_ADDRESS = process.env.DDEX_EMITTER_ADDRESS;
const VERIFIER_ROUTER_ADDRESS = process.env.RISC_ZERO_VERIFIER_ROUTER;

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
  const emitterAddress = required(DDEX_EMITTER_ADDRESS, "DDEX_EMITTER_ADDRESS");
  const routerAddress = required(
    VERIFIER_ROUTER_ADDRESS,
    "RISC_ZERO_VERIFIER_ROUTER"
  );
  const emitter = await ethers.getContractAt("DdexEmitter", emitterAddress, deployer);

  const [currentBlobImageId] = await emitter.getSupportedBlobImageIds();
  const [currentVerifierImageId] = await emitter.getSupportedVerifierImageIds();
  const imageIdChanged =
    currentBlobImageId.toLowerCase() !== newImageId.toLowerCase() ||
    currentVerifierImageId.toLowerCase() !== newImageId.toLowerCase();

  console.log(`Deployer: ${await deployer.getAddress()}`);
  console.log(`Emitter: ${emitterAddress}`);
  console.log(`Current blob image ID: ${currentBlobImageId}`);
  console.log(`New image ID: ${newImageId}`);
  console.log(`Verifier router: ${routerAddress}`);

  const deployment = await deployRiscZeroGroth16Verifier(deployer);
  const newVerifierAddress = await deployment.contract.getAddress();
  console.log(`New verifier: ${newVerifierAddress}`);

  await verifyContracts(hre, [deployment.contractVerificationInput]);

  const router = await ethers.getContractAt(
    "RiscZeroVerifierRouter",
    routerAddress,
    deployer
  );
  const selector = await deployment.contract.SELECTOR();
  const registeredVerifier = await router.verifiers(selector);
  if (registeredVerifier === ethers.ZeroAddress) {
    await (await router.addVerifier(selector, newVerifierAddress)).wait();
    console.log(`Registered verifier selector ${selector} in router`);
  } else if (registeredVerifier.toLowerCase() !== newVerifierAddress.toLowerCase()) {
    throw new Error(
      `Selector ${selector} is already registered to ${registeredVerifier}`
    );
  } else {
    console.log(`Verifier selector ${selector} is already registered`);
  }

  if (!imageIdChanged) {
    console.log(
      `Image ID ${newImageId} is already current on the emitter; leaving image ID slots unchanged.`
    );
    return;
  }

  const targets = [
    await emitter.BLOB_PREVIOUS_IMAGE_ID(),
    await emitter.VERIFIER_PREVIOUS_IMAGE_ID(),
    await emitter.BLOB_CURRENT_IMAGE_ID(),
    await emitter.VERIFIER_CURRENT_IMAGE_ID(),
  ];
  const imageIds = [
    currentBlobImageId,
    currentVerifierImageId,
    newImageId,
    newImageId,
  ];
  const tx = await emitter.setImageIds(targets, imageIds);
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
