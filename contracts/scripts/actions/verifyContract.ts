/* eslint-disable @typescript-eslint/no-explicit-any */
import type { HardhatRuntimeEnvironment } from "hardhat/types";

export const verifyContract = async (
  deployedContractAddress: string,
  hre: HardhatRuntimeEnvironment,
  args?: any[]
) => {
  console.log("Verifying contract at", deployedContractAddress);
  await sleepFor(25000);
  try {
    await hre.run("verify:verify", {
      address: deployedContractAddress,
      constructorArguments: args,
    });
  } catch (error_) {
    let errorMessage = "Failed to verify!";
    if (error_ instanceof Error) {
      if (error_.message.toLowerCase().includes("already verified"))
        console.log("Already verified!");
      errorMessage = error_.message;
    }
    console.log(errorMessage);
  }
};

async function sleepFor(ms: number) {
  return new Promise((resolve) => {
    setTimeout(() => {
      resolve(true);
    }, ms);
  });
}
