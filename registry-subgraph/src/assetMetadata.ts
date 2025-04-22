import { BigInt, Bytes, dataSource, json, JSONValue, log } from '@graphprotocol/graph-ts';

import { AssetMetadata, DisplayArtist, Release } from './types/schema';
import { JSONValueKind } from '@graphprotocol/graph-ts/common/value';
import { getNumberIfExist, getValueIfExist } from './helpers';
import { TypedMap } from '@graphprotocol/graph-ts/common/collections';

export function handleAssetMetadata(content: Bytes): void {
  const cid = dataSource.stringParam();
  const assetMetadata = new AssetMetadata(cid);
  const release = new Release(cid);

  assetMetadata.rawContent = content.toString();
  assetMetadata.blobMetadata = cid.split("/")[0] + "/blob/metadata.json";

  const jsonResult = json.try_fromBytes(content);

  if (jsonResult.isOk) {
    const jsonValue = getObject(jsonResult.value);
    if (jsonValue) {
      let partyNames: string[] = [];
      let partyReferences: string[] = [];

      const partyList = getObject(jsonValue.get('party_list'));
      if (partyList) {
        const partys = getArray(partyList.get('partys'));
        if (partys) {
          for (let i = 0; i < partys.length; i++) {
            const party = getObject(partys[i]);
            if (!party) return;

            const reference = getValueIfExist(party, 'party_reference');
            if (reference) {
              const partyNamesArray = getArray(party.get('parties_names'));
              if (partyNamesArray) {
                const nameElement = getFirstElement(partyNamesArray);
                if (!nameElement) return;

                const name = getObject(nameElement);
                if (!name) return;

                const fullName = getObject(name.get('full_name'));
                if (!fullName) return;

                const content = getValueIfExist(fullName, 'content');
                if (content) {
                  partyReferences.push(reference);
                  partyNames.push(content);
                }
              }
            }
          }
        }
      }

      const releaseList = getObject(jsonValue.get('release_list'));
      if (releaseList) {
        const releaseData = getObject(releaseList.get('release'));
        if (releaseData) {
          release.release_reference = getValueIfExist(releaseData, 'release_reference');

          const releaseTypes = getArray(releaseData.get('release_types'));
          if (releaseTypes) {
            let types: string[] = []
            for (let i = 0; i < releaseTypes.length; i++) {
              const type = getObject(releaseTypes[i]);
              if (type) {
                const value = getValueIfExist(type, 'content')
                if (value) types.push(value)
              }
            }
            release.release_types = types;
          }

          const displayTitles = getArray(releaseData.get('display_titles'));
          if (displayTitles) {
            const titleElement = getFirstElement(displayTitles);
            if (titleElement) {
              const title = getObject(titleElement);
              if (!title) return;

              release.title_text = getValueIfExist(title, 'title_text')
              const subtitles = getArray(title.get('sub_titles'));
              if (subtitles) {
                const subtitleElement = getFirstElement(subtitles);
                if (subtitleElement) {
                  const subtitle = getObject(subtitleElement);
                  if (!subtitle) return;

                  release.subtitle = getValueIfExist(subtitle, 'content');
                }
              }
            }
          }

          const displayTitleTexts = getArray(releaseData.get('display_title_texts'));
          if (displayTitleTexts) {
            const titleTextElement = getFirstElement(displayTitleTexts);
            if (titleTextElement) {
              const titleText = getObject(titleTextElement);
              if (!titleText) return;

              release.display_title_text = getValueIfExist(titleText, 'content');
            }
          }

          const releaseId = getObject(releaseData.get('release_id'));
          if (releaseId) {
            release.grid = getValueIfExist(releaseId,'grid');
            release.icpn = getValueIfExist(releaseId,'icpn');
            const ids = getArray(releaseId.get('proprietary_ids'));
            if (ids) {
              if (ids.length > 0) {
                release.proprietary_ids = ids.map<string>(id => id.toString());
              } else {
                release.proprietary_ids = [];
              }
            }
          }

          const displayArtists = getArray(releaseData.get('display_artists'));
          if (displayArtists) {
            let artistsList: string[] = []
            for (let i = 0; i < displayArtists.length; i++) {
              const displayArtist = getObject(displayArtists[i]);
              if (displayArtist) {
                const artist = new DisplayArtist(`${cid}-${i}`);

                artist.artist_party_reference = getValueIfExist(displayArtist, 'artist_party_reference');
                artist.sequence_number = getNumberIfExist(displayArtist, 'sequence_number');

                const artistRole = getObject(displayArtist.get('display_artist_role'));
                if (artistRole) {
                  const content = getValueIfExist(artistRole, 'content');
                  if (content) {
                    artist.display_artist_roles = [content];
                  }
                }

                artist.save();
                artistsList.push(artist.id);
              }
            }
            release.display_artists = artistsList;
          }
        }
      }
    }
  } else {
    log.warning("Failed to parse JSON from Bytes: {}, CID: {}", [content.toHexString(), cid]);
  }

  assetMetadata.save();
  release.save();
}

function getObject(value: JSONValue | null): TypedMap<string, JSONValue> | null {
  return value && value.kind == JSONValueKind.OBJECT ? value.toObject() : null;
}

function getArray(value: JSONValue | null): JSONValue[] | null {
  return value && value.kind == JSONValueKind.ARRAY ? value.toArray() : null;
}

function getNumber(value: JSONValue | null): BigInt | null {
  return value && value.kind == JSONValueKind.NUMBER ? value.toBigInt() : null;
}

function getFirstElement(arr: JSONValue[] | null): JSONValue | null {
  return arr && arr.length > 0 ? arr[0] : null;
}
