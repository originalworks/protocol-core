import { MessageDigested } from "./types/DdexSequencer/DdexSequencer";
import { DdexMessage, Isrc, Release } from "./types/schema";

export function handleMessageDigested(event: MessageDigested): void {
  const ddexMessage = new DdexMessage(event.transaction.hash.toHex());
  ddexMessage.isrc = event.params.data.isrc;
  ddexMessage.releaseId = event.params.data.releaseId;
  ddexMessage.save();

  let isrc = Isrc.load(event.params.data.isrc);
  if (isrc === null) {
    isrc = new Isrc(`${event.params.data.isrc}`);
  }
  isrc.lastUpdate = event.block.timestamp;
  isrc.save();

  let release = Release.load(event.params.data.releaseId);
  if (release === null) {
    release = new Release(`${event.params.data.releaseId}`);
    release.isrcs = [];
    release.save();
  }
  let alreadyIncluded = false;
  for (let i = 0; i < release.isrcs.length; i++) {
    if (release.isrcs[i] == isrc.id) {
      alreadyIncluded = true;
    }
  }
  if (!alreadyIncluded) {
    let releaseIsrcs = release.isrcs;
    releaseIsrcs.push(isrc.id);
    release.isrcs = releaseIsrcs;
    release.save();
  }
}
