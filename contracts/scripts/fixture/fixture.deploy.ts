import { deployDdexSequencer } from "../actions/contract-deployment/DdexSequencer/DdexSequencer.deploy";
import { deployOwnToken } from "../actions/contract-deployment/OwnToken/OwnToken.deploy";
import { deployStakeVault } from "../actions/contract-deployment/StakeVault/StakeVault.deploy";
import { deployWhitelist } from "../actions/contract-deployment/Whitelist/Whitelist.deploy";
import { ethers as hardhatEthers } from "hardhat";
import { ethers, Signer, HDNodeWallet } from "ethers";
import {
  FixtureInput,
  FixtureOutput,
  GetEthersType3WalletsInput,
} from "./fixture.types";
import { deployRiscZeroGroth16Verifier } from "../actions/contract-deployment/RiscZeroGroth16Verifier/RiscZeroGroth16Verifier.deploy";
import { deployDdexEmitter } from "../actions/contract-deployment/DdexEmitter/DdexEmitter.deploy";
import { deployFakeVerifier } from "../actions/contract-deployment/FakeVerifier/FakeVerifier.deploy";

const SLASH_RATE = 1000;
const DEFAULT_HEAD_PROCESSING_TIME_IN_BLOCKS = 34560; // 2 days in blocks (assuming 5s block time)
const DEFAULT_BLOB_LIFETIME = 30; // 2,5 min

class ConsoleLog {
  constructor(private active: boolean) {}
  public log(message?: any, ...optionalParams: any[]) {
    if (this.active) {
      console.log(message, ...optionalParams);
    }
  }
}

export async function deployFixture(
  input: FixtureInput
): Promise<FixtureOutput> {
  const _console = new ConsoleLog(input.printLogs || false);
  _console.log("Deploying whitelists...");
  const dataProvidersWhitelistOutput = await deployWhitelist(
    input.deployer,
    input.dataProviders
  );
  const validatorsWhitelistOutput = await deployWhitelist(
    input.deployer,
    input.validators
  );

  _console.log("Deploying DdexSequencer...");
  const ownTokenOutput = await deployOwnToken(input.deployer);
  const stakeVaultOutput = await deployStakeVault({
    deployer: input.deployer,
    stakeTokenAddress: await ownTokenOutput.contract.getAddress(),
    _slashRate: SLASH_RATE,
  });
  const ddexSequencerOutput = await deployDdexSequencer({
    deployer: input.deployer,
    dataProvidersWhitelist:
      await dataProvidersWhitelistOutput.contract.getAddress(),
    validatorsWhitelist: await validatorsWhitelistOutput.contract.getAddress(),
    stakeVaultAddress: await stakeVaultOutput.contract.getAddress(),
    brokenDdexSequencer: input.brokenDdexSequencer || false,
    headProcessingTimeInBlocks:
      input.headProcessingTimeInBlocks ||
      DEFAULT_HEAD_PROCESSING_TIME_IN_BLOCKS,
    blobLifetime: input.blobLifetime || DEFAULT_BLOB_LIFETIME,
  });

  if (input.disableWhitelist) {
    await ddexSequencerOutput.contract.setWhitelistingStatus(true);
  }

  _console.log("Deploying DdexEmitter...");
  let riscZeroGroth16VerifierAddress: string;
  if (input.fakeRisc0Groth16Verifier) {
    _console.log("Deploying fake verifier...");
    const fakeVerifier = await deployFakeVerifier(input.deployer);
    riscZeroGroth16VerifierAddress = await fakeVerifier.getAddress();
  } else {
    _console.log("Deploying real verifier...");
    const riscZeroGroth16VerifierOutput = await deployRiscZeroGroth16Verifier(
      input.deployer
    );
    riscZeroGroth16VerifierAddress =
      await riscZeroGroth16VerifierOutput.contract.getAddress();
  }

  const ddexEmitterOutput = await deployDdexEmitter({
    deployer: input.deployer,
    ddexSequencerAddress: await ddexSequencerOutput.contract.getAddress(),
    _riscZeroGroth16VerifierAddress: riscZeroGroth16VerifierAddress,
    fakeImageId: !!input.fakeImageId,
  });
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
  input: GetEthersType3WalletsInput
): Promise<HDNodeWallet[]> {
  let wallets: HDNodeWallet[] = [];
  for (let i = 0; i < input.numberOfWallets; i++) {
    const wallet = ethers.Wallet.createRandom(hardhatEthers.provider);
    const tx = await input.fundsSource.sendTransaction({
      to: wallet,
      value: input.prefundValue,
    });
    await tx.wait();
    wallets.push(wallet);
  }

  return wallets;
}
