import { deployDdexSequencer } from "../actions/contract-deployment/DdexSequencer/DdexSequencer.deploy";
import { deployOwnToken } from "../actions/contract-deployment/OwnToken/OwnToken.deploy";
import { deployStakeVault } from "../actions/contract-deployment/StakeVault/StakeVault.deploy";
import { deployWhitelist } from "../actions/contract-deployment/Whitelist/Whitelist.deploy";
import { ethers } from "hardhat";
import fs from "fs";
import { deployDdexEmitter } from "../actions/contract-deployment/DdexEmitter/DdexEmitter.deploy";

const SLASH_RATE = 1000;

async function main() {
  const [signer] = await ethers.getSigners();

  console.log("Deploying whitelists...");
  const dataProvidersWhitelist = await deployWhitelist(signer, []);
  const validatorsWhitelist = await deployWhitelist(signer, []);

  console.log("Deploying DDEX sequencer...");
  const ownToken = await deployOwnToken();
  const stakeVault = await deployStakeVault({
    stakeTokenAddress: await ownToken.getAddress(),
    _slashRate: SLASH_RATE,
  });
  const ddexSequencer = await deployDdexSequencer({
    dataProvidersWhitelist: await dataProvidersWhitelist.getAddress(),
    validatorsWhitelist: await validatorsWhitelist.getAddress(),
    stakeVaultAddress: await stakeVault.getAddress(),
  });
  await ddexSequencer.disableWhitelist();

  console.log("Deploying Verifier...");
  const ddexEmitter = await deployDdexEmitter(await ddexSequencer.getAddress());

  await ddexSequencer.setVerifier(ddexEmitter);

  const deploymentData = {
    signer: await signer.getAddress(),
    ownToken: await ownToken.getAddress(),
    stakeVault: await stakeVault.getAddress(),
    ddexSequencer: await ddexSequencer.getAddress(),
    dataProvidersWhitelist: await dataProvidersWhitelist.getAddress(),
    validatorsWhitelist: await validatorsWhitelist.getAddress(),
  };

  const timestamp = new Date().getTime();
  const chainId = await ethers.provider.getNetwork();

  fs.writeFileSync(
    `./deployments/${chainId.chainId}-${timestamp}.json`,
    JSON.stringify(deploymentData, null, 2)
  );
}

main();
