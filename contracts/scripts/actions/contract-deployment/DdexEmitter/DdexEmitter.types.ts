import { Signer } from "ethers";

export interface DdexEmitterDeploymentInput {
  deployer: Signer;
  ddexSequencerAddress: string;
  _riscZeroGroth16VerifierAddress?: string;
  fakeImageId?: boolean
}
