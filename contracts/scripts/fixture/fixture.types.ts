import { BytesLike, Signer } from "ethers";
import {
  DdexEmitter,
  DdexSequencer,
  OwnToken,
  StakeVault,
  Whitelist,
} from "../../typechain-types";
import { DeploymentOutput } from "../actions/contract-deployment/types";

export interface FixtureInput {
  deployer: Signer;
  dataProviders: string[];
  validators: string[];
  disableWhitelist: boolean;
  printLogs?: boolean;
  fakeRisc0Groth16Verifier?: boolean;
  fakeImageId?: boolean;
  headProcessingTimeInBlocks?: number;
  brokenDdexSequencer?: boolean;
  blobLifetime?: number;
}

export interface FixtureOutput {
  deployer: Signer;
  ownToken: DeploymentOutput<OwnToken>;
  stakeVault: DeploymentOutput<StakeVault>;
  ddexSequencer: DeploymentOutput<DdexSequencer>;
  ddexEmitter: DeploymentOutput<DdexEmitter> & {
    imageId: BytesLike;
  };
  dataProvidersWhitelist: DeploymentOutput<Whitelist>;
  validatorsWhitelist: DeploymentOutput<Whitelist>;
  dataProviders: string[];
  validators: string[];
  fixtureAddresses: FixtureAddresses;
}

export interface FixtureAddresses {
  deployer: string;
  ownToken: string;
  stakeVault: string;
  ddexSequencer: string;
  ddexEmitter: string;
  dataProvidersWhitelist: string;
  validatorsWhitelist: string;
  dataProviders: string[];
  validators: string[];
}

export interface GetEthersType3WalletsInput {
  fundsSource: Signer;
  numberOfWallets: number;
  prefundValue: bigint;
}
