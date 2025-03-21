import { BigInt, log } from "@graphprotocol/graph-ts";

import {
  BlobRejectedEventId,
  recordBlobsStatuses,
  BlobProcessedEventId,
  recordHealthStatusValidatorData,
} from "./helpers";
import {
  ProvedMessage,
  BlobsRejectedPerDay,
  BlobsProcessedPerDay,
  BlobsRejectedPerMonth,
  BlobsProcessedPerMonth,
  MessagesProcessedPerDay,
} from './types/schema';
import { BlobProcessed, BlobRejected } from "./types/DdexEmitter/DdexEmitter";
import { AssetMetadataTemplate, BlobMetadataTemplate } from "./types/templates";

// Just an example: we create a data source up to 70 files.
const maxFiles = 70;

export function handleBlobProcessed(event: BlobProcessed): void {
  const proverPublicOutputs = event.params.proverPublicOutputs;
  const messages = proverPublicOutputs.messages;

  const date = new Date(BigInt.fromString(`${event.block.timestamp.toI64()}000`).toI64());
  const id = `${date.getUTCMonth() + 1}-${(date.getUTCDate())}-${date.getUTCFullYear()}`;

  for (let i = 0; i < messages.length; i++) {
    const message = messages[i];

    const provedMessage = new ProvedMessage(
      `${event.transaction.hash.toHex()}-${i}`
    );
    provedMessage.message_id = message.release.release_id.icpn.toString();
    provedMessage.timestamp = event.block.timestamp;
    provedMessage.validator = event.transaction.from;
    provedMessage.save();

    let messagesProcessed = MessagesProcessedPerDay.load(id);

    if (messagesProcessed == null) {
      messagesProcessed = new MessagesProcessedPerDay(id);
      messagesProcessed.amount = BigInt.zero();
    }

    messagesProcessed.month = (date.getUTCMonth() + 1).toString();
    messagesProcessed.day = date.getUTCDate().toString();
    messagesProcessed.year = date.getUTCFullYear().toString();
    messagesProcessed.amount = messagesProcessed.amount.plus(BigInt.fromI32(1));

    messagesProcessed.save();
  }

  recordBlobsStatuses(BlobProcessedEventId, event.block.timestamp, event.transaction.hash);
  recordHealthStatusValidatorData(event.block.timestamp, event.transaction.hash);

  let blobsProcessed = BlobsProcessedPerDay.load(id);

  if (blobsProcessed == null) {
    blobsProcessed = new BlobsProcessedPerDay(id);
    blobsProcessed.amount = BigInt.zero();
  }

  blobsProcessed.month = (date.getUTCMonth() + 1).toString();
  blobsProcessed.day = date.getUTCDate().toString();
  blobsProcessed.year = date.getUTCFullYear().toString();
  blobsProcessed.amount = blobsProcessed.amount.plus(BigInt.fromI32(1));

  blobsProcessed.save();

  const idPerMonth = `${date.getUTCMonth() + 1}-1-${date.getUTCFullYear()}`;
  let processedPerMonth = BlobsProcessedPerMonth.load(idPerMonth);

  if (processedPerMonth == null) {
    processedPerMonth = new BlobsProcessedPerMonth(idPerMonth);
    processedPerMonth.amount = BigInt.zero();
  }

  processedPerMonth.month = (date.getUTCMonth() + 1).toString();
  processedPerMonth.year = date.getUTCFullYear().toString();
  processedPerMonth.amount = processedPerMonth.amount.plus(BigInt.fromI32(1));

  processedPerMonth.save();

  log.info("BlobProcessed CID: {}", [event.params.cid]);

  const blobMetadataIPFSPath = event.params.cid + "/blob/metadata.json";
  BlobMetadataTemplate.create(blobMetadataIPFSPath);

  // Now spin up sub‑dataSources for each JSON file in IPFS
  for (let i = 1; i <= maxFiles; i++) {
    const ipfsPath = event.params.cid + "/json/" + i.toString() + ".json";

    // This will invoke the handleAssetMetadata() in "assetMetadata.ts"
    AssetMetadataTemplate.create(ipfsPath);
  }
}

export function handleBlobRejected(event: BlobRejected): void {
  recordBlobsStatuses(BlobRejectedEventId, event.block.timestamp, event.transaction.hash);
  recordHealthStatusValidatorData(event.block.timestamp, event.transaction.hash);

  const date = new Date(BigInt.fromString(`${event.block.timestamp.toI64()}000`).toI64());
  const id = `${date.getUTCMonth() + 1}-${(date.getUTCDate())}-${date.getUTCFullYear()}`;
  let blobsRejected = BlobsRejectedPerDay.load(id);

  if (blobsRejected == null) {
    blobsRejected = new BlobsRejectedPerDay(id);
    blobsRejected.amount = BigInt.zero();
  }

  blobsRejected.month = (date.getUTCMonth() + 1).toString();
  blobsRejected.day = date.getUTCDate().toString();
  blobsRejected.year = date.getUTCFullYear().toString();
  blobsRejected.amount = blobsRejected.amount.plus(BigInt.fromI32(1));

  blobsRejected.save();

  const idPerMonth = `${date.getUTCMonth() + 1}-1-${date.getUTCFullYear()}`;
  let rejectedPerMonth = BlobsRejectedPerMonth.load(idPerMonth);

  if (rejectedPerMonth == null) {
    rejectedPerMonth = new BlobsRejectedPerMonth(idPerMonth);
    rejectedPerMonth.amount = BigInt.zero();
  }

  rejectedPerMonth.month = (date.getUTCMonth() + 1).toString();
  rejectedPerMonth.year = date.getUTCFullYear().toString();
  rejectedPerMonth.amount = rejectedPerMonth.amount.plus(BigInt.fromI32(1));

  rejectedPerMonth.save();
}