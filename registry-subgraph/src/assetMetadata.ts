import { json, Bytes, dataSource, log, JSONValue } from '@graphprotocol/graph-ts';

import { AssetMetadata, Release } from './types/schema';
import { JSONValueKind } from '@graphprotocol/graph-ts/common/value';
import { getValueIfExist } from './helpers';
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
        }
      }
    }
  } else {
    log.warning("Failed to parse JSON from Bytes: {}, CID: {}", [content.toHexString(), cid]);
  }

  // const jsonObject = getObject(jsonResult.value);
  // if (jsonObject) {
  //   const releaseList = getObject(jsonObject.get('release_list'));
  //   if (releaseList) {
  //     const releaseData = releaseList.get('release');
  //     if (releaseData) {
  //       const releaseObj = getObject(releaseData);
  //       if (!releaseObj) return;
  //
  //       const displayTitle = getFirstElement(getArray(releaseObj.get('display_titles')));
  //       if (displayTitle) {
  //         const titleObj = getObject(displayTitle);
  //         if (!titleObj) return;
  //
  //         release.title_text = getValueIfExist(titleObj, 'title_text');
  //
  //         const subtitleArray = titleObj.get('sub_titles');
  //         if (!subtitleArray) return;
  //
  //         const subtitle = getFirstElement(getArray(subtitleArray));
  //         const subtitleObject = getObject(subtitle);
  //         if (!subtitleObject) return;
  //
  //         release.subtitle = subtitle ? getValueIfExist(subtitleObject, 'content') : null;
  //       }
  //     }
  //   }
  // }

  // if (jsonResult.isOk) {
  //   if (jsonResult.value.kind == JSONValueKind.OBJECT) {
  //     const data = jsonResult.value.toObject();
  //     const releaseList = data.get('release_list');
  //     if (releaseList) {
  //       const releaseJSONValue = releaseList.toObject().get('release');
  //       if (releaseJSONValue) {
  //         const releaseObject = releaseJSONValue.toObject();
  //         const displayTitles = releaseObject.get('display_titles');
  //         if (displayTitles) {
  //           const displayTitleArray = displayTitles.toArray();
  //           if (displayTitleArray.length > 0) {
  //             const displayTitle = displayTitleArray[0];
  //             release.title_text = getValueIfExist(displayTitle.toObject(), 'title_text');
  //
  //             const subtitles = displayTitle.toObject().get('sub_titles');
  //             if (subtitles) {
  //               const subtitleArray = subtitles.toArray();
  //               if (subtitleArray.length > 0) {
  //                 const subtitle = subtitleArray[0];
  //                 release.subtitle = getValueIfExist(subtitle.toObject(), 'content');
  //               }
  //             }
  //           }
  //         }
  //
  //         const displayTitleTexts = releaseObject.get('display_title_texts');
  //         if (displayTitleTexts) {
  //           const displayTitleTextsArray = displayTitleTexts.toArray();
  //           if (displayTitleTextsArray.length > 0) {
  //             const displayTitleText = displayTitleTextsArray[0];
  //             release.display_title_text = getValueIfExist(displayTitleText.toObject(), 'content');
  //           }
  //         }
  //
  //         release.release_reference = getValueIfExist(releaseObject, 'release_reference');
  //
  //         const releaseTypesJSONValue = releaseObject.get('release_types');
  //         if (releaseTypesJSONValue) {
  //           const releaseTypesArray = releaseTypesJSONValue.toArray();
  //           if (releaseTypesArray.length > 0) {
  //             let releaseTypes: string[] = []
  //             for (let i = 0; i < releaseTypesArray.length; i++) {
  //               const releaseType = releaseTypesArray[i].toObject();
  //               const content = releaseType.get('content')
  //               if (content) {
  //                 releaseTypes.push(content.toString());
  //               }
  //             }
  //             release.release_types = releaseTypes;
  //           }
  //         }
  //
  //         const releaseId = releaseObject.get('release_id');
  //         if (releaseId) {
  //           release.grid = getValueIfExist(releaseId.toObject(),'grid');
  //           release.icpn = getValueIfExist(releaseId.toObject(),'icpn');
  //           const proprietary_ids = releaseId.toObject().get('proprietary_ids');
  //           if (proprietary_ids) {
  //             const ids = proprietary_ids.toArray();
  //             let idList: string[] = []
  //             for (let i = 0; i < ids.length; i++) {
  //               const id = ids[i].toString();
  //               idList.push(id);
  //             }
  //             release.proprietary_ids = idList;
  //           }
  //         }
  //       }
  //     }
  //   }
  // } else {
  //   log.warning("Failed to parse JSON from Bytes: {}, CID: {}", [content.toHexString(), dataSource.stringParam()]);
  // }

  assetMetadata.save();
  release.save();
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
