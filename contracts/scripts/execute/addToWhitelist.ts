import { ethers } from "hardhat";

const WHITELIST_ADDRESS = '';
const ADDRESS_TO_ADD = '';

async function main() {
  const whitelist = await ethers.getContractAt("Whitelist", WHITELIST_ADDRESS);

  const tx = await whitelist.addToWhitelist(ADDRESS_TO_ADD);
  await tx.wait();
}

main();
