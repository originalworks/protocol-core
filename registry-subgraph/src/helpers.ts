import {
  Bytes,
  BigInt,
  TypedMap,
  JSONValue,
} from "@graphprotocol/graph-ts";

import { BlobsStatus } from "./types/schema";

export function recordBlobsStatuses(id: string, timestamp: BigInt, txHash: Bytes): void {
  let blobs = BlobsStatus.load(id);

  if (blobs == null) {
    blobs = new BlobsStatus(id);
    blobs.amount = BigInt.zero();
  }

  blobs.amount = blobs.amount.plus(BigInt.fromI32(1));
  blobs.latestEventTimestamp = timestamp;
  blobs.latestEventTxHash = txHash;

  blobs.save();
}

export function getValueIfExist(
  sourceObject: TypedMap<string, JSONValue>,
  parameterName: string,
): string | null {
  const jsonValue = sourceObject.get(parameterName);
  if (jsonValue) {
    return jsonValue.toString();
  } else {
    return null;
  }
}
