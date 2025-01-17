import { Signer, HDNodeWallet } from "ethers";
import {
  DdexEmitter,
  DdexSequencer,
  OwnToken,
  StakeVault,
  Whitelist,
} from "../../typechain-types";

export interface FixtureInput {
  deployer: Signer;
  dataProviders: string[];
  validators: string[];
  disableWhitelist: boolean;
}

export interface FixtureOutput {
  deployer: Signer;
  ownToken: OwnToken;
  stakeVault: StakeVault;
  ddexSequencer: DdexSequencer;
  ddexEmitter: DdexEmitter;
  dataProvidersWhitelist: Whitelist;
  validatorsWhitelist: Whitelist;
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
