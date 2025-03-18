import {
  BigInt,
  TypedMap,
  JSONValue
} from "@graphprotocol/graph-ts";

import { BlobsStatus } from "./types/schema";

export function recordBlobsStatuses(id: string, timestamp: BigInt): void {
  let blobs = BlobsStatus.load(id);

  if (blobs == null) {
    blobs = new BlobsStatus(id);
    blobs.amount = BigInt.zero();
  }

  blobs.amount = blobs.amount.plus(BigInt.fromI32(1));
  blobs.latestEvent = timestamp;

  blobs.save();
}

export function getValueIfExist(
  sourceObject: TypedMap<string, JSONValue>,
  parameterName: string,
): string | null {
  const jsonValue = sourceObject.get(parameterName)
  if (jsonValue) {
    return jsonValue.toString()
  } else {
    return null
  }
}
