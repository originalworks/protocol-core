import { StakeVault } from "../../../../typechain-types/contracts/StakeVault";
import { ethers } from "hardhat";
import { StakeVaultDeploymentInput } from "./StakeVault.types";
import { verifyContract } from "../../verifyContract";
import hre from "hardhat";

export async function deployStakeVault(
  input: StakeVaultDeploymentInput
): Promise<StakeVault> {
  const StakeVault = await ethers.getContractFactory("StakeVault");
  const stakeVault = await StakeVault.deploy(
    input.stakeTokenAddress,
    input._slashRate
  );
  await stakeVault.waitForDeployment();
  await verifyContract(await stakeVault.getAddress(), hre, [
    input.stakeTokenAddress,
    input._slashRate,
  ]);

  return stakeVault;
}
