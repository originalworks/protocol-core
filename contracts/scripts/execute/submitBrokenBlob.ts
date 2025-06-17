import { ethers } from "hardhat";
import fs from "fs";
import { KzgHelper } from "../actions/kzg/kzg";

async function main() {
  const dataProvider = new ethers.Wallet(
    process.env.DATA_PROVIDER_PK ||
      "0xab63b23eb7941c1251757e24b3d2350d2bc05c3c388d06f8fe6feafefb1e8c70",
    ethers.provider
  );

  const brokenDdexSequencer = await ethers.getContractAt(
    "BrokenDdexSequencer",
    fs.readFileSync("./tmp.txt", { encoding: "utf-8" }),
    dataProvider
  );

  const ddexEmitter = await ethers.getContractAt(
    "DdexEmitter",
    await brokenDdexSequencer.ddexEmitter()
  );
  const [imageId] = await ddexEmitter.getSupportedBlobImageIds();
  console.log("Sending broken blob...");

  const kzgOutput = await KzgHelper.generate(
    "./test/ddex-messages/new_release.xml"
  );
  const kzgOutput2 = await KzgHelper.generate(
    "./test/ddex-messages/new_release2.xml"
  );

  const tx = await brokenDdexSequencer.submitBrokenBlob(
    imageId,
    kzgOutput.commitment,
    kzgOutput.blobSha2,
    kzgOutput.blobhash,
    {
      type: 3,
      maxFeePerBlobGas: 10,
      gasLimit: 1000000,
      blobs: [
        {
          data: kzgOutput2.blobFile,
          proof: kzgOutput2.proof,
          commitment: kzgOutput2.commitment,
        },
      ],
    }
  );
  console.log("Waiting for receipt...");
  const receipt = await tx.wait();
  console.log("Broken blob submited: ", receipt?.hash);
}
main();

// async function nonono() {
//     const kzgOutput = await KzgHelper.generate("./test/ddex-messages/new_release.xml");
//     const tx = await ddexSequencer
//       .connect(signer)
//       .submitNewBlob(imageId, kzgOutput.commitment, kzgOutput.blobSha2, {
//         type: 3,
//         maxFeePerBlobGas: 10,
//         gasLimit: 1000000,
//         blobs: [
//           {
//             data: kzgOutput.blobFile,
//             proof: kzgOutput.proof,
//             commitment: kzgOutput.commitment,
//           },
//         ],
//       });

//     await tx.wait();

//     return kzgOutput;
// }
