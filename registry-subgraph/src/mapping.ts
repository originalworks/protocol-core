import { BlobProcessed } from "./types/DdexEmitter/DdexEmitter";
import { ProvedMessage } from "./types/schema";

export function handleBlobProcessed(event: BlobProcessed): void {
  for (let i = 0; i < event.params.proverPublicOutputs.messages.length; i++) {
    const message = event.params.proverPublicOutputs.messages[i];

    const provedMessage = new ProvedMessage(
      `${event.transaction.hash.toHex()}-${i}`
    );
    provedMessage.message_id = message.release.release_id.toString();
    provedMessage.timestamp = event.block.timestamp;
    provedMessage.validator = event.transaction.from;
    provedMessage.save();
  }
}
