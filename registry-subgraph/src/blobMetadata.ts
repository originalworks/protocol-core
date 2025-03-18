import {
  log,
  json,
  Bytes,
  dataSource
} from "@graphprotocol/graph-ts";
import { JSONValueKind } from "@graphprotocol/graph-ts/common/value";

import { getValueIfExist } from "./helpers";
import { BlobMetadata } from "./types/schema";

export function handleBlobMetadata(content: Bytes): void {
  const blobMetadata = new BlobMetadata(dataSource.stringParam());

  const jsonResult = json.try_fromBytes(content);

  if (jsonResult.isOk) {
    if (jsonResult.value.kind == JSONValueKind.OBJECT) {
      const metadata = jsonResult.value.toObject();

      blobMetadata.versioned_hash = getValueIfExist(metadata, "versioned_hash");
      blobMetadata.transaction_hash = getValueIfExist(metadata, "transaction_hash");
      blobMetadata.block_number = getValueIfExist(metadata, "block_number");
      blobMetadata.timestamp = getValueIfExist(metadata, "timestamp");
      blobMetadata.chain_id = getValueIfExist(metadata, "chain_id");
      blobMetadata.network_name = getValueIfExist(metadata, "network_name");
      blobMetadata.blob_ipfs_cid = getValueIfExist(metadata, "blob_ipfs_cid");
    }
  } else {
    log.warning("Failed to parse JSON from Bytes: {}, CID: {}", [content.toHexString(), dataSource.stringParam()]);
  }

  blobMetadata.save();
}
