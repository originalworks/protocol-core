import { ethers } from "hardhat";
import { RiscZeroGroth16Verifier } from "../../../../typechain-types";
import hre from "hardhat";
import { verifyContract } from "../../verifyContract";

export async function deployRiscZeroGroth16Verifier(): Promise<RiscZeroGroth16Verifier> {
  const ControlID = await ethers.getContractFactory("ControlID");
  const controlId = await ControlID.deploy();
  await controlId.waitForDeployment();

  const RiscZeroGroth16Verifier = await ethers.getContractFactory(
    "RiscZeroGroth16Verifier"
  );

  const riscZeroGroth16Verifier = await RiscZeroGroth16Verifier.deploy(
    await controlId.CONTROL_ROOT(),
    await controlId.BN254_CONTROL_ID()
  );
  await riscZeroGroth16Verifier.waitForDeployment();
  await verifyContract(await riscZeroGroth16Verifier.getAddress(), hre, [
    await controlId.CONTROL_ROOT(),
    await controlId.BN254_CONTROL_ID(),
  ]);
  return riscZeroGroth16Verifier;
}
