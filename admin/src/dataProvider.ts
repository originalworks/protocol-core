import {
  GetCommand,
  QueryCommand,
  type NativeAttributeValue,
} from "@aws-sdk/lib-dynamodb";
import type {
  DataProvider,
  GetListParams,
  GetListResult,
  GetOneParams,
  GetOneResult,
  RaRecord,
} from "react-admin";
import { getDynamoClient } from "./auth/aws";
import { loadConfig } from "./config";

type MessageRecord = RaRecord & {
  id: string;
  messageFolder: string;
  processingStatus?: string;
  createdTimestamp?: number;
  updatedTimestamp?: number;
  owenInstance?: string;
};

const STATUSES = ["unprocessed", "reserved", "processed", "rejected"] as const;

function statusFromFilter(filter: GetListParams["filter"]): string {
  const status = filter?.processingStatus;
  if (
    typeof status === "string" &&
    STATUSES.includes(status as (typeof STATUSES)[number])
  ) {
    return status;
  }
  return "unprocessed";
}

function itemToRecord(
  item: Record<string, NativeAttributeValue>,
): MessageRecord | null {
  const messageFolder = item.messageFolder;
  if (typeof messageFolder !== "string") {
    return null;
  }
  return {
    id: messageFolder,
    messageFolder,
    processingStatus:
      typeof item.processingStatus === "string"
        ? item.processingStatus
        : undefined,
    createdTimestamp:
      typeof item.createdTimestamp === "number"
        ? item.createdTimestamp
        : typeof item.createdTimestamp === "string"
          ? Number(item.createdTimestamp)
          : undefined,
    updatedTimestamp:
      typeof item.updatedTimestamp === "number"
        ? item.updatedTimestamp
        : typeof item.updatedTimestamp === "string"
          ? Number(item.updatedTimestamp)
          : undefined,
    owenInstance:
      typeof item.owenInstance === "string" ? item.owenInstance : undefined,
  };
}

function toHttpError(error: unknown): Error {
  const err = new Error(
    error instanceof Error ? error.message : "DynamoDB request failed",
  );
  const name =
    error && typeof error === "object" && "name" in error
      ? String((error as { name: string }).name)
      : "";
  if (name === "NotAuthorizedException" || name === "ExpiredTokenException") {
    (err as Error & { status: number }).status = 401;
  }
  return err;
}

export const dataProvider: DataProvider = {
  getList: async <RecordType extends RaRecord = MessageRecord>(
    resource: string,
    params: GetListParams,
  ): Promise<GetListResult<RecordType>> => {
    if (resource !== "messages") {
      throw new Error(`Unknown resource: ${resource}`);
    }

    const config = loadConfig();
    const status = statusFromFilter(params.filter);
    const perPage = params.pagination?.perPage ?? 25;
    const page = params.pagination?.page ?? 1;

    try {
      const client = await getDynamoClient();
      const result = await client.send(
        new QueryCommand({
          TableName: config.messageStatusTableName,
          IndexName: config.processingStatusIndexName,
          KeyConditionExpression: "processingStatus = :status",
          ExpressionAttributeValues: {
            ":status": status,
          },
          Limit: perPage,
          ScanIndexForward: false,
        }),
      );

      const data = (result.Items ?? [])
        .map((item) => itemToRecord(item))
        .filter((item): item is MessageRecord => item !== null);

      return {
        data: data as unknown as RecordType[],
        total: result.LastEvaluatedKey
          ? page * perPage + 1
          : (page - 1) * perPage + data.length,
        pageInfo: {
          hasNextPage: Boolean(result.LastEvaluatedKey),
          hasPreviousPage: page > 1,
        },
      };
    } catch (error) {
      throw toHttpError(error);
    }
  },

  getOne: async <RecordType extends RaRecord = MessageRecord>(
    resource: string,
    params: GetOneParams,
  ): Promise<GetOneResult<RecordType>> => {
    if (resource !== "messages") {
      throw new Error(`Unknown resource: ${resource}`);
    }

    const config = loadConfig();
    try {
      const client = await getDynamoClient();
      const result = await client.send(
        new GetCommand({
          TableName: config.messageStatusTableName,
          Key: { messageFolder: String(params.id) },
        }),
      );

      if (!result.Item) {
        const error = new Error(`Message not found: ${params.id}`);
        (error as Error & { status: number }).status = 404;
        throw error;
      }

      const record = itemToRecord(result.Item);
      if (!record) {
        throw new Error("Failed to parse DynamoDB item");
      }

      return { data: record as unknown as RecordType };
    } catch (error) {
      throw toHttpError(error);
    }
  },

  getMany: async () => {
    throw new Error("getMany is not supported");
  },
  getManyReference: async () => {
    throw new Error("getManyReference is not supported");
  },
  create: async () => {
    throw new Error("Read-only admin");
  },
  update: async () => {
    throw new Error("Read-only admin");
  },
  updateMany: async () => {
    throw new Error("Read-only admin");
  },
  delete: async () => {
    throw new Error("Read-only admin");
  },
  deleteMany: async () => {
    throw new Error("Read-only admin");
  },
};
