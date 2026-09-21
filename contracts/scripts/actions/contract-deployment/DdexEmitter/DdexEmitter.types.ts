import { Signer } from "ethers";

export interface DdexEmitterDeploymentInput {
  deployer: Signer;
  ddexSequencerAddress: string;
  _verifierRouterAddress?: string;
  fakeImageId?: boolean
}
