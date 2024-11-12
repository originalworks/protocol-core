import { ethers } from "hardhat";
import fs from "fs";

async function main() {
  const [signer] = await ethers.getSigners();
  const OwnToken = await ethers.getContractFactory("OwnToken");
  const StakeVault = await ethers.getContractFactory("StakeVault");
  const DdexSequencer = await ethers.getContractFactory("DdexSequencer");
  const Whitelist = await ethers.getContractFactory("Whitelist");

  const dataProvidersWhitelist = await Whitelist.deploy(signer);
  await dataProvidersWhitelist.waitForDeployment();

  const validatorsWhitelist = await Whitelist.deploy(signer);
  await validatorsWhitelist.waitForDeployment();

  const ownToken = await OwnToken.deploy();
  await ownToken.waitForDeployment();

  const stakeVault = await StakeVault.deploy(ownToken, 1000);
  await stakeVault.waitForDeployment();

  const ddexSequencer = await DdexSequencer.deploy(
    dataProvidersWhitelist,
    validatorsWhitelist,
    stakeVault
  );
  await ddexSequencer.waitForDeployment();

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
