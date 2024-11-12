import { KzgHelper } from "../kzg/kzg";
import { DdexSequencer } from "../../../typechain-types";
import { Signer } from "ethers";

export async function sendBlob(
  ddexSequencer: DdexSequencer,
  signer: Signer,
  ddexMessagePath: string
) {
  const kzgOutput = await KzgHelper.generate(ddexMessagePath);
  const blobhash = KzgHelper.blobhashFromCommitment(kzgOutput.commitment);

  const tx = await ddexSequencer
    .connect(signer)
    .submitNewBlob(kzgOutput.commitment, {
      type: 3,
      maxFeePerBlobGas: 10,
      gasLimit: 1000000,
      blobs: [
        {
          data: kzgOutput.blobFile,
          proof: kzgOutput.proof,
          commitment: kzgOutput.commitment,
        },
      ],
    });

  await tx.wait();

  return { ...kzgOutput, blobhash };
}
