import { BigInt, log } from "@graphprotocol/graph-ts";

import {
  ProvedMessage,
  BlobsRejectedPerDay,
  BlobsProcessedPerDay,
  MessagesProcessedPerDay
} from "./types/schema";
import { recordBlobsStatuses } from "./helpers";
import { AssetMetadataTemplate } from "./types/templates";
import { BlobProcessed, BlobRejected } from "./types/DdexEmitter/DdexEmitter";

// Hardcoded IPFS folder with raw JSON files, for demonstration.
// Each file is something like <HASH>/jsons/1.json, <HASH>/jsons/2.json, etc.
const ipfsFolderCID = "bafybeid5l3yfspemqdmwhw4sal5r4fd2odtsuvuc2jt4teq42v3a4lloy4/jsons";
// Just an example: we create a data source up to 70 files.
const maxFiles = 70

const BlobProcessedEventId = "processed";
const BlobRejectedEventId = "rejected";

export function handleBlobProcessed(event: BlobProcessed): void {
  const proverPublicOutputs = event.params.proverPublicOutputs;
  const messages = proverPublicOutputs.messages;

  for (let i = 0; i < messages.length; i++) {
    const message = messages[i];

    const provedMessage = new ProvedMessage(
      `${event.transaction.hash.toHex()}-${i}`
    );
    provedMessage.message_id = message.release.release_id.icpn.toString();
    provedMessage.timestamp = event.block.timestamp;
    provedMessage.validator = event.transaction.from;
    provedMessage.save();

    let date = new Date(BigInt.fromString(`${event.block.timestamp.toI64()}000`).toI64());
    let id = `${date.getUTCMonth() + 1}-${(date.getUTCDate())}-${date.getUTCFullYear()}`;
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

  recordBlobsStatuses(BlobProcessedEventId, event.block.timestamp);

  let date = new Date(BigInt.fromString(`${event.block.timestamp.toI64()}000`).toI64());
  let id = `${date.getUTCMonth() + 1}-${(date.getUTCDate())}-${date.getUTCFullYear()}`;
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

  log.info("BlobProcessed CID: {}", [event.params.cid])
  // Now spin up sub‑dataSources for each JSON file in IPFS
  // for (let i = 1; i <= maxFiles; i++) {
  //   let ipfsPath = ipfsFolderCID + "/" + i.toString() + ".json";
  //   log.info("Creating IPFS data source for file: {}", [ipfsPath]);
  //
  //   // This will invoke the handleAssetMetadata() in "assetMetadata.ts"
  //   AssetMetadataTemplate.create(ipfsPath);
  // }
}

export function handleBlobRejected(event: BlobRejected): void {
  recordBlobsStatuses(BlobRejectedEventId, event.block.timestamp);

  let date = new Date(BigInt.fromString(`${event.block.timestamp.toI64()}000`).toI64());
  let id = `${date.getUTCMonth() + 1}-${(date.getUTCDate())}-${date.getUTCFullYear()}`;
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
}