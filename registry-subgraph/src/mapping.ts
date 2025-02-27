import { log } from "@graphprotocol/graph-ts";

import { ProvedMessage } from "./types/schema";
import { BlobProcessed } from "./types/DdexEmitter/DdexEmitter";

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
}