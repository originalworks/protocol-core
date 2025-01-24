import { Signer } from "ethers";

export interface StakeVaultDeploymentInput {
  deployer: Signer;
  stakeTokenAddress: string;
  _slashRate: number;
}
