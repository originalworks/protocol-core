import { StakeVault } from "../../../../typechain-types/contracts/StakeVault";
import { ethers } from "hardhat";
import { StakeVaultDeploymentInput } from "./StakeVault.types";
import { DeploymentOutput } from "../types";

export async function deployStakeVault(
  input: StakeVaultDeploymentInput
): Promise<DeploymentOutput<StakeVault>> {
  const StakeVault = await ethers.getContractFactory("StakeVault");
  const stakeVault = await StakeVault.connect(input.deployer).deploy(
    input.stakeTokenAddress,
    input._slashRate
  );
  await stakeVault.waitForDeployment();

  return {
    contract: stakeVault,
    contractVerificationInput: {
      deployedContractAddress: await stakeVault.getAddress(),
      args: [input.stakeTokenAddress, input._slashRate],
    },
  };
}
