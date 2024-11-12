import { StakeVault } from "../../../../typechain-types/contracts/StakeVault";
import { ethers } from "hardhat";
import { StakeVaultDeploymentInput } from "./StakeVault.types";

export async function deployStakeVault(
  input: StakeVaultDeploymentInput
): Promise<StakeVault> {
  const StakeVault = await ethers.getContractFactory("StakeVault");
  const stakeVault = await StakeVault.deploy(
    input.stakeTokenAddress,
    input._slashRate
  );
  await stakeVault.waitForDeployment();

  return stakeVault;
}
