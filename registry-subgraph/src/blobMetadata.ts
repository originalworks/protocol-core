import {
  json,
  Bytes,
  dataSource
} from "@graphprotocol/graph-ts";

import { getValueIfExist } from "./helpers";
import { BlobMetadata } from "./types/schema";

export function handleBlobMetadata(content: Bytes): void {
  const blobMetadata = new BlobMetadata(dataSource.stringParam());

  const metadata = json.fromBytes(content).toObject();

  if (metadata) {
    blobMetadata.versioned_hash = getValueIfExist(metadata, "versioned_hash");
    blobMetadata.transaction_hash = getValueIfExist(metadata, "transaction_hash");
    blobMetadata.block_number = getValueIfExist(metadata, "block_number");
    blobMetadata.timestamp = getValueIfExist(metadata, "timestamp");
    blobMetadata.chain_id = getValueIfExist(metadata, "chain_id");
    blobMetadata.network_name = getValueIfExist(metadata, "network_name");
    blobMetadata.blob_ipfs_cid = getValueIfExist(metadata, "blob_ipfs_cid");
  }

  blobMetadata.save();
}
