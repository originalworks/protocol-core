import { BigInt, log } from "@graphprotocol/graph-ts";

import {
  BlobRejectedEventId,
  recordBlobsStatuses,
  BlobProcessedEventId,
  recordHealthStatusValidatorData,
} from "./helpers";
import {
  Cid,
  Track,
  Artist,
  Release,
  ProvedMessage,
  TracksAddedPerDay,
  ValidatorTxPerDay,
  ArtistsAddedPerDay,
  BlobsRejectedPerDay,
  TracksAddedPerMonth,
  ValidatorTxPerMonth,
  ArtistsAddedPerMonth,
  BlobsProcessedPerDay,
  BlobsRejectedPerMonth,
  BlobsProcessedPerMonth,
  MessagesProcessedPerDay,
} from "./types/schema";
import { BlobProcessed, BlobRejected } from "./types/DdexEmitter/DdexEmitter";
import { AssetMetadataTemplate, BlobMetadataTemplate } from "./types/templates";

export function handleBlobProcessed(event: BlobProcessed): void {
  const proverPublicOutputs = event.params.proverPublicOutputs;
  const messages = proverPublicOutputs.messages;

  const date = new Date(BigInt.fromString(`${event.block.timestamp.toI64()}000`).toI64());
  const id = `${date.getUTCMonth() + 1}-${(date.getUTCDate())}-${date.getUTCFullYear()}`;
  const idPerMonth = `${date.getUTCMonth() + 1}-1-${date.getUTCFullYear()}`;

  for (let i = 0; i < messages.length; i++) {
    const message = messages[i];
    const mRelease = message.release;
    const image = event.params.cid + "/images/" + i.toString() + ".avif";

    const provedMessage = new ProvedMessage(
      `${event.transaction.hash.toHex()}-${i}`
    );
    provedMessage.message_id = mRelease.release_id.icpn.toString();
    provedMessage.timestamp = event.block.timestamp;
    provedMessage.validator = event.transaction.from;
    provedMessage.cid = event.params.cid;
    provedMessage.save();

    let cid = Cid.load(event.params.cid + "/json/" + i.toString() + ".json");
    if (cid == null) {
      cid = new Cid(event.params.cid + "/json/" + i.toString() + ".json");
    }
    cid.timestamp = event.block.timestamp;
    cid.save();

    AssetMetadataTemplate.create(cid.id);

    let release = Release.load(mRelease.release_id.icpn.toString());
    if (release == null) {
      release = new Release(mRelease.release_id.icpn.toString());
    }
    release.icpn = mRelease.release_id.icpn.toString();
    release.title_text = mRelease.title_text.toString();
    release.subtitle = mRelease.subtitle.toString();
    release.display_title_text = mRelease.display_title_text.toString();
    release.release_types = mRelease.release_types.map<string>((type) => type.toString());
    release.display_artist_names = mRelease.display_artist_names.map<string>((artist) => artist.display_artist_name.toString());
    const recordings: string[] = [];
    for (let j = 0; j < message.sound_recordings.length; j++) {
      const editions = message.sound_recordings[j].sound_recording_editions;
      for (let k = 0; k < editions.length; k++) {
        recordings.push(editions[k].isrc);
      }
    }
    release.sound_recordings = recordings;
    release.image = image;
    release.imageMetadata = cid.id;
    release.timestamp = event.block.timestamp;
    release.save();

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

    const displayArtistNames = mRelease.display_artist_names;
    if (displayArtistNames.length > 0) {
      for (let j = 0; j < displayArtistNames.length; j++) {
        const artistName = displayArtistNames[j].display_artist_name;
        if (artistName) {
          const namesArray = artistName.split(" feat. ").join(" , ").split(" & ").join(" , ").split(" , ");
          if (namesArray.length > 0) {
            for (let k = 0; k < namesArray.length; k++) {
              const name = namesArray[k];
              if (name) {
                let artist = Artist.load(name);
                if (artist == null) {
                  artist = new Artist(name);
                  artist.name = name;
                  artist.timestamp = event.block.timestamp;
                  artist.cid = event.params.cid;
                  artist.save();

                  let artistsPerDay = ArtistsAddedPerDay.load(id);
                  if (artistsPerDay == null) {
                    artistsPerDay = new ArtistsAddedPerDay(id);
                    artistsPerDay.amount = BigInt.zero();
                  }
                  artistsPerDay.month = (date.getUTCMonth() + 1).toString();
                  artistsPerDay.day = date.getUTCDate().toString();
                  artistsPerDay.year = date.getUTCFullYear().toString();
                  artistsPerDay.amount = artistsPerDay.amount.plus(BigInt.fromI32(1));
                  artistsPerDay.save();

                  let artistsPerMonth = ArtistsAddedPerMonth.load(idPerMonth);
                  if (artistsPerMonth == null) {
                    artistsPerMonth = new ArtistsAddedPerMonth(idPerMonth);
                    artistsPerMonth.amount = BigInt.zero();
                  }
                  artistsPerMonth.month = (date.getUTCMonth() + 1).toString();
                  artistsPerMonth.year = date.getUTCFullYear().toString();
                  artistsPerMonth.amount = artistsPerMonth.amount.plus(BigInt.fromI32(1));
                  artistsPerMonth.save();
                }
              }
            }
          }
        }
      }
    }

    const soundRecordings = message.sound_recordings;
    if (soundRecordings.length > 0) {
      for (let j = 0; j < soundRecordings.length; j++) {
        const soundRecordingEditions = soundRecordings[j].sound_recording_editions;
        for (let k = 0; k < soundRecordingEditions.length; k++) {
          const isrc = soundRecordingEditions[k].isrc;
          const pLine = soundRecordings[j].sound_recording_editions[0].p_lines[0]
          if (isrc) {
            let track = Track.load(isrc);
            if (track == null) {
              track = new Track(isrc);
              track.isrc = isrc;
              track.cids = [cid.id];
              track.display_title = soundRecordings[j].display_title;
              track.subtitle = soundRecordings[j].subtitle;
              track.display_title_text = soundRecordings[j].display_title_text;
              track.label = pLine.p_line_text.replace(pLine.year.toString(), "").trim();
              track.image = image;
              track.imageMetadata = cid.id;
              track.releases = [release.id];
              track.timestamp = event.block.timestamp;
              track.save();

              let tracksPerDay = TracksAddedPerDay.load(id);
              if (tracksPerDay == null) {
                tracksPerDay = new TracksAddedPerDay(id);
                tracksPerDay.amount = BigInt.zero();
              }
              tracksPerDay.month = (date.getUTCMonth() + 1).toString();
              tracksPerDay.day = date.getUTCDate().toString();
              tracksPerDay.year = date.getUTCFullYear().toString();
              tracksPerDay.amount = tracksPerDay.amount.plus(BigInt.fromI32(1));
              tracksPerDay.save();

              let tracksPerMonth = TracksAddedPerMonth.load(idPerMonth);
              if (tracksPerMonth == null) {
                tracksPerMonth = new TracksAddedPerMonth(idPerMonth);
                tracksPerMonth.amount = BigInt.zero();
              }
              tracksPerMonth.month = (date.getUTCMonth() + 1).toString();
              tracksPerMonth.year = date.getUTCFullYear().toString();
              tracksPerMonth.amount = tracksPerMonth.amount.plus(BigInt.fromI32(1));
              tracksPerMonth.save();
            } else {
              if (event.block.timestamp > track.timestamp) {
                track.isrc = isrc;
                if (track.cids == null) {
                  track.cids = [cid.id];
                } else {
                  track.cids = [cid.id].concat(track.cids!);
                }
                track.display_title = soundRecordings[j].display_title;
                track.subtitle = soundRecordings[j].subtitle;
                track.display_title_text = soundRecordings[j].display_title_text;
                track.label = pLine.p_line_text.replace(pLine.year.toString(), "").trim();
                track.image = image;
                track.imageMetadata = cid.id;
                if (track.releases == null) {
                  track.releases = [release.id];
                } else {
                  track.releases = [release.id].concat(track.releases!);
                }
                track.timestamp = event.block.timestamp;
                track.save();
              }
            }
          }
        }
      }
    }
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
