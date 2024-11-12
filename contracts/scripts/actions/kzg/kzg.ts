import fs from "fs";
import { KzgOutput } from "../blobs/types";
import { ethers } from "hardhat";
import { loadKZG } from "kzg-wasm";

export class KzgHelper {
  public BYTES_PER_BLOB = 131072;

  static blobhashFromCommitment(commitment: Uint8Array): string {
    return `0x01${ethers.sha256(commitment).slice(4)}`;
  }

  static async generate(filePath: string): Promise<KzgOutput> {
    const kzg = await loadKZG();
    kzg.loadTrustedSetup();
    const file = fs.readFileSync(filePath);
    let fileHexString = "";

    for (let i = 0; i < file.buffer.byteLength; i++) {
      fileHexString = fileHexString + file.at(i)?.toString(16);
    }

    const blobFile = Buffer.alloc(131072, fileHexString);
    const commitment = kzg.blobToKzgCommitment(blobFile);
    const proof = kzg.computeBlobKzgProof(blobFile, commitment);

    return { proof, commitment, blobFile, blobFileHexString: fileHexString };
  }
}
