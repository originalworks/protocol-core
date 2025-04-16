import { BigInt, log } from "@graphprotocol/graph-ts";

import {
  BlobRejectedEventId,
  recordBlobsStatuses,
  BlobProcessedEventId,
  recordHealthStatusValidatorData,
} from "./helpers";
import {
  Release,
  DisplayArtist,
  ProvedMessage,
  DisplayArtistName,
  ValidatorTxPerDay,
  BlobsRejectedPerDay,
  ValidatorTxPerMonth,
  BlobsProcessedPerDay,
  BlobsRejectedPerMonth,
  BlobsProcessedPerMonth,
  MessagesProcessedPerDay, SoundRecording,
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
    const messageRelease = message.release;
    const messageSoundRecordings = message.sound_recordings;

    const displayArtists: string[] = []
    for (let j = 0; j < messageRelease.display_artists.length; j++) {
      const displayArtist = new DisplayArtist(`${messageRelease.release_id.icpn}-${messageRelease.display_artists[j].artist_party_reference}`)
      displayArtist.artist_party_reference = messageRelease.display_artists[j].artist_party_reference;
      displayArtist.sequence_number = BigInt.fromI32(messageRelease.display_artists[j].sequence_number);
      displayArtist.display_artist_roles = messageRelease.display_artists[j].display_artist_roles;
      displayArtist.save();
      displayArtists.push(displayArtist.id);
    }

    const displayArtistNames: string[] = []
    for (let j = 0; j < messageRelease.display_artist_names.length; j++) {
      const displayArtistName = new DisplayArtistName(`${messageRelease.release_id.icpn}-${messageRelease.display_artist_names[j].display_artist_name}`)
      displayArtistName.applicable_territory_code = messageRelease.display_artist_names[j].applicable_territory_code;
      displayArtistName.language_and_script_type = messageRelease.display_artist_names[j].language_and_script_type;
      displayArtistName.display_artist_name = messageRelease.display_artist_names[j].display_artist_name;
      displayArtistName.save();
      displayArtistNames.push(displayArtistName.id);
    }

    const release = new Release(messageRelease.release_id.icpn);
    release.title_text = messageRelease.title_text;
    release.subtitle = messageRelease.subtitle;
    release.display_title_text = messageRelease.display_title_text;
    release.release_types = messageRelease.release_types;
    release.display_artists = displayArtists;
    release.display_artist_names = displayArtistNames;
    release.grid = messageRelease.release_id.grid;
    release.icpn = messageRelease.release_id.icpn;
    release.proprietary_ids = messageRelease.release_id.proprietary_ids;
    release.save();

    const soundRecordings: string[] = []
    for (let j = 0; j < messageSoundRecordings.length; j++) {
      const soundRecording = new SoundRecording(messageSoundRecordings[j].track_release_id.icpn)
      soundRecording.display_title = messageSoundRecordings[j].display_title;
      soundRecording.subtitle = messageSoundRecordings[j].subtitle;
      soundRecording.display_title_text = messageSoundRecordings[j].display_title_text;
      soundRecording.save();
      soundRecordings.push(soundRecording.id);
    }

    const provedMessage = new ProvedMessage(
      `${event.transaction.hash.toHex()}-${i}`
    );
    provedMessage.message_id = message.release.release_id.icpn.toString();
    provedMessage.timestamp = event.block.timestamp;
    provedMessage.validator = event.transaction.from;
    provedMessage.release = release.id;
    provedMessage.soundRecordings = soundRecordings;
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
  handleValidatorData(event.block.timestamp);

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
  handleValidatorData(event.block.timestamp);

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

function handleValidatorData(validationTimestamp: BigInt): void {
  const date = new Date(BigInt.fromString(`${validationTimestamp.toI64()}000`).toI64());
  const id = `${date.getUTCMonth() + 1}-${(date.getUTCDate())}-${date.getUTCFullYear()}`;
  let validatorDay = ValidatorTxPerDay.load(id);

  if (validatorDay == null) {
    validatorDay = new ValidatorTxPerDay(id);
    validatorDay.amount = BigInt.zero();
  }

  validatorDay.month = (date.getUTCMonth() + 1).toString();
  validatorDay.day = date.getUTCDate().toString();
  validatorDay.year = date.getUTCFullYear().toString();
  validatorDay.amount = validatorDay.amount.plus(BigInt.fromI32(1));

  validatorDay.save();

  const idMonth = `${date.getUTCMonth() + 1}-1-${date.getUTCFullYear()}`;
  let validatorMonth = ValidatorTxPerMonth.load(idMonth);

  if (validatorMonth == null) {
    validatorMonth = new ValidatorTxPerMonth(idMonth);
    validatorMonth.amount = BigInt.zero();
  }

  validatorMonth.month = (date.getUTCMonth() + 1).toString();
  validatorMonth.year = date.getUTCFullYear().toString();
  validatorMonth.amount = validatorMonth.amount.plus(BigInt.fromI32(1));

  validatorMonth.save();
}
