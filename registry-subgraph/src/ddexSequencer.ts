import { BigInt } from "@graphprotocol/graph-ts";

import {
  recordBlobsStatuses,
  BlobsSubmittedEventId,
  initializeHealthStatus,
  initializeBlobsStatuses,
  recordHealthStatusBatchData,
  recordHealthStatusMoveQueueData,
} from './helpers';
import { BlobsSubmittedPerDay, BlobsSubmittedPerMonth } from './types/schema';
import { Initialized, NewBlobSubmitted } from './types/DdexSequencer/DdexSequencer';

export function handleInitialized(event: Initialized): void {
  initializeBlobsStatuses();
  initializeHealthStatus();
}

export function handleNewBlobSubmitted(event: NewBlobSubmitted): void {
  recordBlobsStatuses(BlobsSubmittedEventId, event.block.timestamp, event.transaction.hash);
  recordHealthStatusBatchData(event.block.timestamp, event.transaction.hash);

  const date = new Date(BigInt.fromString(`${event.block.timestamp.toI64()}000`).toI64());
  const id = `${date.getUTCMonth() + 1}-${(date.getUTCDate())}-${date.getUTCFullYear()}`;
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

  const idPerMonth = `${date.getUTCMonth() + 1}-1-${date.getUTCFullYear()}`;
  let submittedPerMonth = BlobsSubmittedPerMonth.load(idPerMonth);

  if (submittedPerMonth == null) {
    submittedPerMonth = new BlobsSubmittedPerMonth(idPerMonth);
    submittedPerMonth.amount = BigInt.zero();
  }

  submittedPerMonth.month = (date.getUTCMonth() + 1).toString();
  submittedPerMonth.year = date.getUTCFullYear().toString();
  submittedPerMonth.amount = submittedPerMonth.amount.plus(BigInt.fromI32(1));

  submittedPerMonth.save();
}

export function handleMoveQueue(): void {
  recordHealthStatusMoveQueueData();
}
