import { log, BigInt, ethereum } from '@graphprotocol/graph-ts';

import { BlobCall, Blobs } from './types/schema';
import { NewBlobSubmitted, SubmitNewBlobCall } from './types/DdexSequencer/DdexSequencer';
import { json } from '@graphprotocol/graph-ts/common/json';

const BlobsSubmittedCallId = "submitNewBlob";
const BlobsSubmittedEventId = "NewBlobSubmitted";

export function handleSubmitNewBlob(call: SubmitNewBlobCall): void {
  log.info("Received submitNewBlob call", []);

  let blobs = Blobs.load(BlobsSubmittedCallId);

  if (blobs == null) {
    blobs = new Blobs(BlobsSubmittedCallId);
    blobs.amount = BigInt.zero();
  }

  blobs.amount = blobs.amount.plus(BigInt.fromI32(1));

  blobs.save();
}

export function handleNewBlobSubmitted(event: NewBlobSubmitted): void {
  log.info("Received NewBlobSubmitted event", []);

  let blobs = Blobs.load(BlobsSubmittedEventId);

  if (blobs == null) {
    blobs = new Blobs(BlobsSubmittedEventId);
    blobs.amount = BigInt.zero();
  }

  blobs.amount = blobs.amount.plus(BigInt.fromI32(1));

  blobs.save();
}
