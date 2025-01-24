import { ethers } from "hardhat";
import { RiscZeroGroth16Verifier } from "../../../../typechain-types";
import { DeploymentOutput } from "../types";

export async function deployRiscZeroGroth16Verifier(): Promise<
  DeploymentOutput<RiscZeroGroth16Verifier>
> {
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

  return {
    contract: riscZeroGroth16Verifier,
    contractVerificationInput: {
      deployedContractAddress: await riscZeroGroth16Verifier.getAddress(),
      args: [
        await controlId.CONTROL_ROOT(),
        await controlId.BN254_CONTROL_ID(),
      ],
    },
  };
}
