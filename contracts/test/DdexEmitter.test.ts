import { ethers, parseEther, Signer} from "ethers";
import { FixtureOutput } from "../scripts/fixture/fixture.types";
import {
  deployFixture,
  getEthersType3Wallets,
} from "../scripts/fixture/fixture.deploy";
import { expect } from "chai";
import hre from "hardhat";

const ZERO_BYTES32 =
  "0x0000000000000000000000000000000000000000000000000000000000000000";

describe("DdexEmitter", () => {
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
      fakeImageId: true
    });
  });

  it("Gets/sets image ids", async () => {
    const {ddexEmitter} = fixture;
    const initialImageId = ethers.hexlify(ddexEmitter.imageId)
    
    // accepted targets are 0x01 to 0x04
    await expect(ddexEmitter.contract.setImageIds(["0x00"], [ethers.randomBytes(32)])).to.be.rejectedWith("DdexEmitter: Invalid target")
    await expect(ddexEmitter.contract.setImageIds(["0x05"], [ethers.randomBytes(32)])).to.be.rejectedWith("DdexEmitter: Invalid target")
    await expect(ddexEmitter.contract.setImageIds(["0x01", "0x02"], [ethers.randomBytes(32)])).to.be.rejectedWith("DdexEmitter: Mismatched array lengths")


    let [currentBlobImageId, previousBlobImageId] = await ddexEmitter.contract.getSupportedBlobImageIds();
    
    expect(currentBlobImageId).to.equal(initialImageId);
    expect(previousBlobImageId).to.equal(ZERO_BYTES32);
    
    let [currentVerifierImageId, previousVerifierImageId] = await ddexEmitter.contract.getSupportedVerifierImageIds();
    
    expect(currentVerifierImageId).to.equal(initialImageId);
    expect(previousVerifierImageId).to.equal(ZERO_BYTES32);

    const currBlobTarget = await ddexEmitter.contract.BLOB_CURRENT_IMAGE_ID()
    const prevBlobTarget = await ddexEmitter.contract.BLOB_PREVIOUS_IMAGE_ID()
    const currVerifierTarget = await ddexEmitter.contract.VERIFIER_CURRENT_IMAGE_ID()
    const prevVerifierTarget = await ddexEmitter.contract.VERIFIER_PREVIOUS_IMAGE_ID()

    const newCurrBlobImageId = ethers.hexlify(ethers.randomBytes(32));
    const newPrevBlobImageId = ethers.hexlify(ethers.randomBytes(32));
    const newCurrVerifierImageId = ethers.hexlify(ethers.randomBytes(32));
    const newPrevVerifierImageId = ethers.hexlify(ethers.randomBytes(32));
    
    await (
      await ddexEmitter.contract.setImageIds(
        [currBlobTarget, prevBlobTarget, currVerifierTarget, prevVerifierTarget], 
        [newCurrBlobImageId, newPrevBlobImageId, newCurrVerifierImageId, newPrevVerifierImageId]
      )
    ).wait()



    ;[currentBlobImageId, previousBlobImageId] = await ddexEmitter.contract.getSupportedBlobImageIds();
    ;[currentVerifierImageId, previousVerifierImageId] = await ddexEmitter.contract.getSupportedVerifierImageIds();

    expect(newCurrBlobImageId).to.equal(currentBlobImageId)
    expect(newPrevBlobImageId).to.equal(previousBlobImageId)
    expect(newCurrVerifierImageId).to.equal(currentVerifierImageId)
    expect(newPrevVerifierImageId).to.equal(previousVerifierImageId)
  })
})
