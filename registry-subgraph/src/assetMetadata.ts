import {
  log,
  json,
  Bytes,
  TypedMap,
  JSONValue,
  dataSource,
  JSONValueKind,
} from "@graphprotocol/graph-ts";

import {
  Image,
  PLine,
  Release,
  ResourceId,
  Fingerprint,
  Contributor,
  PartyElement,
  ResourceList,
  AssetMetadata,
  DisplayArtist,
  SoundRecording,
  TechnicalDetail,
  DisplayArtistName,
  SoundRecordingEdition,
} from "./types/schema";
import { getNumberIfExist, getValueIfExist } from "./helpers";

export function handleAssetMetadata(content: Bytes): void {
  const cid = dataSource.stringParam();
  const assetMetadata = new AssetMetadata(cid);
  const release = new Release(cid);
  const resourceList = new ResourceList(cid);

  assetMetadata.rawContent = content.toString();
  assetMetadata.blobMetadata = cid.split("/")[0] + "/blob/metadata.json";

  const jsonResult = json.try_fromBytes(content);

  if (jsonResult.isOk) {
    const jsonValue = getObject(jsonResult.value);
    if (jsonValue) {
      const partyList = getObject(jsonValue.get("party_list"));
      if (partyList) {
        let partyListMetadata: string[] = []
        const partys = getArray(partyList.get("partys"));
        if (partys) {
          for (let i = 0; i < partys.length; i++) {
            const partyEl = new PartyElement(`${cid}-${i}`)
            const party = getObject(partys[i]);
            if (!party) return;

            partyEl.party_reference = getValueIfExist(party, "party_reference");;

            const partyNamesArray = getArray(party.get("parties_names"));
            if (partyNamesArray) {
              const nameElement = getFirstElement(partyNamesArray);
              if (!nameElement) return;

              const name = getObject(nameElement);
              if (!name) return;

              const fullName = getObject(name.get("full_name"));
              if (!fullName) return;

              partyEl.party_name = getValueIfExist(fullName, "content");
            }
            partyEl.save()
            partyListMetadata.push(partyEl.id);
          }
        }
        assetMetadata.partyList = partyListMetadata;
      }

      const releaseList = getObject(jsonValue.get("release_list"));
      if (releaseList) {
        const releaseData = getObject(releaseList.get("release"));
        if (releaseData) {
          release.release_reference = getValueIfExist(releaseData, "release_reference");

          const releaseTypes = getArray(releaseData.get("release_types"));
          if (releaseTypes) {
            let types: string[] = []
            for (let i = 0; i < releaseTypes.length; i++) {
              const type = getObject(releaseTypes[i]);
              if (type) {
                const value = getValueIfExist(type, "content")
                if (value) types.push(value)
              }
            }
            release.release_types = types;
          }

          const releaseLabelReferences = getArray(releaseData.get("release_label_references"));
          if (releaseLabelReferences) {
            let labels: string[] = []
            for (let i = 0; i < releaseLabelReferences.length; i++) {
              const label = getObject(releaseLabelReferences[i]);
              if (label) {
                const value = getValueIfExist(label, "content")
                if (value) labels.push(value)
              }
            }
            release.release_label_references = labels;
          }

          const genres = getArray(releaseData.get("genres"));
          if (genres) {
            let genresList: string[] = []
            for (let i = 0; i < genres.length; i++) {
              const genre = getObject(genres[i]);
              if (genre) {
                const value = getValueIfExist(genre, "genre_text")
                if (value) genresList.push(value)
              }
            }
            release.genres = genresList;
          }

          const displayTitles = getArray(releaseData.get("display_titles"));
          if (displayTitles) {
            const titleElement = getFirstElement(displayTitles);
            if (titleElement) {
              const title = getObject(titleElement);
              if (!title) return;

              release.title_text = getValueIfExist(title, "title_text")
              const subtitles = getArray(title.get("sub_titles"));
              if (subtitles) {
                const subtitleElement = getFirstElement(subtitles);
                if (subtitleElement) {
                  const subtitle = getObject(subtitleElement);
                  if (!subtitle) return;

                  release.subtitle = getValueIfExist(subtitle, "content");
                }
              }
            }
          }

          const displayTitleTexts = getArray(releaseData.get("display_title_texts"));
          if (displayTitleTexts) {
            const titleTextElement = getFirstElement(displayTitleTexts);
            if (titleTextElement) {
              const titleText = getObject(titleTextElement);
              if (!titleText) return;

              release.display_title_text = getValueIfExist(titleText, "content");
            }
          }

          const releaseId = getObject(releaseData.get("release_id"));
          if (releaseId) {
            release.grid = getValueIfExist(releaseId,"grid");
            release.icpn = getValueIfExist(releaseId,"icpn");
            const ids = getArray(releaseId.get("proprietary_ids"));
            if (ids) {
              if (ids.length > 0) {
                release.proprietary_ids = ids.map<string>(id => id.toString());
              } else {
                release.proprietary_ids = [];
              }
            }
          }

          const displayArtists = getArray(releaseData.get("display_artists"));
          if (displayArtists) {
            let artistsList: string[] = []
            for (let i = 0; i < displayArtists.length; i++) {
              const displayArtist = getObject(displayArtists[i]);
              if (displayArtist) {
                const artist = new DisplayArtist(`${cid}-${i}`);

                artist.artist_party_reference = getValueIfExist(displayArtist, "artist_party_reference");
                artist.sequence_number = getNumberIfExist(displayArtist, "sequence_number");

                const artistRole = getObject(displayArtist.get("display_artist_role"));
                if (artistRole) {
                  const content = getValueIfExist(artistRole, "content");
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

          const displayArtistNames = getArray(releaseData.get("display_artist_names"));
          if (displayArtistNames) {
            let artistsList: string[] = []
            for (let i = 0; i < displayArtistNames.length; i++) {
              const displayArtist = getObject(displayArtistNames[i]);
              if (displayArtist) {
                const artist = new DisplayArtistName(`${cid}-${i}`);

                artist.applicable_territory_code = getValueIfExist(displayArtist, "applicable_territory_code");
                artist.language_and_script_type = getValueIfExist(displayArtist, "language_and_script_code");
                artist.display_artist_name = getValueIfExist(displayArtist, "content");

                artist.save();
                artistsList.push(artist.id);
              }
            }
            release.display_artist_names = artistsList;
          }
        }
      }

      const resourceListObject = getObject(jsonValue.get("resource_list"));
      if (resourceListObject) {
        const soundRecordingData = getArray(resourceListObject.get("sound_recordings"));
        if (soundRecordingData) {
          const soundRecordingsList: string[] = []
          for (let i = 0; i < soundRecordingData.length; i++) {
            const soundRecordingObject = getObject(soundRecordingData[i]);
            if (soundRecordingObject) {
              const soundRecording = new SoundRecording(`${cid}-${i}`);

              soundRecording.resource_reference = getValueIfExist(soundRecordingObject, "resource_reference");

              const kind = getObject(soundRecordingObject.get("kind"));
              if (kind) {
                soundRecording.kind = getValueIfExist(kind, "content");
              }

              const workIdsArray = getArray(soundRecordingObject.get("work_ids"));
              if (workIdsArray) {
                const workIdsList: string[] = []
                for (let j = 0; j < workIdsArray.length; j++) {
                  const workId = getObject(workIdsArray[j]);
                  if (workId) {
                    const iswc = getValueIfExist(workId, "iswc");
                    if (iswc) workIdsList.push(iswc);
                  }
                }
                soundRecording.work_ids = workIdsList;
              }

              const displayTitles = getArray(soundRecordingObject.get("display_titles"));
              if (displayTitles) {
                const titleElement = getFirstElement(displayTitles);
                if (titleElement) {
                  const title = getObject(titleElement);
                  if (!title) return;

                  soundRecording.display_title = getValueIfExist(title, "title_text")
                  const subtitles = getArray(title.get("sub_titles"));
                  if (subtitles) {
                    const subtitleElement = getFirstElement(subtitles);
                    if (subtitleElement) {
                      const subtitle = getObject(subtitleElement);
                      if (!subtitle) return;

                      soundRecording.subtitle = getValueIfExist(subtitle, "content");
                    }
                  }
                }
              }

              const displayTitleTexts = getArray(soundRecordingObject.get("display_title_texts"));
              if (displayTitleTexts) {
                const titleTextElement = getFirstElement(displayTitleTexts);
                if (titleTextElement) {
                  const titleText = getObject(titleTextElement);
                  if (!titleText) return;

                  soundRecording.display_title_text = getValueIfExist(titleText, "content");
                }
              }

              const soundRecordingEditions = getArray(soundRecordingObject.get("sound_recording_editions"));
              if (soundRecordingEditions) {
                const soundRecordingEditionsList: string[] = []
                for (let j = 0; j < soundRecordingEditions.length; j++) {
                  const soundRecordingEdition = new SoundRecordingEdition(`${cid}-${i}-${j}`);
                  const edition = getObject(soundRecordingEditions[j]);
                  if (edition) {
                    const resourceIds = getArray(edition.get("resource_ids"));
                    if (resourceIds) {
                      const resourceId = getFirstElement(resourceIds);
                      if (resourceId) {
                        const resourceIdObject = getObject(resourceId);
                        if (resourceIdObject) {
                          soundRecordingEdition.isrc = getValueIfExist(resourceIdObject, "isrc");
                        }
                      }
                    }

                    const plinesArray = getArray(edition.get("p_lines"));
                    if (plinesArray) {
                      const plinesList: string[] = []
                      for (let k = 0; k < plinesArray.length; k++) {
                        const pline = new PLine(`${cid}-${i}-${j}-${k}`);
                        const plineObject = getObject(plinesArray[j]);
                        if (plineObject) {
                          pline.year = getNumberIfExist(plineObject, "year")
                          pline.namespace = getValueIfExist(plineObject, "namespace");
                        }
                        pline.save();
                        plinesList.push(pline.id);
                      }
                      soundRecordingEdition.p_lines = plinesList;
                    }

                    const technicalDetailsArray = getArray(edition.get("technical_details"));
                    if (technicalDetailsArray) {
                      for (let k = 0; k < technicalDetailsArray.length; k++) {
                        const technicalDetail = getObject(technicalDetailsArray[k]);
                        if (technicalDetail) {
                          const deliveryFilesArray = getArray(technicalDetail.get("delivery_files"));
                          if (deliveryFilesArray) {
                            for (let l = 0; l < deliveryFilesArray.length; l++) {
                              const deliveryFile = getObject(deliveryFilesArray[l]);
                              if (deliveryFile) {
                                const fingerprintsArray = getArray(deliveryFile.get("fingerprints"));
                                if (fingerprintsArray) {
                                  const fingerprintsList: string[] = []
                                  for (let m = 0; m < fingerprintsArray.length; m++) {
                                    const fingerprint = new Fingerprint(`${cid}-${i}-${j}-${k}-${l}-${m}`);
                                    const fingerprintObject = getObject(fingerprintsArray[m]);
                                    if (fingerprintObject) {
                                      fingerprint.namespace = getValueIfExist(fingerprintObject, "namespace");
                                      fingerprint.user_defined_value = getValueIfExist(fingerprintObject, "user_defined_value");
                                      fingerprint.parameter = getValueIfExist(fingerprintObject, "parameter");
                                    }
                                    fingerprint.save();
                                    fingerprintsList.push(fingerprint.id);
                                  }
                                  soundRecordingEdition.fingerprints = fingerprintsList;
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                  soundRecordingEdition.save();
                  soundRecordingEditionsList.push(soundRecordingEdition.id);
                }
                soundRecording.sound_recording_editions = soundRecordingEditionsList;
              }

              const contributors = getArray(soundRecordingObject.get("contributors"));
              if (contributors) {
                const contributorsList: string[] = []
                for (let j = 0; j < contributors.length; j++) {
                  const contributor = new Contributor(`${cid}-${i}-${j}`);
                  const contributorObject = getObject(contributors[j]);
                  if (contributorObject) {
                    contributor.contributor_party_reference = getValueIfExist(contributorObject, "contributor_party_reference");
                    contributor.sequence_number = getNumberIfExist(contributorObject, "sequence_number");
                    const roles = getArray(contributorObject.get("roles"));
                    if (roles) {
                      const rolesList: string[] = []
                      for (let k = 0; k < roles.length; k++) {
                        const role = getObject(roles[k]);
                        if (role) {
                          const content = getValueIfExist(role, "content");
                          if (content) {
                            rolesList.push(content);
                          }
                        }
                      }
                      contributor.contributor_roles = rolesList;
                    }
                  }
                  contributor.save();
                  contributorsList.push(contributor.id);
                }
                soundRecording.contributors = contributorsList;
              }

              soundRecording.save();
              soundRecordingsList.push(soundRecording.id);
            }
          }
          resourceList.sound_recordings = soundRecordingsList;
        }

        const imagesData = getArray(resourceListObject.get("images"));
        if (imagesData) {
          const imagesList: string[] = []
          for (let i = 0; i < imagesData.length; i++) {
            const imageObject = getObject(imagesData[i]);
            if (imageObject) {
              const image = new Image(`${cid}-${i}`);

              image.resource_reference = getValueIfExist(imageObject, "resource_reference");

              const kind = getObject(imageObject.get("kind"));
              if (kind) {
                image.kind = getValueIfExist(kind, "content");
              }

              const resourceIdsList: string[] = []
              const resourcesIds = getArray(imageObject.get("resource_ids"));
              if (resourcesIds) {
                for (let j = 0; j < resourcesIds.length; j++) {
                  const resourceIdObject = getObject(resourcesIds[j]);
                  if (resourceIdObject) {
                    const resourceId = new ResourceId(`${cid}-${i}-${j}`);

                    const proprietaryIds = getArray(resourceIdObject.get("proprietary_ids"));
                    if (proprietaryIds) {
                      const proprietaryId = getFirstElement(proprietaryIds);
                      if (proprietaryId) {
                        const proprietaryIdObject = getObject(proprietaryId);
                        if (proprietaryIdObject) {
                          resourceId.content = getValueIfExist(proprietaryIdObject, "content");
                          resourceId.namespace = getValueIfExist(proprietaryIdObject, "namespace");
                        }
                      }
                    }

                    resourceId.save();
                    resourceIdsList.push(resourceId.id);
                  }
                }
              }
              image.resource_ids = resourceIdsList;

              const warningTypesList: string[] = []
              const warningTypes = getArray(imageObject.get("parental_warning_types"));
              if (warningTypes) {
                for (let j = 0; j < warningTypes.length; j++) {
                  const warningTypeObject = getObject(warningTypes[j]);
                  if (warningTypeObject) {
                    const warningType = getValueIfExist(warningTypeObject, "content");
                    if (warningType) {
                      warningTypesList.push(warningType);
                    }
                  }
                }
              }
              image.parental_warning_types = warningTypesList;

              const technicalDetailsList: string[] = []
              const technicalDetails = getArray(imageObject.get("technical_details"));
              if (technicalDetails) {
                for (let j = 0; j < technicalDetails.length; j++) {
                  const technicalDetailObject = getObject(technicalDetails[j]);
                  if (technicalDetailObject) {
                    const technicalDetail = new TechnicalDetail(`${cid}-${i}-${j}`);
                    technicalDetail.technical_resource_details_reference = getValueIfExist(technicalDetailObject, "technical_resource_details_reference");

                    const file = getObject(technicalDetailObject.get("file"));
                    if (file) {
                      technicalDetail.fileUri = getValueIfExist(file, "uri");
                    }

                    technicalDetail.save();
                    technicalDetailsList.push(technicalDetail.id);
                  }
                }
              }
              image.technical_details = technicalDetailsList;

              image.save();
              imagesList.push(image.id);
            }
            resourceList.images = imagesList;
          }
        }
      }
    }
  } else {
    log.warning("Failed to parse JSON from Bytes: {}, CID: {}", [content.toHexString(), cid]);
  }

  assetMetadata.resourceList = resourceList.id;

  assetMetadata.save();
  release.save();
  resourceList.save();
}

function getObject(value: JSONValue | null): TypedMap<string, JSONValue> | null {
  return value && value.kind == JSONValueKind.OBJECT ? value.toObject() : null;
}

function getArray(value: JSONValue | null): JSONValue[] | null {
  return value && value.kind == JSONValueKind.ARRAY ? value.toArray() : null;
}

function getFirstElement(arr: JSONValue[] | null): JSONValue | null {
  return arr && arr.length > 0 ? arr[0] : null;
}
