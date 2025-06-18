import { BytesLike, ethers, parseEther, Signer, ZeroAddress } from "ethers";
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
      fakeImageId: true,
    });
  });

  describe("Queue orchestration", () => {
    it("Can add to empty queue", async () => {
      const dataProvider = dataProviders[0];
      const {
        ddexSequencer,
        ddexEmitter: { imageId },
      } = fixture;
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
      const {
        ddexSequencer,
        ddexEmitter: { imageId },
      } = fixture;

      const blob1Result = await sendBlob(
        ddexSequencer.contract,
        dataProviders[0],
        "./test/ddex-messages/new_release.xml",
        imageId
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
        ddexEmitter: { imageId },
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
      const {
        ddexSequencer,
        ddexEmitter: { imageId },
      } = fixture;

      const { blobhash } = await sendBlob(
        ddexSequencer.contract,
        dataProviders[0],
        "./test/ddex-messages/new_release.xml",
        imageId
      );

      expect(await ddexSequencer.contract.blobQueueHead()).equal(blobhash);
      expect(await ddexSequencer.contract.blobQueueTail()).equal(blobhash);

      const blobDetailsBefore = await ddexSequencer.contract.blobs(blobhash);
      await ddexSequencer.contract.connect(validators[0]).assignBlob();
      await (
        await ddexSequencer.contract
          .connect(validators[0])
          .submitProof(imageId, JOURNAL_EXAMPLE, "0x00", "ipfscid")
      ).wait();
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
      const {
        ddexSequencer,
        ddexEmitter: { imageId },
      } = fixture;

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

      // assign both blobs to validator[0]
      await ddexSequencer.contract.connect(validators[0]).assignBlob();
      await ddexSequencer.contract.connect(validators[0]).assignBlob();

      // first blob
      expect(await ddexSequencer.contract.blobQueueHead()).equal(blobhash1);
      expect(await ddexSequencer.contract.blobQueueTail()).equal(blobhash2);

      const blob1DetailsBefore = await ddexSequencer.contract.blobs(blobhash1);

      await ddexSequencer.contract
        .connect(validators[0])
        .submitProof(imageId, JOURNAL_EXAMPLE, "0x00", "ipfscid");
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
        .submitProof(imageId, JOURNAL_EXAMPLE, "0x00", "ipfscid");
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
      const {
        ddexSequencer,
        ddexEmitter: { imageId },
      } = fixture;

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

      // assign all blobs to validator[0]
      await ddexSequencer.contract.connect(validators[0]).assignBlob();
      await ddexSequencer.contract.connect(validators[0]).assignBlob();
      await ddexSequencer.contract.connect(validators[0]).assignBlob();

      // first blob
      expect(await ddexSequencer.contract.blobQueueHead()).equal(blobhash1);
      expect(await ddexSequencer.contract.blobQueueTail()).equal(blobhash3);

      const blob1DetailsBefore = await ddexSequencer.contract.blobs(blobhash1);
      await ddexSequencer.contract
        .connect(validators[0])
        .submitProof(imageId, JOURNAL_EXAMPLE, "0x00", "ipfscid");
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
        .submitProof(imageId, JOURNAL_EXAMPLE, "0x00", "ipfscid");
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
        .submitProof(imageId, JOURNAL_EXAMPLE, "0x00", "ipfscid");
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
      const {
        ddexSequencer,
        ddexEmitter: { imageId },
      } = fixture;
      expect(await ddexSequencer.contract.blobQueueHead()).equal(ZERO_BYTES32);
      await expect(
        ddexSequencer.contract
          .connect(validators[0])
          .submitProof(imageId, JOURNAL_EXAMPLE, "0x00", "ipfscid")
      ).to.rejected;
    });
    it("Queue can be cleared and repopulate again", async () => {
      const {
        ddexSequencer,
        ddexEmitter: { imageId },
      } = fixture;

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

      // validator[0] is assigned to blobhash1 and submits proof
      await ddexSequencer.contract.connect(validators[0]).assignBlob();
      expect(await ddexSequencer.contract.blobQueueHead()).equal(blobhash1);
      await ddexSequencer.contract
        .connect(validators[0])
        .submitProof(imageId, JOURNAL_EXAMPLE, "0x00", "ipfscid");

      const { blobhash: blobhash3 } = await sendBlob(
        ddexSequencer.contract,
        dataProviders[0],
        "./test/ddex-messages/new_release3.xml",
        imageId
      );

      // validator[0] is assigned to blobhash2 and validator[1] to blobhash3
      await ddexSequencer.contract.connect(validators[0]).assignBlob();
      await ddexSequencer.contract.connect(validators[1]).assignBlob();

      // validator[0] submits proof for blobhash2 and validator[1] for blobhash3
      expect(await ddexSequencer.contract.blobQueueHead()).equal(blobhash2);
      await ddexSequencer.contract
        .connect(validators[0])
        .submitProof(imageId, JOURNAL_EXAMPLE, "0x00", "ipfscid");

      expect(await ddexSequencer.contract.blobQueueHead()).equal(blobhash3);
      await ddexSequencer.contract
        .connect(validators[1])
        .submitProof(imageId, JOURNAL_EXAMPLE, "0x00", "ipfscid");

      // queue was cleared
      expect(await ddexSequencer.contract.blobQueueHead()).equal(ZERO_BYTES32);
      expect(await ddexSequencer.contract.nextBlobAssignment()).equal(
        ZERO_BYTES32
      );

      // repopulate queue
      const { blobhash: blobhash4 } = await sendBlob(
        ddexSequencer.contract,
        dataProviders[0],
        "./test/ddex-messages/new_release4.xml",
        imageId
      );
      expect(await ddexSequencer.contract.blobQueueHead()).equal(blobhash4);
      expect(await ddexSequencer.contract.nextBlobAssignment()).equal(
        blobhash4
      );

      // validator[0] is assigned to blobhash4 and submits proof
      await ddexSequencer.contract.connect(validators[1]).assignBlob();
      await ddexSequencer.contract
        .connect(validators[1])
        .submitProof(imageId, JOURNAL_EXAMPLE, "0x00", "ipfscid");

      // queue was cleared again
      expect(await ddexSequencer.contract.blobQueueHead()).equal(ZERO_BYTES32);
      expect(await ddexSequencer.contract.nextBlobAssignment()).equal(
        ZERO_BYTES32
      );
    });
  });

  describe("ImageId compatibility", () => {
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
          });
      };

      // rejects: bytes32(0)
      await expect(submitBlob(ZERO_BYTES32, kzgInput1)).to.be.rejectedWith(
        "DdexSequencer: ImageId cannot be 0"
      );

      // rejects: unknown version
      await expect(
        submitBlob(ethers.randomBytes(32), kzgInput1)
      ).to.be.rejectedWith("DdexSequencer: Unsupported imageId");

      // success: imageId == currentBlobImageId
      await expect(submitBlob(ddexEmitter.imageId, kzgInput1)).not.to.be
        .rejected;

      const currTarget = await ddexEmitter.contract.BLOB_CURRENT_IMAGE_ID();
      const prevTarget = await ddexEmitter.contract.BLOB_PREVIOUS_IMAGE_ID();

      // previous = current, current = new one
      await (
        await ddexEmitter.contract.setImageIds(
          [currTarget, prevTarget],
          [ethers.randomBytes(32), ddexEmitter.imageId],
          [ethers.Wallet.createRandom(), ethers.Wallet.createRandom()]
        )
      ).wait();

      // success: imageId == currentBlobImageId
      await expect(submitBlob(ddexEmitter.imageId, kzgInput2)).not.to.be
        .rejected;
    });

    it("Rejects on incompatible verifier imageId", async () => {
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

      await ddexSequencer.contract.connect(validators[0]).assignBlob();
      await ddexSequencer.contract.connect(validators[0]).assignBlob();

      // rejects: bytes32(0)
      await expect(
        ddexSequencer.contract
          .connect(validators[0])
          .submitProof(ZERO_BYTES32, JOURNAL_EXAMPLE, "0x00", "ipfscid")
      ).to.be.rejectedWith("DdexEmitter: ImageId cannot be 0");

      // rejects: unknown version
      await expect(
        ddexSequencer.contract
          .connect(validators[0])
          .submitProof(
            ethers.randomBytes(32),
            JOURNAL_EXAMPLE,
            "0x00",
            "ipfscid"
          )
      ).to.be.rejectedWith("DdexEmitter: Unsupported imageId");

      // success: imageId == currentBlobImageId
      await expect(
        ddexSequencer.contract
          .connect(validators[0])
          .submitProof(ddexEmitter.imageId, JOURNAL_EXAMPLE, "0x00", "ipfscid")
      ).not.to.be.rejected;

      const currTarget = await ddexEmitter.contract.VERIFIER_CURRENT_IMAGE_ID();
      const prevTarget =
        await ddexEmitter.contract.VERIFIER_PREVIOUS_IMAGE_ID();

      // previous = current, current = new one
      await (
        await ddexEmitter.contract.setImageIds(
          [currTarget, prevTarget],
          [ethers.randomBytes(32), ddexEmitter.imageId],
          [
            ethers.Wallet.createRandom(),
            fixture.fixtureAddresses.riscZeroGroth16Verifier,
          ]
        )
      ).wait();

      // success: imageId == currentBlobImageId
      await expect(
        ddexSequencer.contract
          .connect(validators[0])
          .submitProof(ddexEmitter.imageId, JOURNAL_EXAMPLE, "0x00", "ipfscid")
      ).not.to.be.rejected;
    });
  });

  describe("Blob assignment", () => {
    it("Assign different validators to BLOBs", async () => {
      const {
        ddexSequencer,
        ddexEmitter: { imageId },
      } = fixture;

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
      // this should assing blobhash1 to validator[0]
      await ddexSequencer.contract.connect(validators[0]).assignBlob();

      // this should assing blobhash2 to validator[1]
      await ddexSequencer.contract.connect(validators[1]).assignBlob();

      const blob1Details = await ddexSequencer.contract.blobs(blobhash1);
      const blob2Details = await ddexSequencer.contract.blobs(blobhash2);

      expect(blob1Details.assignedValidator).equal(
        await validators[0].getAddress()
      );
      expect(blob2Details.assignedValidator).equal(
        await validators[1].getAddress()
      );
    });
    it("Only assigned validator can post proof for BLOB", async () => {
      const {
        ddexSequencer,
        ddexEmitter: { imageId },
      } = fixture;

      const { blobhash } = await sendBlob(
        ddexSequencer.contract,
        dataProviders[0],
        "./test/ddex-messages/new_release.xml",
        imageId
      );

      await ddexSequencer.contract.connect(validators[0]).assignBlob();

      const blobDetails = await ddexSequencer.contract.blobs(blobhash);
      expect(blobDetails.assignedValidator).equal(
        await validators[0].getAddress()
      );

      await expect(
        ddexSequencer.contract
          .connect(validators[1])
          .submitProof(imageId, JOURNAL_EXAMPLE, "0x00", "ipfscid")
      ).to.rejected;

      await expect(
        ddexSequencer.contract
          .connect(validators[0])
          .submitProof(imageId, JOURNAL_EXAMPLE, "0x00", "ipfscid")
      ).to.not.rejected;
    });
    it("Can't assign blob when queue is empty", async () => {
      const {
        ddexSequencer,
        ddexEmitter: { imageId },
      } = fixture;

      await expect(ddexSequencer.contract.connect(validators[0]).assignBlob())
        .to.rejected;
    });
    it("Emit event when blob is assigned", async () => {
      const {
        ddexSequencer,
        ddexEmitter: { imageId },
      } = fixture;

      const { blobhash } = await sendBlob(
        ddexSequencer.contract,
        dataProviders[0],
        "./test/ddex-messages/new_release.xml",
        imageId
      );

      const tx = await ddexSequencer.contract
        .connect(validators[0])
        .assignBlob();

      const receipt = await tx.wait();

      const filter = ddexSequencer.contract.filters.BlobAssigned;
      const events = await ddexSequencer.contract.queryFilter(
        filter,
        receipt?.blockNumber,
        receipt?.blockNumber
      );

      const blobFromEvent = events[0].args.blob;
      const assignedValidator = events[0].args.assignedValidator;

      expect(blobFromEvent).equal(blobhash);
      expect(assignedValidator).equal(await validators[0].getAddress());
    });
    it("Reassign head after time for processing passed", async () => {
      const {
        ddexSequencer,
        ddexEmitter: { imageId },
      } = fixture;

      const timeForProcessing =
        await ddexSequencer.contract.headProcessingTimeInBlocks();

      const { blobhash: blobQueueHead } = await sendBlob(
        ddexSequencer.contract,
        dataProviders[0],
        "./test/ddex-messages/new_release.xml",
        imageId
      );

      await sendBlob(
        ddexSequencer.contract,
        dataProviders[0],
        "./test/ddex-messages/new_release2.xml",
        imageId
      );

      // assign blob from queue head to validator[0]
      // since now validator[0] has 'timeForProcessing' blocks to process the blob
      await ddexSequencer.contract.connect(validators[0]).assignBlob();

      // assign second blob to validator[1]
      await ddexSequencer.contract.connect(validators[1]).assignBlob();

      expect(
        (await ddexSequencer.contract.blobs(blobQueueHead)).assignedValidator
      ).equal(await validators[0].getAddress());

      // mine 'timeForProcessing' blocks
      for (let i = 0; i < timeForProcessing; i++) {
        await hre.ethers.provider.send("evm_mine", []);
      }

      // validator[0] can no longer submit proof for blobQueueHead
      await expect(
        ddexSequencer.contract
          .connect(validators[0])
          .submitProof(imageId, JOURNAL_EXAMPLE, "0x00", "ipfscid")
      ).to.rejected;

      // validator[1] reclaim assignment to queue head
      await ddexSequencer.contract.connect(validators[1]).assignBlob();

      expect(
        (await ddexSequencer.contract.blobs(blobQueueHead)).assignedValidator
      ).equal(await validators[1].getAddress());
    });

    it("Clear processing time when queue is moved", async () => {
      const {
        ddexSequencer,
        ddexEmitter: { imageId },
      } = fixture;

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

      const headAssignmentTx = await ddexSequencer.contract
        .connect(validators[0])
        .assignBlob();
      const headAssignmentTxReceipt = await headAssignmentTx.wait();
      const blob1ProcessingTimeStartBlock =
        headAssignmentTxReceipt?.blockNumber;

      expect(await ddexSequencer.contract.blobQueueHead()).equal(blobhash1);
      expect(blob1ProcessingTimeStartBlock).equal(
        await ddexSequencer.contract.headProcessingStartBlock()
      );

      const submitProofTx = await ddexSequencer.contract
        .connect(validators[0])
        .submitProof(imageId, JOURNAL_EXAMPLE, "0x00", "ipfscid");
      const submitProofReceipt = await submitProofTx.wait();
      const blob2ProcessingTimeStartBlock = submitProofReceipt?.blockNumber;

      expect(await ddexSequencer.contract.blobQueueHead()).equal(blobhash2);
      expect(blob2ProcessingTimeStartBlock).equal(
        await ddexSequencer.contract.headProcessingStartBlock()
      );
    });
    it("BLOBs proofs still need to be sent in order (only proof for queue head can be accepted)", async () => {
      const {
        ddexSequencer,
        ddexEmitter: { imageId },
      } = fixture;

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

      // assign blobhash1 to validator[0]
      await ddexSequencer.contract.connect(validators[0]).assignBlob();

      // assign blobhash2 to validator[1]
      await ddexSequencer.contract.connect(validators[1]).assignBlob();

      await expect(
        ddexSequencer.contract
          .connect(validators[1])
          .submitProof(imageId, JOURNAL_EXAMPLE, "0x00", "ipfscid")
      ).to.rejected;

      await expect(
        ddexSequencer.contract
          .connect(validators[0])
          .submitProof(imageId, JOURNAL_EXAMPLE, "0x00", "ipfscid")
      ).to.not.rejected;
    });

    it("Validator can be assigned to more than one blob", async () => {
      const {
        ddexSequencer,
        ddexEmitter: { imageId },
      } = fixture;

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

      await ddexSequencer.contract.connect(validators[0]).assignBlob();
      await ddexSequencer.contract.connect(validators[0]).assignBlob();
      await ddexSequencer.contract.connect(validators[0]).assignBlob();

      const validatorAddress = await validators[0].getAddress();

      expect(
        (await ddexSequencer.contract.blobs(blobhash1)).assignedValidator
      ).equal(validatorAddress);
      expect(
        (await ddexSequencer.contract.blobs(blobhash2)).assignedValidator
      ).equal(validatorAddress);
      expect(
        (await ddexSequencer.contract.blobs(blobhash3)).assignedValidator
      ).equal(validatorAddress);
    });

    it("Can't assign BLOB when all blobs were already assigned", async () => {
      const {
        ddexSequencer,
        ddexEmitter: { imageId },
      } = fixture;

      await sendBlob(
        ddexSequencer.contract,
        dataProviders[0],
        "./test/ddex-messages/new_release.xml",
        imageId
      );
      await sendBlob(
        ddexSequencer.contract,
        dataProviders[0],
        "./test/ddex-messages/new_release2.xml",
        imageId
      );

      await ddexSequencer.contract.connect(validators[0]).assignBlob();
      await ddexSequencer.contract.connect(validators[1]).assignBlob();

      await expect(ddexSequencer.contract.assignBlob()).to.rejected;
    });
    it("Can't submit proof for queue head BLOB if it wasn't assigned to validator", async () => {
      const {
        ddexSequencer,
        ddexEmitter: { imageId },
      } = fixture;

      const { blobhash } = await sendBlob(
        ddexSequencer.contract,
        dataProviders[0],
        "./test/ddex-messages/new_release.xml",
        imageId
      );

      expect(await ddexSequencer.contract.blobQueueHead()).equal(blobhash);
      expect(await ddexSequencer.contract.nextBlobAssignment()).equal(blobhash);
      expect(
        (await ddexSequencer.contract.blobs(blobhash)).assignedValidator
      ).equal(ZeroAddress);

      await expect(
        ddexSequencer.contract
          .connect(validators[0])
          .submitProof(imageId, JOURNAL_EXAMPLE, "0x00", "ipfscid")
      ).to.rejected;
    });
  });
  describe("Blob expiry", () => {
    it("Rejects to remove exipred blob from queue", async () => {
      const dataProvider = dataProviders[0];
      const validator = validators[0];
      const { ddexSequencer, ddexEmitter } = fixture;

      await sendBlob(
        ddexSequencer.contract,
        dataProvider,
        "./test/ddex-messages/new_release.xml",
        ddexEmitter.imageId
      );

      await expect(
        ddexSequencer.contract.connect(validator).removeExpiredBlob()
      ).to.be.rejectedWith("DdexSequencer: Blob is still considered alive");
    });

    it("Removes expired blob", async () => {
      const BLOB_LIFETIME = 10;

      const dataProvider = dataProviders[0];
      const validator = validators[0];
      const { ddexSequencer, ddexEmitter } = fixture;
      await ddexSequencer.contract.setBlobLifetime(BLOB_LIFETIME);

      await sendBlob(
        ddexSequencer.contract,
        dataProvider,
        "./test/ddex-messages/new_release.xml",
        ddexEmitter.imageId
      );

      let queueHead = await ddexSequencer.contract.blobQueueHead();
      let queueTail = await ddexSequencer.contract.blobQueueTail();

      expect(queueHead).not.to.equal(hre.ethers.ZeroAddress);
      expect(queueTail).not.to.equal(hre.ethers.ZeroAddress);

      for (let i = 0; i < BLOB_LIFETIME; i++) {
        await hre.ethers.provider.send("evm_mine", []);
      }
      await expect(
        ddexSequencer.contract.connect(validator).removeExpiredBlob()
      ).not.to.be.rejected;

      queueHead = await ddexSequencer.contract.blobQueueHead();
      queueTail = await ddexSequencer.contract.blobQueueTail();

      expect(queueHead).to.equal(BigInt(0));
      expect(queueTail).to.equal(BigInt(0));
    });
  });
});
