// import { BigInt } from "@graphprotocol/graph-ts";
import { NewBlob, BlobProcessedWithSubmitter } from "./types/schema";

import {
  DdexSequencer,
  NewBlobSubmitted,
  SubmitNewBlobCall,
} from "./types/DdexSequencer/DdexSequencer";
import { BlobProcessed } from "./types/DdexEmitter/DdexEmitter";
import { Address } from "@graphprotocol/graph-ts";

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
  if (submittedBlob) {
    const id = event.transaction.hash.toHex().toString();
    const blobProcessedWithSubmitter = new BlobProcessedWithSubmitter(id);
    blobProcessedWithSubmitter.firstProvedMessageReleaseText =
      event.params.proverPublicOutputs.messages[0].release.title_text;
    blobProcessedWithSubmitter.submitter = submittedBlob.blobSubmitter;
    blobProcessedWithSubmitter.save();
  } else {
    const id = event.transaction.hash.toHex().toString();
    const blobProcessedWithSubmitter = new BlobProcessedWithSubmitter(id);
    blobProcessedWithSubmitter.firstProvedMessageReleaseText =
      "here submitter is a blobsha2";
    blobProcessedWithSubmitter.submitter = blobSha2;
    blobProcessedWithSubmitter.save();
  }

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

export function handleNewBlobSubmitted(call: SubmitNewBlobCall): void {
  const id = call.inputs._blobSha2.toHex().toString();
  const newBlob = new NewBlob(id);
  newBlob.blobSubmitter = call.transaction.from;

  newBlob.save();
}
