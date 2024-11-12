import { ethers } from "ethers";

export interface KzgOutput {
  blobFile: Buffer;
  blobFileHexString: string;
  proof: Uint8Array;
  commitment: Uint8Array;
}

export interface SendBlobInput {
  kzg: KzgOutput;
  wallet: ethers.Wallet;
  provider: ethers.Provider;
}

export interface SendBlobOutput {
  blobhash: string;
  commitment: string;
  txHash: string;
  parentBeaconBlockRoot: string;
}

export interface FindBlobInBeaconChainInput {
  parentBeaconBlockRoot: string;
  commitmentToFind: string;
}

export interface BlobSidecar {
  kzg_commitment: string;
  blob: string;
  index: string;
}
