import { BigInt } from "@graphprotocol/graph-ts";

import { Blobs } from "./types/schema";
import { NewBlobSubmitted } from "./types/DdexSequencer/DdexSequencer";

const BlobsSubmittedEventId = "NewBlobSubmitted";

export function handleNewBlobSubmitted(event: NewBlobSubmitted): void {
  let blobs = Blobs.load(BlobsSubmittedEventId);

  if (blobs == null) {
    blobs = new Blobs(BlobsSubmittedEventId);
    blobs.amount = BigInt.zero();
  }

  blobs.amount = blobs.amount.plus(BigInt.fromI32(1));

  blobs.save();
}
