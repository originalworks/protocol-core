import { Whitelist } from "../../../../typechain-types/contracts/Whitelist/Whitelist";
import { ethers } from "hardhat";
import { Signer } from "ethers";

export async function deployWhitelist(
  deployer: Signer,
  initiallyWhitelisted: string[]
): Promise<Whitelist> {
  const Whitelist = await ethers.getContractFactory("Whitelist");
  const whitelist = await Whitelist.deploy(deployer);
  await whitelist.waitForDeployment();

  for (let i = 0; i < initiallyWhitelisted.length; i++) {
    const address = initiallyWhitelisted[i];
    await whitelist.addToWhitelist(address);
  }

  return whitelist;
}
