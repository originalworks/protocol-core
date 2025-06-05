import { Bytes, dataSource } from "@graphprotocol/graph-ts";

import { AssetMetadata } from "./types/schema";

export function handleAssetMetadata(content: Bytes): void {
  const cid = dataSource.stringParam();
  const assetMetadata = new AssetMetadata(cid);

  assetMetadata.rawContent = content.toString();
  assetMetadata.blobMetadata = cid.split("/")[0] + "/blob/metadata.json";

  assetMetadata.save();
}
