import { log } from "@graphprotocol/graph-ts";

import { ProvedMessage } from "./types/schema";
import { BlobProcessed } from "./types/DdexEmitter/DdexEmitter";
import { AssetMetadataTemplate } from './types/templates';

// Hardcoded ipfsHash to test getting info from files
const ipfsHash = "bafybeid5l3yfspemqdmwhw4sal5r4fd2odtsuvuc2jt4teq42v3a4lloy4/jsons";
// Hardcoded maximum files amount per folder
const maxFiles = 70

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

    log.info("Processed message subtitle: {}, icpn: {}, title_text: {}", [
      message.release.subtitle.toString(),
      message.release.release_id.icpn.toString(),
      message.release.title_text.toString(),
    ])
  }

  for (let i = 0; i < maxFiles; i++) {
    log.info("Processed i: {}", [i.toString()]);
    AssetMetadataTemplate.create(ipfsHash + `/${i + 1}.json`);
  }
}