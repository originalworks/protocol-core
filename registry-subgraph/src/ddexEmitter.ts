import { BigInt, log } from "@graphprotocol/graph-ts";

import {
  Cid,
  Track,
  Artist,
  Release,
  ProvedMessage,
  TracksAddedPerDay,
  ArtistsAddedPerDay,
  TracksAddedPerMonth,
  ArtistsAddedPerMonth,
} from "./types/schema";
import { deduplicateStringList } from "./helpers";
import { BlobProcessed } from "./types/DdexEmitter/DdexEmitter";
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

    const displayArtistNamesList = displayArtistNames.map<string>((artist) => artist.display_artist_name.toString());

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
              if (displayArtistNamesList.length > 0) {
                track.artist_names = deduplicateStringList(displayArtistNamesList.join(', '));
              } else {
                track.artist_names = '';
              }
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
                if (track.artist_names == null) {
                  if (displayArtistNamesList.length > 0) {
                    track.artist_names = deduplicateStringList(displayArtistNamesList.join(", "));
                  } else {
                    track.artist_names = '';
                  }
                } else {
                  if (displayArtistNamesList.length > 0) {
                    if (track.artist_names.length > 0) {
                      track.artist_names = deduplicateStringList(track.artist_names + ", " + displayArtistNamesList.join(", "));
                    } else {
                      track.artist_names = deduplicateStringList(displayArtistNamesList.join(", "));
                    }
                  }
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

  log.info("BlobProcessed CID: {}", [event.params.cid]);

  const blobMetadataIPFSPath = event.params.cid + "/blob/metadata.json";
  BlobMetadataTemplate.create(blobMetadataIPFSPath);
}
