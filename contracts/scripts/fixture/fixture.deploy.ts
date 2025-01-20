import { deployDdexSequencer } from "../actions/contract-deployment/DdexSequencer/DdexSequencer.deploy";
import { deployOwnToken } from "../actions/contract-deployment/OwnToken/OwnToken.deploy";
import { deployStakeVault } from "../actions/contract-deployment/StakeVault/StakeVault.deploy";
import { deployWhitelist } from "../actions/contract-deployment/Whitelist/Whitelist.deploy";
import { ethers as hardhatEthers } from "hardhat";
import { ethers, Signer, HDNodeWallet } from "ethers";
import { FixtureInput, FixtureOutput } from "./fixture.types";
import { deployRiscZeroGroth16Verifier } from "../actions/contract-deployment/RiscZeroGroth16Verifier/RiscZeroGroth16Verifier.deploy";
import { deployDdexEmitter } from "../actions/contract-deployment/DdexEmitter/DdexEmitter.deploy";

const SLASH_RATE = 1000;

export async function deployFixture(
  input: FixtureInput
): Promise<FixtureOutput> {
  console.log("Deploying whitelists...");
  const dataProvidersWhitelist = await deployWhitelist(
    input.deployer,
    input.dataProviders
  );
  const validatorsWhitelist = await deployWhitelist(
    input.deployer,
    input.validators
  );

  console.log("Deploying DdexSequencer...");
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

  if (input.disableWhitelist) {
    await ddexSequencer.disableWhitelist();
  }

  console.log("Deploying DdexEmitter...");
  const riscZeroGroth16Verifier = await deployRiscZeroGroth16Verifier();
  const ddexEmitter = await deployDdexEmitter(
    await ddexSequencer.getAddress(),
    await riscZeroGroth16Verifier.getAddress()
  );
  await ddexSequencer.setDdexEmitter(ddexEmitter);

  return {
    deployer: input.deployer,
    ownToken,
    stakeVault,
    ddexSequencer,
    ddexEmitter,
    dataProvidersWhitelist,
    validatorsWhitelist,
    dataProviders: input.dataProviders,
    validators: input.validators,
    fixtureAddresses: {
      deployer: await input.deployer.getAddress(),
      ownToken: await ownToken.getAddress(),
      stakeVault: await stakeVault.getAddress(),
      ddexSequencer: await ddexSequencer.getAddress(),
      ddexEmitter: await ddexEmitter.getAddress(),
      dataProvidersWhitelist: await dataProvidersWhitelist.getAddress(),
      validatorsWhitelist: await validatorsWhitelist.getAddress(),
      dataProviders: input.dataProviders,
      validators: input.validators,
    },
  };
}

// it's necessary to use ethers.Wallet instead of hardhatEthers.Wallet
// as only the first one currently supports type 3 EIP4844 transaction
export async function getEthersType3Wallets(
  fundsSource: Signer
): Promise<HDNodeWallet> {
  const wallet = ethers.Wallet.createRandom(hardhatEthers.provider);
  const tx = await fundsSource.sendTransaction({
    to: wallet,
    value: ethers.parseEther("5"),
  });
  await tx.wait();
  return wallet;
}
