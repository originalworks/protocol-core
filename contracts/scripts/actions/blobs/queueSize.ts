import { ethers } from "hardhat";

const DDEX_SEQUENCER_ADDRESS = '';

async function main() {
  const sequencer = await ethers.getContractAt(
    "DdexSequencer",
    DDEX_SEQUENCER_ADDRESS
  );

  let cursor = await sequencer.blobQueueHead();
  let size = 0;
  const visited = new Set<string>();

  while (cursor !== ethers.ZeroHash) {
    const key = cursor.toLowerCase();
    if (visited.has(key)) {
      throw new Error(`Cycle detected while traversing the queue at ${cursor}`);
    }
    visited.add(key);

    size += 1;
    const blob = await sequencer.blobs(cursor);
    cursor = blob.nextBlob;
  }

  console.log(`Blobs in queue: ${size}`);
}

void main()
