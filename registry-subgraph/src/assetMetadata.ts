import { Bytes, dataSource } from "@graphprotocol/graph-ts";

import { AssetMetadata } from "./types/schema";

export function handleAssetMetadata(content: Bytes): void {
  const assetMetadata = new AssetMetadata(dataSource.stringParam());

  assetMetadata.rawContent = content.toString();

  assetMetadata.save();
}
