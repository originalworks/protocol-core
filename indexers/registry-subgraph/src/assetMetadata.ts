import { Bytes, dataSource, json } from "@graphprotocol/graph-ts";

import { AssetMetadata, ImageMetadata } from "./types/schema";
import { getArray, getObject, getValueIfExist } from "./helpers";

export function handleAssetMetadata(content: Bytes): void {
  const cid = dataSource.stringParam();
  const assetMetadata = new AssetMetadata(cid);

  assetMetadata.rawContent = content.toString();
  assetMetadata.blobMetadata = cid.split("/")[0] + "/blob/metadata.json";

  assetMetadata.save();

  const imageMetadata = new ImageMetadata(cid);
  imageMetadata.images = [];

  const jsonResult = json.try_fromBytes(content);

  if (jsonResult.isOk) {
    const jsonValue = getObject(jsonResult.value);
    if (jsonValue) {
      const resourceListObject = getObject(jsonValue.get("resource_list"));
      if (resourceListObject) {
        const imagesData = getArray(resourceListObject.get("images"));
        if (imagesData) {
          const imagesList: string[] = [];
          for (let i = 0; i < imagesData.length; i++) {
            const imageObject = getObject(imagesData[i]);
            if (imageObject) {
              const technicalDetails = getArray(imageObject.get("technical_details"));
              if (technicalDetails) {
                for (let j = 0; j < technicalDetails.length; j++) {
                  const technicalDetailObject = getObject(technicalDetails[j]);
                  if (technicalDetailObject) {
                    const file = getObject(technicalDetailObject.get("file"));
                    if (file) {
                      const uri = getValueIfExist(file, "uri");
                      if (uri) {
                        imagesList.push(uri);
                      }
                    }
                  }
                }
              }
            }
          }

          imageMetadata.images = imagesList;
        }
      }
    }
  }

  imageMetadata.save();
}
