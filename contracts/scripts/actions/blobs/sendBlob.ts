import { KzgHelper } from "../kzg/kzg";
import { DdexSequencer } from "../../../typechain-types";
import { BytesLike, Signer } from "ethers";

export async function sendBlob(
  ddexSequencer: DdexSequencer,
  signer: Signer,
  ddexMessagePath: string,
  imageId: BytesLike
) {
  const kzgOutput = await KzgHelper.generate(ddexMessagePath);
  const tx = await ddexSequencer
    .connect(signer)
    .submitNewBlob(imageId, kzgOutput.commitment, kzgOutput.blobSha2, {
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

  return kzgOutput;
}
