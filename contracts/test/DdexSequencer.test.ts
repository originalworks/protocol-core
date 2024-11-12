import { ethers } from "ethers";
import { FixtureOutput } from "../scripts/fixture/fixture.types";
import { deployFixture } from "../scripts/fixture/fixture.deploy";
import { KzgHelper } from "../scripts/actions/kzg/kzg";
import { expect } from "chai";
import { sendBlob } from "../scripts/actions/blobs/sendBlob";

const ZERO_BYTES32 =
  "0x0000000000000000000000000000000000000000000000000000000000000000";

describe("DdexSequencer", () => {
  let fixture: FixtureOutput;

  beforeEach(async () => {
    fixture = await deployFixture();
  });

  it("Can add to empty queue", async () => {
    const {
      ddexSequencer,
      dataProviders: [dataProvider],
    } = fixture;
    const kzgInput = await KzgHelper.generate(
      "./test/ddex-messages/new_release.xml"
    );
    const blobhash = KzgHelper.blobhashFromCommitment(kzgInput.commitment);

    // check that the queue is emtpy
    expect(await ddexSequencer.blobQueueHead()).equal(ZERO_BYTES32);
    expect(await ddexSequencer.blobQueueTail()).equal(ZERO_BYTES32);

    await expect(
      ddexSequencer.connect(dataProvider).submitNewBlob(kzgInput.commitment, {
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

    const blobsMappingResults = await ddexSequencer.blobs(blobhash);

    expect(blobsMappingResults.nextBlob).equal(ZERO_BYTES32);
    expect(blobsMappingResults.submitted).equal(true);
    expect(blobsMappingResults.proposer).equal(dataProvider);

    expect(await ddexSequencer.blobQueueHead()).equal(blobhash);
    expect(await ddexSequencer.blobQueueTail()).equal(blobhash);
  });

  it("Can add to non-empty queue", async () => {
    const {
      ddexSequencer,
      dataProviders: [dataProvider],
    } = fixture;

    const blob1Result = await sendBlob(
      ddexSequencer,
      dataProvider,
      "./test/ddex-messages/new_release.xml"
    );

    const blob2Result = await sendBlob(
      ddexSequencer,
      dataProvider,
      "./test/ddex-messages/new_release2.xml"
    );

    expect(await ddexSequencer.blobQueueHead()).equal(blob1Result.blobhash);
    expect(await ddexSequencer.blobQueueTail()).equal(blob2Result.blobhash);

    const blob3Result = await sendBlob(
      ddexSequencer,
      dataProvider,
      "./test/ddex-messages/new_release3.xml"
    );

    expect(await ddexSequencer.blobQueueHead()).equal(blob1Result.blobhash);
    expect(await ddexSequencer.blobQueueTail()).equal(blob3Result.blobhash);
  });

  it("Set nextBlob for previous tail after adding new message", async () => {
    const {
      ddexSequencer,
      dataProviders: [dataProvider],
    } = fixture;

    const { blobhash: blobhash1 } = await sendBlob(
      ddexSequencer,
      dataProvider,
      "./test/ddex-messages/new_release.xml"
    );

    expect((await ddexSequencer.blobs(blobhash1)).nextBlob).equal(ZERO_BYTES32);

    expect(await ddexSequencer.blobQueueTail()).equal(blobhash1);

    const { blobhash: blobhash2 } = await sendBlob(
      ddexSequencer,
      dataProvider,
      "./test/ddex-messages/new_release2.xml"
    );

    expect((await ddexSequencer.blobs(blobhash1)).nextBlob).equal(blobhash2);
    expect(await ddexSequencer.blobQueueTail()).equal(blobhash2);

    const { blobhash: blobhash3 } = await sendBlob(
      ddexSequencer,
      dataProvider,
      "./test/ddex-messages/new_release3.xml"
    );

    expect((await ddexSequencer.blobs(blobhash2)).nextBlob).equal(blobhash3);
    expect(await ddexSequencer.blobQueueTail()).equal(blobhash3);
  });

  it("Clear queue after submitting proof for the last message", async () => {
    const {
      ddexSequencer,
      dataProviders: [dataProvider],
      validators: [validator],
    } = fixture;

    const { blobhash } = await sendBlob(
      ddexSequencer,
      dataProvider,
      "./test/ddex-messages/new_release.xml"
    );

    expect(await ddexSequencer.blobQueueHead()).equal(blobhash);
    expect(await ddexSequencer.blobQueueTail()).equal(blobhash);

    const blobDetailsBefore = await ddexSequencer.blobs(blobhash);
    await ddexSequencer.connect(validator).submitProofOfProcessing(true, []);
    const blobDetailsAfter = await ddexSequencer.blobs(blobhash);

    expect(await ddexSequencer.blobQueueHead()).equal(ZERO_BYTES32);
    expect(await ddexSequencer.blobQueueTail()).equal(ZERO_BYTES32);

    expect(blobDetailsBefore.nextBlob).equal(ZERO_BYTES32);
    expect(blobDetailsBefore.submitted).equal(true);
    expect(blobDetailsBefore.proposer).equal(dataProvider);

    expect(blobDetailsAfter.nextBlob).equal(ZERO_BYTES32);
    expect(blobDetailsAfter.submitted).equal(false);
    expect(blobDetailsAfter.proposer).equal(ethers.ZeroAddress);
  });

  it("Move queue when proof is submitted (2 messages in the queue)", async () => {
    const {
      ddexSequencer,
      dataProviders: [dataProvider],
      validators: [validator],
    } = fixture;

    const { blobhash: blobhash1 } = await sendBlob(
      ddexSequencer,
      dataProvider,
      "./test/ddex-messages/new_release.xml"
    );

    const { blobhash: blobhash2 } = await sendBlob(
      ddexSequencer,
      dataProvider,
      "./test/ddex-messages/new_release2.xml"
    );

    // first blob
    expect(await ddexSequencer.blobQueueHead()).equal(blobhash1);
    expect(await ddexSequencer.blobQueueTail()).equal(blobhash2);

    const blob1DetailsBefore = await ddexSequencer.blobs(blobhash1);
    await ddexSequencer.connect(validator).submitProofOfProcessing(true, []);
    const blob1DetailsAfter = await ddexSequencer.blobs(blobhash1);

    expect(blob1DetailsBefore.nextBlob).equal(blobhash2);
    expect(blob1DetailsBefore.submitted).equal(true);
    expect(blob1DetailsBefore.proposer).equal(dataProvider);

    expect(blob1DetailsAfter.nextBlob).equal(ZERO_BYTES32);
    expect(blob1DetailsAfter.submitted).equal(false);
    expect(blob1DetailsAfter.proposer).equal(ethers.ZeroAddress);

    // second blob
    expect(await ddexSequencer.blobQueueHead()).equal(blobhash2);
    expect(await ddexSequencer.blobQueueTail()).equal(blobhash2);
    const blob2DetailsBefore = await ddexSequencer.blobs(blobhash2);
    await ddexSequencer.connect(validator).submitProofOfProcessing(true, []);
    const blob2DetailsAfter = await ddexSequencer.blobs(blobhash2);

    expect(blob2DetailsBefore.nextBlob).equal(ZERO_BYTES32);
    expect(blob2DetailsBefore.submitted).equal(true);
    expect(blob2DetailsBefore.proposer).equal(dataProvider);

    expect(blob2DetailsAfter.nextBlob).equal(ZERO_BYTES32);
    expect(blob2DetailsAfter.submitted).equal(false);
    expect(blob2DetailsAfter.proposer).equal(ethers.ZeroAddress);

    // queue was cleared
    expect(await ddexSequencer.blobQueueHead()).equal(ZERO_BYTES32);
    expect(await ddexSequencer.blobQueueTail()).equal(ZERO_BYTES32);
  });

  it("Move queue when proof is submitted (3 messages in the queue)", async () => {
    const {
      ddexSequencer,
      dataProviders: [dataProvider],
      validators: [validator],
    } = fixture;

    const { blobhash: blobhash1 } = await sendBlob(
      ddexSequencer,
      dataProvider,
      "./test/ddex-messages/new_release.xml"
    );

    const { blobhash: blobhash2 } = await sendBlob(
      ddexSequencer,
      dataProvider,
      "./test/ddex-messages/new_release2.xml"
    );

    const { blobhash: blobhash3 } = await sendBlob(
      ddexSequencer,
      dataProvider,
      "./test/ddex-messages/new_release3.xml"
    );

    // first blob
    expect(await ddexSequencer.blobQueueHead()).equal(blobhash1);
    expect(await ddexSequencer.blobQueueTail()).equal(blobhash3);

    const blob1DetailsBefore = await ddexSequencer.blobs(blobhash1);
    await ddexSequencer.connect(validator).submitProofOfProcessing(true, []);
    const blob1DetailsAfter = await ddexSequencer.blobs(blobhash1);

    expect(blob1DetailsBefore.nextBlob).equal(blobhash2);
    expect(blob1DetailsBefore.submitted).equal(true);
    expect(blob1DetailsBefore.proposer).equal(dataProvider);

    expect(blob1DetailsAfter.nextBlob).equal(ZERO_BYTES32);
    expect(blob1DetailsAfter.submitted).equal(false);
    expect(blob1DetailsAfter.proposer).equal(ethers.ZeroAddress);

    // second blob
    expect(await ddexSequencer.blobQueueHead()).equal(blobhash2);
    expect(await ddexSequencer.blobQueueTail()).equal(blobhash3);
    const blob2DetailsBefore = await ddexSequencer.blobs(blobhash2);
    await ddexSequencer.connect(validator).submitProofOfProcessing(true, []);
    const blob2DetailsAfter = await ddexSequencer.blobs(blobhash2);

    expect(blob2DetailsBefore.nextBlob).equal(blobhash3);
    expect(blob2DetailsBefore.submitted).equal(true);
    expect(blob2DetailsBefore.proposer).equal(dataProvider);

    expect(blob2DetailsAfter.nextBlob).equal(ZERO_BYTES32);
    expect(blob2DetailsAfter.submitted).equal(false);
    expect(blob2DetailsAfter.proposer).equal(ethers.ZeroAddress);

    // third blob
    expect(await ddexSequencer.blobQueueHead()).equal(blobhash3);
    expect(await ddexSequencer.blobQueueTail()).equal(blobhash3);
    const blob3DetailsBefore = await ddexSequencer.blobs(blobhash3);
    await ddexSequencer.connect(validator).submitProofOfProcessing(true, []);
    const blob3DetailsAfter = await ddexSequencer.blobs(blobhash3);

    expect(blob3DetailsBefore.nextBlob).equal(ZERO_BYTES32);
    expect(blob3DetailsBefore.submitted).equal(true);
    expect(blob3DetailsBefore.proposer).equal(dataProvider);

    expect(blob3DetailsAfter.nextBlob).equal(ZERO_BYTES32);
    expect(blob3DetailsAfter.submitted).equal(false);
    expect(blob3DetailsAfter.proposer).equal(ethers.ZeroAddress);

    // queue was cleared
    expect(await ddexSequencer.blobQueueHead()).equal(ZERO_BYTES32);
    expect(await ddexSequencer.blobQueueTail()).equal(ZERO_BYTES32);
  });

  it("Can't submit proof for empty queue", async () => {
    const {
      ddexSequencer,
      validators: [validator],
    } = fixture;
    expect(await ddexSequencer.blobQueueHead()).equal(ZERO_BYTES32);
    await expect(
      ddexSequencer.connect(validator).submitProofOfProcessing(true, [])
    ).to.rejected;
  });
});
