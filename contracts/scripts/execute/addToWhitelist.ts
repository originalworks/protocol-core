import { ethers } from "hardhat";

const WHITELIST_ADDRESS = "0x5d39340D198Bb7CEF1c7f3bf6c8a373716dAD5BD";
const ADDRESS_TO_ADD = "0x778df9D70ED197d433f7a8011ca5314A52488038";

async function main() {
  const whitelist = await ethers.getContractAt("Whitelist", WHITELIST_ADDRESS);

  const tx = await whitelist.addToWhitelist(ADDRESS_TO_ADD);
  await tx.wait();
}

main();
