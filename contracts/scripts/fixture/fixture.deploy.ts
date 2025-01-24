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
  const dataProvidersWhitelistOutput = await deployWhitelist(
    input.deployer,
    input.dataProviders
  );
  const validatorsWhitelistOutput = await deployWhitelist(
    input.deployer,
    input.validators
  );

  console.log("Deploying DdexSequencer...");
  const ownTokenOutput = await deployOwnToken();
  const stakeVaultOutput = await deployStakeVault({
    stakeTokenAddress: await ownTokenOutput.contract.getAddress(),
    _slashRate: SLASH_RATE,
  });
  const ddexSequencerOutput = await deployDdexSequencer({
    dataProvidersWhitelist:
      await dataProvidersWhitelistOutput.contract.getAddress(),
    validatorsWhitelist: await validatorsWhitelistOutput.contract.getAddress(),
    stakeVaultAddress: await stakeVaultOutput.contract.getAddress(),
  });

  if (input.disableWhitelist) {
    await ddexSequencerOutput.contract.disableWhitelist();
  }

  console.log("Deploying DdexEmitter...");
  const riscZeroGroth16VerifierOutput = await deployRiscZeroGroth16Verifier();
  const ddexEmitterOutput = await deployDdexEmitter(
    await ddexSequencerOutput.contract.getAddress(),
    await riscZeroGroth16VerifierOutput.contract.getAddress()
  );
  await ddexSequencerOutput.contract.setDdexEmitter(
    await ddexEmitterOutput.contract.getAddress()
  );

  return {
    deployer: input.deployer,
    ownToken: ownTokenOutput,
    stakeVault: stakeVaultOutput,
    ddexSequencer: ddexSequencerOutput,
    ddexEmitter: ddexEmitterOutput,
    dataProvidersWhitelist: validatorsWhitelistOutput,
    validatorsWhitelist: validatorsWhitelistOutput,
    dataProviders: input.dataProviders,
    validators: input.validators,
    fixtureAddresses: {
      deployer: await input.deployer.getAddress(),
      ownToken: await ownTokenOutput.contract.getAddress(),
      stakeVault: await stakeVaultOutput.contract.getAddress(),
      ddexSequencer: await ddexSequencerOutput.contract.getAddress(),
      ddexEmitter: await ddexEmitterOutput.contract.getAddress(),
      dataProvidersWhitelist:
        await dataProvidersWhitelistOutput.contract.getAddress(),
      validatorsWhitelist:
        await validatorsWhitelistOutput.contract.getAddress(),
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
