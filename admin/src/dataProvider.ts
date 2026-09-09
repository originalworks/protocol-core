import type {
  DataProvider,
  GetListParams,
  GetListResult,
  GetOneParams,
  GetOneResult,
  RaRecord,
} from "react-admin";
import { getIdToken } from "./auth/cognito";
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

async function apiFetch(path: string, init?: RequestInit): Promise<Response> {
  const config = loadConfig();
  const token = await getIdToken();
  const response = await fetch(`${config.apiBaseUrl}${path}`, {
    ...init,
    headers: {
      ...(init?.headers ?? {}),
      authorization: `Bearer ${token}`,
      accept: "application/json",
    },
  });
  if (!response.ok) {
    const error = new Error(`API error ${response.status}`);
    (error as Error & { status: number }).status = response.status;
    throw error;
  }
  return response;
}

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

export const dataProvider: DataProvider = {
  getList: async <RecordType extends RaRecord = MessageRecord>(
    resource: string,
    params: GetListParams,
  ): Promise<GetListResult<RecordType>> => {
    if (resource !== "messages") {
      throw new Error(`Unknown resource: ${resource}`);
    }

    const status = statusFromFilter(params.filter);
    const perPage = params.pagination?.perPage ?? 25;
    const page = params.pagination?.page ?? 1;
    const query = new URLSearchParams({
      status,
      limit: String(perPage),
    });

    const response = await apiFetch(`/messages?${query.toString()}`);
    const json = (await response.json()) as {
      data: MessageRecord[];
      nextCursor?: string | null;
    };

    return {
      data: json.data as unknown as RecordType[],
      // DynamoDB does not return total counts cheaply; approximate for React Admin.
      total: json.nextCursor
        ? page * perPage + 1
        : (page - 1) * perPage + json.data.length,
      pageInfo: {
        hasNextPage: Boolean(json.nextCursor),
        hasPreviousPage: page > 1,
      },
    };
  },

  getOne: async <RecordType extends RaRecord = MessageRecord>(
    resource: string,
    params: GetOneParams,
  ): Promise<GetOneResult<RecordType>> => {
    if (resource !== "messages") {
      throw new Error(`Unknown resource: ${resource}`);
    }
    const id = encodeURIComponent(String(params.id));
    const response = await apiFetch(`/messages/${id}`);
    const data = (await response.json()) as MessageRecord;
    return { data: data as unknown as RecordType };
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
