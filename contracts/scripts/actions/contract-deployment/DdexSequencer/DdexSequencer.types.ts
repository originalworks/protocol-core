import { Signer } from "ethers";

export interface DdexSequencerDeploymentInput {
  deployer: Signer;
  dataProvidersWhitelist: string;
  validatorsWhitelist: string;
  stakeVaultAddress: string;
  headProcessingTimeInBlocks: number;
}
