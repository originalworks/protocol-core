import {
  Bytes,
  BigInt,
  TypedMap,
  JSONValue,
  JSONValueKind,
} from "@graphprotocol/graph-ts";

import { BlobsStatus, HealthStatus } from "./types/schema";

export const BlobsSubmittedEventId = "submitted";
export const BlobProcessedEventId = "processed";
export const BlobRejectedEventId = "rejected";

const BlobsStatusesIds = [BlobsSubmittedEventId, BlobProcessedEventId, BlobRejectedEventId];

export function initializeBlobsStatuses(): void {
  for (let i = 0; i < BlobsStatusesIds.length; i++) {
    let blobs = BlobsStatus.load(BlobsStatusesIds[i]);

    if (blobs == null) {
      blobs = new BlobsStatus(BlobsStatusesIds[i]);
      blobs.amount = BigInt.zero();
    }

    blobs.save();
  }
}

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

export function initializeHealthStatus(): void {
  let healthStatus = HealthStatus.load("status");

  if (healthStatus == null) {
    healthStatus = new HealthStatus("status");
    healthStatus.txAmount = BigInt.zero();
    healthStatus.owenTxAmount = BigInt.zero();
    healthStatus.validatorTxAmount = BigInt.zero();
    healthStatus.blobsInQueueAmount = BigInt.zero();
  }

  healthStatus.save();
}

export function recordHealthStatusBatchData(batchTimestamp: BigInt, batchTxHash: Bytes): void {
  let healthStatus = HealthStatus.load("status");

  if (healthStatus == null) {
    healthStatus = new HealthStatus("status");
  }
  healthStatus.txAmount = healthStatus.txAmount.plus(BigInt.fromI32(1));
  healthStatus.owenTxAmount = healthStatus.owenTxAmount.plus(BigInt.fromI32(1));
  healthStatus.blobsInQueueAmount = healthStatus.blobsInQueueAmount.plus(BigInt.fromI32(1));
  healthStatus.batchTimestamp = batchTimestamp;
  healthStatus.batchTxHash = batchTxHash;

  healthStatus.save();
}

export function recordHealthStatusValidatorData(validationTimestamp: BigInt, validationTxHash: Bytes): void {
  let healthStatus = HealthStatus.load("status");

  if (healthStatus == null) {
    healthStatus = new HealthStatus("status");
  }
  healthStatus.txAmount = healthStatus.txAmount.plus(BigInt.fromI32(1));
  healthStatus.validatorTxAmount = healthStatus.validatorTxAmount.plus(BigInt.fromI32(1));
  healthStatus.blobsInQueueAmount = healthStatus.blobsInQueueAmount.minus(BigInt.fromI32(1));
  healthStatus.validationTimestamp = validationTimestamp;
  healthStatus.validationTxHash = validationTxHash;

  healthStatus.save();
}

export function recordHealthStatusMoveQueueData(): void {
  let healthStatus = HealthStatus.load("status");

  if (healthStatus != null) {
    if (healthStatus.blobsInQueueAmount != BigInt.zero()) {
      healthStatus.blobsInQueueAmount = healthStatus.blobsInQueueAmount.minus(BigInt.fromI32(1));
    }
    healthStatus.save();
  }
}

export function getValueIfExist(
  sourceObject: TypedMap<string, JSONValue>,
  parameterName: string,
): string | null {
  const jsonValue = sourceObject.get(parameterName);
  if (jsonValue && jsonValue.kind == JSONValueKind.STRING) {
    return jsonValue.toString();
  } else {
    return null;
  }
}
