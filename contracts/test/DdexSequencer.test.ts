import { ethers, parseEther, Signer, Wallet } from "ethers";
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
    const { ddexSequencer } = fixture;
    const kzgInput = await KzgHelper.generate(
      "./test/ddex-messages/new_release.xml"
    );

    // check that the queue is emtpy
    expect(await ddexSequencer.contract.blobQueueHead()).equal(ZERO_BYTES32);
    expect(await ddexSequencer.contract.blobQueueTail()).equal(ZERO_BYTES32);
    await expect(
      ddexSequencer.contract
        .connect(dataProvider)
        .submitNewBlob(kzgInput.commitment, kzgInput.blobSha2, {
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
    const { ddexSequencer } = fixture;

    const blob1Result = await sendBlob(
      ddexSequencer.contract,
      dataProviders[0],
      "./test/ddex-messages/new_release.xml"
    );

    const blob2Result = await sendBlob(
      ddexSequencer.contract,
      dataProviders[0],
      "./test/ddex-messages/new_release2.xml"
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
      "./test/ddex-messages/new_release3.xml"
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
      dataProviders: [dataProvider],
    } = fixture;

    const { blobhash: blobhash1 } = await sendBlob(
      ddexSequencer.contract,
      dataProviders[0],
      "./test/ddex-messages/new_release.xml"
    );

    expect((await ddexSequencer.contract.blobs(blobhash1)).nextBlob).equal(
      ZERO_BYTES32
    );

    expect(await ddexSequencer.contract.blobQueueTail()).equal(blobhash1);

    const { blobhash: blobhash2 } = await sendBlob(
      ddexSequencer.contract,
      dataProviders[0],
      "./test/ddex-messages/new_release2.xml"
    );

    expect((await ddexSequencer.contract.blobs(blobhash1)).nextBlob).equal(
      blobhash2
    );
    expect(await ddexSequencer.contract.blobQueueTail()).equal(blobhash2);

    const { blobhash: blobhash3 } = await sendBlob(
      ddexSequencer.contract,
      dataProviders[0],
      "./test/ddex-messages/new_release3.xml"
    );

    expect((await ddexSequencer.contract.blobs(blobhash2)).nextBlob).equal(
      blobhash3
    );
    expect(await ddexSequencer.contract.blobQueueTail()).equal(blobhash3);
  });

  it("Clear queue after submitting proof for the last message", async () => {
    const { ddexSequencer } = fixture;

    const { blobhash } = await sendBlob(
      ddexSequencer.contract,
      dataProviders[0],
      "./test/ddex-messages/new_release.xml"
    );

    expect(await ddexSequencer.contract.blobQueueHead()).equal(blobhash);
    expect(await ddexSequencer.contract.blobQueueTail()).equal(blobhash);

    const blobDetailsBefore = await ddexSequencer.contract.blobs(blobhash);
    await ddexSequencer.contract
      .connect(validators[0])
      .submitProof(JOURNAL_EXAMPLE, "0x00");
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
    const { ddexSequencer } = fixture;

    const { blobhash: blobhash1 } = await sendBlob(
      ddexSequencer.contract,
      dataProviders[0],
      "./test/ddex-messages/new_release.xml"
    );

    const { blobhash: blobhash2 } = await sendBlob(
      ddexSequencer.contract,
      dataProviders[0],
      "./test/ddex-messages/new_release2.xml"
    );

    // first blob
    expect(await ddexSequencer.contract.blobQueueHead()).equal(blobhash1);
    expect(await ddexSequencer.contract.blobQueueTail()).equal(blobhash2);

    const blob1DetailsBefore = await ddexSequencer.contract.blobs(blobhash1);
    await ddexSequencer.contract
      .connect(validators[0])
      .submitProof(JOURNAL_EXAMPLE, "0x00");
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
      .submitProof(JOURNAL_EXAMPLE, "0x00");
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
    const { ddexSequencer } = fixture;

    const { blobhash: blobhash1 } = await sendBlob(
      ddexSequencer.contract,
      dataProviders[0],
      "./test/ddex-messages/new_release.xml"
    );

    const { blobhash: blobhash2 } = await sendBlob(
      ddexSequencer.contract,
      dataProviders[0],
      "./test/ddex-messages/new_release2.xml"
    );

    const { blobhash: blobhash3 } = await sendBlob(
      ddexSequencer.contract,
      dataProviders[0],
      "./test/ddex-messages/new_release3.xml"
    );

    // first blob
    expect(await ddexSequencer.contract.blobQueueHead()).equal(blobhash1);
    expect(await ddexSequencer.contract.blobQueueTail()).equal(blobhash3);

    const blob1DetailsBefore = await ddexSequencer.contract.blobs(blobhash1);
    await ddexSequencer.contract
      .connect(validators[0])
      .submitProof(JOURNAL_EXAMPLE, "0x00");
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
      .submitProof(JOURNAL_EXAMPLE, "0x00");
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
      .submitProof(JOURNAL_EXAMPLE, "0x00");
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
    const { ddexSequencer } = fixture;
    expect(await ddexSequencer.contract.blobQueueHead()).equal(ZERO_BYTES32);
    await expect(
      ddexSequencer.contract
        .connect(validators[0])
        .submitProof(JOURNAL_EXAMPLE, "0x00")
    ).to.rejected;
  });
});
