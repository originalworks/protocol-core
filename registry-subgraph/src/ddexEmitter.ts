import { BigInt, log } from "@graphprotocol/graph-ts";

import { Blobs, MessagesProcessedPerDay, ProvedMessage } from './types/schema';
import { AssetMetadataTemplate } from "./types/templates";
import { BlobProcessed } from "./types/DdexEmitter/DdexEmitter";

// Hardcoded IPFS folder with raw JSON files, for demonstration.
// Each file is something like <HASH>/jsons/1.json, <HASH>/jsons/2.json, etc.
const ipfsFolderCID = "bafybeid5l3yfspemqdmwhw4sal5r4fd2odtsuvuc2jt4teq42v3a4lloy4/jsons";
// Just an example: we create a data source up to 70 files.
const maxFiles = 70

const BlobProcessedEventId = "BlobProcessed";

export function handleBlobProcessed(event: BlobProcessed): void {
  const proverPublicOutputs = event.params.proverPublicOutputs;
  const messages = proverPublicOutputs.messages;

  const isValid = proverPublicOutputs.is_valid;

  log.info("isValid: {}", [isValid.toString()])

  for (let i = 0; i < messages.length; i++) {
    const message = messages[i];

    const provedMessage = new ProvedMessage(
      `${event.transaction.hash.toHex()}-${i}`
    );
    provedMessage.message_id = message.release.release_id.icpn.toString();
    provedMessage.timestamp = event.block.timestamp;
    provedMessage.validator = event.transaction.from;
    provedMessage.save();

    log.info("Processed message, subtitle: {}, icpn: {}, title_text: {}", [
      message.release.subtitle.toString(),
      message.release.release_id.icpn.toString(),
      message.release.title_text.toString(),
    ])

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

  let blobs = Blobs.load(BlobProcessedEventId);

  if (blobs == null) {
    blobs = new Blobs(BlobProcessedEventId);
    blobs.amount = BigInt.zero();
  }

  log.info("handleBlobProcessed, before amount: {}", [blobs.amount.toString()]);
  blobs.amount = blobs.amount.plus(BigInt.fromI32(1));
  log.info("handleBlobProcessed, after amount: {}", [blobs.amount.toString()]);

  blobs.save();

  // Now spin up sub‑dataSources for each JSON file in IPFS
  // for (let i = 1; i <= maxFiles; i++) {
  //   let ipfsPath = ipfsFolderCID + "/" + i.toString() + ".json";
  //   log.info("Creating IPFS data source for file: {}", [ipfsPath]);
  //
  //   // This will invoke the handleAssetMetadata() in "assetMetadata.ts"
  //   AssetMetadataTemplate.create(ipfsPath);
  // }
}