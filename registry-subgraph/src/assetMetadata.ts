import { Bytes, dataSource, log } from '@graphprotocol/graph-ts';

import { AssetMetadata } from './types/schema';

export function handleAssetMetadata(content: Bytes): void {
  const assetMetadata = new AssetMetadata(dataSource.stringParam());
  log.info("Asset metadata blob metadata CID: {}", [dataSource.stringParam().split('/')[0]]);

  assetMetadata.rawContent = content.toString();
  assetMetadata.blobMetadata = dataSource.stringParam().split('/')[0] + '/blob/metadata.json';

  assetMetadata.save();
}
