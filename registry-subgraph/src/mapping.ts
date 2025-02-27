import { log } from "@graphprotocol/graph-ts";
import { ProvedMessage } from "./types/schema";
import { BlobProcessed } from "./types/DdexEmitter/DdexEmitter";
import { AssetMetadataTemplate } from "./types/templates";

// Hardcoded IPFS folder with raw JSON files, for demonstration.
// Each file is something like <HASH>/jsons/1.json, <HASH>/jsons/2.json, etc.
const ipfsFolderCID = "bafybeid5l3yfspemqdmwhw4sal5r4fd2odtsuvuc2jt4teq42v3a4lloy4/jsons";

// Just an example: we create a data source up to 70 files.
const maxFiles = 70;

export function handleBlobProcessed(event: BlobProcessed): void {
  const proverPublicOutputs = event.params.proverPublicOutputs;
  const messages = proverPublicOutputs.messages;

  const isValid = proverPublicOutputs.is_valid;
  log.info("isValid: {}", [isValid.toString()]);

  // Example: storing each message as an entity
  for (let i = 0; i < messages.length; i++) {
    const message = messages[i];
    const provedMessage = new ProvedMessage(
      `${event.transaction.hash.toHex()}-${i}`
    );

    provedMessage.message_id = message.release.release_id.icpn.toString();
    provedMessage.timestamp = event.block.timestamp;
    provedMessage.validator = event.transaction.from;
    provedMessage.save();

    log.info("Processed message. subtitle: {}, icpn: {}, title_text: {}", [
      message.release.subtitle.toString(),
      message.release.release_id.icpn.toString(),
      message.release.title_text.toString(),
    ]);
  }

  // Now spin up sub‑dataSources for each JSON file in IPFS
  for (let i = 1; i <= maxFiles; i++) {
    let ipfsPath = ipfsFolderCID + "/" + i.toString() + ".json";
    log.info("Creating IPFS data source for file: {}", [ipfsPath]);

    // This will invoke the handleAssetMetadata() in 'assetMetadata.ts'
    AssetMetadataTemplate.create(ipfsPath);
  }
}
