import { Signer, HDNodeWallet } from "ethers";
import {
  DdexSequencer,
  OwnToken,
  StakeVault,
  Whitelist,
} from "../../typechain-types";

export interface FixtureOutput {
  deployer: Signer;
  ownToken: OwnToken;
  stakeVault: StakeVault;
  ddexSequencer: DdexSequencer;
  dataProvidersWhitelist: Whitelist;
  validatorsWhitelist: Whitelist;
  dataProviders: Signer[];
  validators: Signer[];
}
