import { json, Bytes, dataSource, log } from '@graphprotocol/graph-ts';

import { AssetMetadata, Release } from './types/schema';
import { JSONValueKind } from '@graphprotocol/graph-ts/common/value';
import { getValueIfExist } from './helpers';

export function handleAssetMetadata(content: Bytes): void {
  const assetMetadata = new AssetMetadata(dataSource.stringParam());
  const release = new Release(dataSource.stringParam());

  assetMetadata.rawContent = content.toString();
  assetMetadata.blobMetadata = dataSource.stringParam().split("/")[0] + "/blob/metadata.json";

  const jsonResult = json.try_fromBytes(content);
  if (jsonResult.isOk) {
    if (jsonResult.value.kind == JSONValueKind.OBJECT) {
      const data = jsonResult.value.toObject();
      const releaseList = data.get('release_list');
      if (releaseList) {
        const releaseJSONValue = releaseList.toObject().get('release');
        if (releaseJSONValue) {
          const releaseObject = releaseJSONValue.toObject();
          const displayTitles = releaseObject.get('display_titles');
          if (displayTitles) {
            const displayTitleArray = displayTitles.toArray();
            if (displayTitleArray.length > 0) {
              const displayTitle = displayTitleArray[0];
              release.title_text = getValueIfExist(displayTitle.toObject(), 'title_text');

              const subtitles = displayTitle.toObject().get('sub_titles');
              if (subtitles) {
                const subtitleArray = subtitles.toArray();
                if (subtitleArray.length > 0) {
                  const subtitle = subtitleArray[0];
                  release.subtitle = getValueIfExist(subtitle.toObject(), 'content');
                }
              }
            }
          }

          const releaseId = releaseObject.get('release_id');
          if (releaseId) {
            release.grid = getValueIfExist(releaseId.toObject(),'grid');
            release.icpn = getValueIfExist(releaseId.toObject(),'icpn');
            const proprietary_ids = releaseId.toObject().get('proprietary_ids');
            if (proprietary_ids) {
              const ids = proprietary_ids.toArray();
              let idList: string[] = []
              for (let i = 0; i < ids.length; i++) {
                const id = ids[i].toString();
                idList.push(id);
              }
              release.proprietary_ids = idList;
            }
          }
        }
      }
    }
  } else {
    log.warning("Failed to parse JSON from Bytes: {}, CID: {}", [content.toHexString(), dataSource.stringParam()]);
  }

  assetMetadata.save();
  release.save();
}

// if (metadata) {
//   const type = metadata.get('type')
//
//   if (type) {
//     agreementMetadata.type = type.toString()
//     if (type.toString() == 'AGREEMENT') {
//       agreementMetadata.title = getValueIfExist(metadata, 'title')
//     } else if (type.toString() == 'ASSET') {
//       const factsheetJSONValue = metadata.get('factsheet')
//       if (factsheetJSONValue) {
//         const factsheetObject = factsheetJSONValue.toObject()
//         agreementMetadata.isrc = getValueIfExist(factsheetObject, 'isrc')
//         agreementMetadata.assetURI = getValueIfExist(
//           factsheetObject,
//           'assetURI',
//         )
//         agreementMetadata.ownerURI = getValueIfExist(
//           factsheetObject,
//           'ownerURI',
//         )
//         agreementMetadata.title = getValueIfExist(factsheetObject, 'name')
//       }
//     }
//   }
// }
