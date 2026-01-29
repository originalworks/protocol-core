// import { BigInt } from "@graphprotocol/graph-ts";
import { NewBlob, Submitter, TrackProcessedWithSubmitter } from "./types/schema";

import {
  DdexSequencer,
  NewBlobSubmitted,
  SubmitNewBlobCall,
} from "./types/DdexSequencer/DdexSequencer";
import { BlobProcessed } from "./types/DdexEmitter/DdexEmitter";

export function handleNewBlobSubmitted(call: SubmitNewBlobCall): void {
  const id = call.inputs._blobSha2.toHex().toString();
  const from = call.transaction.from;

  let submitter = Submitter.load(from);
  if (submitter == null) {
    submitter = new Submitter(from);
    submitter.address = from;
    submitter.save();
  }

  let newBlob = NewBlob.load(id);
  if (newBlob == null) {
    newBlob = new NewBlob(id);
    newBlob.blobSubmitters = [];
  }

  if (newBlob.blobSubmitters.length > 0) {
    const submitters = newBlob.blobSubmitters;
    const index = submitters.indexOf(from);
    if (index < 0) {
      newBlob.blobSubmitters = submitters.concat([from]);
    }
  } else {
    newBlob.blobSubmitters = [from];
  }

  newBlob.save();
}

// export function handleNewBlobSubmitted(event: NewBlobSubmitted): void {
//   const id = event.params.commitment.toHex().toString();

//   let newBlob = NewBlob.load(id);

//   if (newBlob == null) {
//     newBlob = new NewBlob(id);
//     newBlob.msgSender = event.transaction.from;
//     newBlob.commitment = event.params.commitment;
//     newBlob.save();
//   } else {
//     newBlob.msgSender = event.transaction.from;
//     newBlob.commitment = event.params.commitment;
//     newBlob.save();
//   }
// }

export function handleBlobProcessed(event: BlobProcessed): void {
  const blobSha2 = event.params.proverPublicOutputs.digest;

  let submittedBlob = NewBlob.load(blobSha2.toHex().toString());

  const messages = event.params.proverPublicOutputs.messages;
  for (let i = 0; i < messages.length; i++) {
    const message = messages[i];
    const recordings = message.sound_recordings;
    for (let j = 0; j < recordings.length; j++) {
      const editions = message.sound_recordings[j].sound_recording_editions;
      for (let k = 0; k < editions.length; k++) {
        const isrc = editions[k].isrc;

        if (submittedBlob == null) {
          let trackProcessed = TrackProcessedWithSubmitter.load(isrc);
          if (trackProcessed == null) {
            trackProcessed = new TrackProcessedWithSubmitter(isrc);
            trackProcessed.isrc = isrc;
            trackProcessed.submitters = [blobSha2];
            trackProcessed.save();
          } else {
            trackProcessed.submitters = [blobSha2];
            trackProcessed.save();
          }
        } else {
          let trackProcessed = TrackProcessedWithSubmitter.load(isrc);
          if (trackProcessed == null) {
            trackProcessed = new TrackProcessedWithSubmitter(isrc);
            trackProcessed.isrc = isrc;
            trackProcessed.submitters = submittedBlob.blobSubmitters;
            trackProcessed.save();
          } else {
            trackProcessed.submitters = submittedBlob.blobSubmitters;
            trackProcessed.save();
          }
        }
      }
    }
  }

  // if (submittedBlob) {
  //   const id = event.transaction.hash.toHex().toString();
  //   const blobProcessedWithSubmitter = new BlobProcessedWithSubmitter(id);
  //   blobProcessedWithSubmitter.firstProvedMessageReleaseText =
  //     event.params.proverPublicOutputs.messages[0].release.title_text;
  //   blobProcessedWithSubmitter.submitter = submittedBlob.blobSubmitter;
  //   blobProcessedWithSubmitter.save();
  // } else {
  //   const id = event.transaction.hash.toHex().toString();
  //   const blobProcessedWithSubmitter = new BlobProcessedWithSubmitter(id);
  //   blobProcessedWithSubmitter.firstProvedMessageReleaseText =
  //     "here submitter is a blobsha2";
  //   blobProcessedWithSubmitter.submitter = blobSha2;
  //   blobProcessedWithSubmitter.save();
  // }
  //
  //   blobProcessedWithSubmitter.submitter =
  // if (provedMessageSubmitter == null) {
  // provedMessageSubmitter.blobDigest = event.params.proverPublicOutputs.digest;
  // provedMessageSubmitter.submitter = event.params.proverPublicOutputs.digest;

  // let sequencer = DdexSequencer.bind(
  //   Address.fromString("0x75AbeCf07C26368F0f4AA0b0d3637A732E25467e")
  // );

  // let value = sequencer.try_getQueueHeadDetails();
  // if (value) {
  //   provedMessageSubmitter.submitter = value.value.value0.proposer;
  // }

  // provedMessageSubmitter.save();
  //   }
}
