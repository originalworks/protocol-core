import { BigInt } from "@graphprotocol/graph-ts";

import { recordBlobsStatuses } from "./helpers";
import { BlobsSubmittedPerDay } from "./types/schema";
import { NewBlobSubmitted } from "./types/DdexSequencer/DdexSequencer";

const BlobsSubmittedEventId = "submitted";

export function handleNewBlobSubmitted(event: NewBlobSubmitted): void {
  recordBlobsStatuses(BlobsSubmittedEventId, event.block.timestamp);

  let date = new Date(BigInt.fromString(`${event.block.timestamp.toI64()}000`).toI64());
  let id = `${date.getUTCMonth() + 1}-${(date.getUTCDate())}-${date.getUTCFullYear()}`;
  let blobsSubmitted = BlobsSubmittedPerDay.load(id);

  if (blobsSubmitted == null) {
    blobsSubmitted = new BlobsSubmittedPerDay(id);
    blobsSubmitted.amount = BigInt.zero();
  }

  blobsSubmitted.month = (date.getUTCMonth() + 1).toString();
  blobsSubmitted.day = date.getUTCDate().toString();
  blobsSubmitted.year = date.getUTCFullYear().toString();
  blobsSubmitted.amount = blobsSubmitted.amount.plus(BigInt.fromI32(1));

  blobsSubmitted.save();
}
