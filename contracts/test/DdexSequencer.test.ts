import { BytesLike, ethers, parseEther, randomBytes, Signer, Wallet } from "ethers";
import { FixtureOutput } from "../scripts/fixture/fixture.types";
import {
  deployFixture,
  getEthersType3Wallets,
} from "../scripts/fixture/fixture.deploy";
import { KzgHelper } from "../scripts/actions/kzg/kzg";
import { expect } from "chai";
import { sendBlob } from "../scripts/actions/blobs/sendBlob";
import hre from "hardhat";
import { JOURNAL_EXAMPLE } from "./journalExample";
import { KzgOutput } from "../scripts/actions/blobs/types";

const ZERO_BYTES32 =
  "0x0000000000000000000000000000000000000000000000000000000000000000";

describe("DdexSequencer", () => {
  const abiCoder = ethers.AbiCoder.defaultAbiCoder();
  let fixture: FixtureOutput;
  let dataProviders: Signer[];
  let validators: Signer[];
  let deployer: Signer;

  before(async () => {
    const [fundsSource] = await hre.ethers.getSigners();
    const [_deployer, dataProvider1, dataProvider2, validator1, validator2] =
      await getEthersType3Wallets({
        fundsSource,
        numberOfWallets: 5,
        prefundValue: parseEther("2"),
      });
    deployer = _deployer;
    validators = [validator1, validator2];
    dataProviders = [dataProvider1, dataProvider2];
  });

  beforeEach(async () => {
    fixture = await deployFixture({
      deployer,
      dataProviders: [
        await dataProviders[0].getAddress(),
        await dataProviders[1].getAddress(),
      ],
      validators: [
        await validators[0].getAddress(),
        await validators[1].getAddress(),
      ],
      disableWhitelist: false,
      fakeRisc0Groth16Verifier: true,
    });
  });

  it("Can add to empty queue", async () => {
    const dataProvider = dataProviders[0];
    const { ddexSequencer, ddexEmitter: {imageId} } = fixture;
    const kzgInput = await KzgHelper.generate(
      "./test/ddex-messages/new_release.xml"
    );

    // check that the queue is emtpy
    expect(await ddexSequencer.contract.blobQueueHead()).equal(ZERO_BYTES32);
    expect(await ddexSequencer.contract.blobQueueTail()).equal(ZERO_BYTES32);
    await expect(
      ddexSequencer.contract
        .connect(dataProvider)
        .submitNewBlob(imageId, kzgInput.commitment, kzgInput.blobSha2, {
          type: 3,
          maxFeePerBlobGas: 10,
          gasLimit: 1000000,
          blobs: [
            {
              data: kzgInput.blobFile,
              proof: kzgInput.proof,
              commitment: kzgInput.commitment,
            },
          ],
        })
    ).to.not.rejected;

    const blobsMappingResults = await ddexSequencer.contract.blobs(
      kzgInput.blobhash
    );

    expect(blobsMappingResults.nextBlob).equal(ZERO_BYTES32);
    expect(blobsMappingResults.submitted).equal(true);
    expect(blobsMappingResults.proposer).equal(dataProvider);
    expect(blobsMappingResults.blobId).equal(kzgInput.blobId);

    expect(await ddexSequencer.contract.blobQueueHead()).equal(
      kzgInput.blobhash
    );
    expect(await ddexSequencer.contract.blobQueueTail()).equal(
      kzgInput.blobhash
    );
  });

  it("Can add to non-empty queue", async () => {
    const { ddexSequencer, ddexEmitter: {imageId} } = fixture;

    const blob1Result = await sendBlob(
      ddexSequencer.contract,
      dataProviders[0],
      "./test/ddex-messages/new_release.xml",
      imageId,
    );

    const blob2Result = await sendBlob(
      ddexSequencer.contract,
      dataProviders[0],
      "./test/ddex-messages/new_release2.xml",
      imageId
    );

    expect(await ddexSequencer.contract.blobQueueHead()).equal(
      blob1Result.blobhash
    );
    expect(await ddexSequencer.contract.blobQueueTail()).equal(
      blob2Result.blobhash
    );

    const blob3Result = await sendBlob(
      ddexSequencer.contract,
      dataProviders[0],
      "./test/ddex-messages/new_release3.xml",
      imageId
    );

    expect(await ddexSequencer.contract.blobQueueHead()).equal(
      blob1Result.blobhash
    );
    expect(await ddexSequencer.contract.blobQueueTail()).equal(
      blob3Result.blobhash
    );
  });

  it("Set nextBlob for previous tail after adding new message", async () => {
    const {
      ddexSequencer,
      ddexEmitter: {imageId},
      dataProviders: [dataProvider],
    } = fixture;

    const { blobhash: blobhash1 } = await sendBlob(
      ddexSequencer.contract,
      dataProviders[0],
      "./test/ddex-messages/new_release.xml",
      imageId
    );

    expect((await ddexSequencer.contract.blobs(blobhash1)).nextBlob).equal(
      ZERO_BYTES32
    );

    expect(await ddexSequencer.contract.blobQueueTail()).equal(blobhash1);

    const { blobhash: blobhash2 } = await sendBlob(
      ddexSequencer.contract,
      dataProviders[0],
      "./test/ddex-messages/new_release2.xml",
      imageId
    );

    expect((await ddexSequencer.contract.blobs(blobhash1)).nextBlob).equal(
      blobhash2
    );
    expect(await ddexSequencer.contract.blobQueueTail()).equal(blobhash2);

    const { blobhash: blobhash3 } = await sendBlob(
      ddexSequencer.contract,
      dataProviders[0],
      "./test/ddex-messages/new_release3.xml",
      imageId
    );

    expect((await ddexSequencer.contract.blobs(blobhash2)).nextBlob).equal(
      blobhash3
    );
    expect(await ddexSequencer.contract.blobQueueTail()).equal(blobhash3);
  });

  it("Clear queue after submitting proof for the last message", async () => {
    const { ddexSequencer,ddexEmitter: {imageId} } = fixture;

    const { blobhash } = await sendBlob(
      ddexSequencer.contract,
      dataProviders[0],
      "./test/ddex-messages/new_release.xml",
      imageId
    );

    expect(await ddexSequencer.contract.blobQueueHead()).equal(blobhash);
    expect(await ddexSequencer.contract.blobQueueTail()).equal(blobhash);

    const blobDetailsBefore = await ddexSequencer.contract.blobs(blobhash);
    await(await ddexSequencer.contract
      .connect(validators[0])
      .submitProof(imageId,JOURNAL_EXAMPLE, "0x00")).wait();
    const blobDetailsAfter = await ddexSequencer.contract.blobs(blobhash);
    expect(await ddexSequencer.contract.blobQueueHead()).equal(ZERO_BYTES32);
    expect(await ddexSequencer.contract.blobQueueTail()).equal(ZERO_BYTES32);

    expect(blobDetailsBefore.nextBlob).equal(ZERO_BYTES32);
    expect(blobDetailsBefore.submitted).equal(true);
    expect(blobDetailsBefore.proposer).equal(dataProviders[0]);

    expect(blobDetailsAfter.nextBlob).equal(ZERO_BYTES32);
    expect(blobDetailsAfter.submitted).equal(false);
    expect(blobDetailsAfter.proposer).equal(ethers.ZeroAddress);
  });

  it("Move queue when proof is submitted (2 messages in the queue)", async () => {
    const { ddexSequencer, ddexEmitter: {imageId} } = fixture;

    const { blobhash: blobhash1 } = await sendBlob(
      ddexSequencer.contract,
      dataProviders[0],
      "./test/ddex-messages/new_release.xml",
      imageId
    );

    const { blobhash: blobhash2 } = await sendBlob(
      ddexSequencer.contract,
      dataProviders[0],
      "./test/ddex-messages/new_release2.xml",
      imageId
    );

    // first blob
    expect(await ddexSequencer.contract.blobQueueHead()).equal(blobhash1);
    expect(await ddexSequencer.contract.blobQueueTail()).equal(blobhash2);

    const blob1DetailsBefore = await ddexSequencer.contract.blobs(blobhash1);
    await ddexSequencer.contract
      .connect(validators[0])
      .submitProof(imageId, JOURNAL_EXAMPLE, "0x00");
    const blob1DetailsAfter = await ddexSequencer.contract.blobs(blobhash1);

    expect(blob1DetailsBefore.nextBlob).equal(blobhash2);
    expect(blob1DetailsBefore.submitted).equal(true);
    expect(blob1DetailsBefore.proposer).equal(dataProviders[0]);

    expect(blob1DetailsAfter.nextBlob).equal(ZERO_BYTES32);
    expect(blob1DetailsAfter.submitted).equal(false);
    expect(blob1DetailsAfter.proposer).equal(ethers.ZeroAddress);

    // second blob
    expect(await ddexSequencer.contract.blobQueueHead()).equal(blobhash2);
    expect(await ddexSequencer.contract.blobQueueTail()).equal(blobhash2);
    const blob2DetailsBefore = await ddexSequencer.contract.blobs(blobhash2);
    await ddexSequencer.contract
      .connect(validators[0])
      .submitProof(imageId, JOURNAL_EXAMPLE, "0x00");
    const blob2DetailsAfter = await ddexSequencer.contract.blobs(blobhash2);

    expect(blob2DetailsBefore.nextBlob).equal(ZERO_BYTES32);
    expect(blob2DetailsBefore.submitted).equal(true);
    expect(blob2DetailsBefore.proposer).equal(dataProviders[0]);

    expect(blob2DetailsAfter.nextBlob).equal(ZERO_BYTES32);
    expect(blob2DetailsAfter.submitted).equal(false);
    expect(blob2DetailsAfter.proposer).equal(ethers.ZeroAddress);

    // queue was cleared
    expect(await ddexSequencer.contract.blobQueueHead()).equal(ZERO_BYTES32);
    expect(await ddexSequencer.contract.blobQueueTail()).equal(ZERO_BYTES32);
  });

  it("Move queue when proof is submitted (3 messages in the queue)", async () => {
    const { ddexSequencer, ddexEmitter: {imageId} } = fixture;

    const { blobhash: blobhash1 } = await sendBlob(
      ddexSequencer.contract,
      dataProviders[0],
      "./test/ddex-messages/new_release.xml",
      imageId
    );

    const { blobhash: blobhash2 } = await sendBlob(
      ddexSequencer.contract,
      dataProviders[0],
      "./test/ddex-messages/new_release2.xml",
      imageId
    );

    const { blobhash: blobhash3 } = await sendBlob(
      ddexSequencer.contract,
      dataProviders[0],
      "./test/ddex-messages/new_release3.xml",
      imageId
    );

    // first blob
    expect(await ddexSequencer.contract.blobQueueHead()).equal(blobhash1);
    expect(await ddexSequencer.contract.blobQueueTail()).equal(blobhash3);

    const blob1DetailsBefore = await ddexSequencer.contract.blobs(blobhash1);
    await ddexSequencer.contract
      .connect(validators[0])
      .submitProof(imageId, JOURNAL_EXAMPLE, "0x00");
    const blob1DetailsAfter = await ddexSequencer.contract.blobs(blobhash1);

    expect(blob1DetailsBefore.nextBlob).equal(blobhash2);
    expect(blob1DetailsBefore.submitted).equal(true);
    expect(blob1DetailsBefore.proposer).equal(dataProviders[0]);

    expect(blob1DetailsAfter.nextBlob).equal(ZERO_BYTES32);
    expect(blob1DetailsAfter.submitted).equal(false);
    expect(blob1DetailsAfter.proposer).equal(ethers.ZeroAddress);

    // second blob
    expect(await ddexSequencer.contract.blobQueueHead()).equal(blobhash2);
    expect(await ddexSequencer.contract.blobQueueTail()).equal(blobhash3);
    const blob2DetailsBefore = await ddexSequencer.contract.blobs(blobhash2);
    await ddexSequencer.contract
      .connect(validators[0])
      .submitProof(imageId, JOURNAL_EXAMPLE, "0x00");
    const blob2DetailsAfter = await ddexSequencer.contract.blobs(blobhash2);

    expect(blob2DetailsBefore.nextBlob).equal(blobhash3);
    expect(blob2DetailsBefore.submitted).equal(true);
    expect(blob2DetailsBefore.proposer).equal(dataProviders[0]);

    expect(blob2DetailsAfter.nextBlob).equal(ZERO_BYTES32);
    expect(blob2DetailsAfter.submitted).equal(false);
    expect(blob2DetailsAfter.proposer).equal(ethers.ZeroAddress);

    // third blob
    expect(await ddexSequencer.contract.blobQueueHead()).equal(blobhash3);
    expect(await ddexSequencer.contract.blobQueueTail()).equal(blobhash3);
    const blob3DetailsBefore = await ddexSequencer.contract.blobs(blobhash3);
    await ddexSequencer.contract
      .connect(validators[0])
      .submitProof(imageId, JOURNAL_EXAMPLE, "0x00");
    const blob3DetailsAfter = await ddexSequencer.contract.blobs(blobhash3);

    expect(blob3DetailsBefore.nextBlob).equal(ZERO_BYTES32);
    expect(blob3DetailsBefore.submitted).equal(true);
    expect(blob3DetailsBefore.proposer).equal(dataProviders[0]);

    expect(blob3DetailsAfter.nextBlob).equal(ZERO_BYTES32);
    expect(blob3DetailsAfter.submitted).equal(false);
    expect(blob3DetailsAfter.proposer).equal(ethers.ZeroAddress);

    // queue was cleared
    expect(await ddexSequencer.contract.blobQueueHead()).equal(ZERO_BYTES32);
    expect(await ddexSequencer.contract.blobQueueTail()).equal(ZERO_BYTES32);
  });

  it("Can't submit proof for empty queue", async () => {
    const { ddexSequencer, ddexEmitter: {imageId} } = fixture;
    expect(await ddexSequencer.contract.blobQueueHead()).equal(ZERO_BYTES32);
    await expect(
      ddexSequencer.contract
        .connect(validators[0])
        .submitProof(imageId, JOURNAL_EXAMPLE, "0x00")
    ).to.rejected;
  });

  it("Rejects on incompatible blob imageId", async () => {
    const dataProvider = dataProviders[0];
    const { ddexSequencer, ddexEmitter } = fixture;
    
    const kzgInput1 = await KzgHelper.generate(
      "./test/ddex-messages/new_release.xml"
    );

    const kzgInput2 = await KzgHelper.generate(
      "./test/ddex-messages/new_release2.xml"
    );

    const submitBlob = (imageId: BytesLike, kzgInput: KzgOutput) => {
      return ddexSequencer.contract
        .connect(dataProvider)
        .submitNewBlob(imageId, kzgInput.commitment, kzgInput.blobSha2, {
          type: 3,
          maxFeePerBlobGas: 10,
          gasLimit: 1000000,
          blobs: [
            {
              data: kzgInput.blobFile,
              proof: kzgInput.proof,
              commitment: kzgInput.commitment,
            },
          ],
        })
    }

    // rejects: bytes32(0)
    await expect(submitBlob(ZERO_BYTES32, kzgInput1)).to.be.rejectedWith("DdexSequencer: ImageId cannot be 0");
    
    // rejects: unknown version
    await expect(submitBlob(ethers.randomBytes(32), kzgInput1)).to.be.rejectedWith("DdexSequencer: Unsupported imageId");
    
    // success: imageId == currentBlobImageId
    await expect(submitBlob(ddexEmitter.imageId, kzgInput1)).not.to.be.rejected;

    const currTarget = await ddexEmitter.contract.BLOB_CURRENT_IMAGE_ID()
    const prevTarget = await ddexEmitter.contract.BLOB_PREVIOUS_IMAGE_ID()

    // previous = current, current = new one
    await(await ddexEmitter.contract.setImageId(currTarget, ethers.randomBytes(32))).wait();
    await(await ddexEmitter.contract.setImageId(prevTarget, ddexEmitter.imageId)).wait();

    // success: imageId == currentBlobImageId
    await expect(submitBlob(ddexEmitter.imageId, kzgInput2)).not.to.be.rejected;
  })

  it("Rejects on incompatible verifier imageId", async () => {
    const dataProvider = dataProviders[0];
    const { ddexSequencer, ddexEmitter } = fixture;

    await sendBlob(
      ddexSequencer.contract,
      dataProviders[0],
      "./test/ddex-messages/new_release.xml",
      ddexEmitter.imageId
    );

    await sendBlob(
      ddexSequencer.contract,
      dataProviders[0],
      "./test/ddex-messages/new_release2.xml",
      ddexEmitter.imageId
    );

    // rejects: bytes32(0)
    await expect(ddexSequencer.contract
    .connect(validators[0])
    .submitProof(ZERO_BYTES32, JOURNAL_EXAMPLE, "0x00")).to.be.rejectedWith("DdexEmitter: ImageId cannot be 0");
    
    // rejects: unknown version
    await expect(ddexSequencer.contract
      .connect(validators[0])
      .submitProof(ethers.randomBytes(32), JOURNAL_EXAMPLE, "0x00")).to.be.rejectedWith("DdexEmitter: Unsupported imageId");
    
    // success: imageId == currentBlobImageId
    await expect(ddexSequencer.contract
      .connect(validators[0])
      .submitProof(ddexEmitter.imageId, JOURNAL_EXAMPLE, "0x00")).not.to.be.rejected;

    const currTarget = await ddexEmitter.contract.VERIFIER_CURRENT_IMAGE_ID()
    const prevTarget = await ddexEmitter.contract.VERIFIER_PREVIOUS_IMAGE_ID()

    // previous = current, current = new one
    await(await ddexEmitter.contract.setImageId(currTarget, ethers.randomBytes(32))).wait();
    await(await ddexEmitter.contract.setImageId(prevTarget, ddexEmitter.imageId)).wait();

    // success: imageId == currentBlobImageId
    await expect(ddexSequencer.contract
      .connect(validators[0])
      .submitProof(ddexEmitter.imageId, JOURNAL_EXAMPLE, "0x00")).not.to.be.rejected;
  })
});
