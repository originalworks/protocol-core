import { json, Bytes, dataSource, log } from '@graphprotocol/graph-ts';

import { getValueIfExist } from "./helpers";
import { BlobMetadata } from "./types/schema";
import { AssetMetadataTemplate } from './types/templates';

// Just an example: we create a data source up to 70 files.
const maxFiles = 70

export function handleBlobMetadata(content: Bytes): void {
  log.info("Blob metadata CID: {}", [dataSource.stringParam()]);

  const blobMetadata = new BlobMetadata(dataSource.stringParam());

  const metadata = json.fromBytes(content).toObject();

  if (metadata) {
    blobMetadata.versioned_hash = getValueIfExist(metadata, 'versioned_hash');
    blobMetadata.transaction_hash = getValueIfExist(metadata, 'transaction_hash');
    blobMetadata.block_number = getValueIfExist(metadata, 'block_number');
    blobMetadata.timestamp = getValueIfExist(metadata, 'timestamp');
    blobMetadata.chain_id = getValueIfExist(metadata, 'chain_id');
    blobMetadata.network_name = getValueIfExist(metadata, 'network_name');
    blobMetadata.blob_ipfs_cid = getValueIfExist(metadata, 'blob_ipfs_cid');
  }

  blobMetadata.save();

  // Now spin up sub‑dataSources for each JSON file in IPFS
  for (let i = 1; i <= maxFiles; i++) {
    let cid = dataSource.stringParam().split('/')[0];
    let ipfsPath = cid + "/json/" + i.toString() + ".json";
    log.info("Creating IPFS data source for file: {}", [ipfsPath]);

    // This will invoke the handleAssetMetadata() in "assetMetadata.ts"
    AssetMetadataTemplate.create(ipfsPath);
  }
}
