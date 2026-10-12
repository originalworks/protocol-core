import { DynamoDBClient } from "@aws-sdk/client-dynamodb";
import { fromCognitoIdentityPool } from "@aws-sdk/credential-provider-cognito-identity";
import { DynamoDBDocumentClient } from "@aws-sdk/lib-dynamodb";
import { getIdToken } from "./cognito";
import { loadConfig } from "../config";

let cachedClient: DynamoDBDocumentClient | null = null;
let cachedToken: string | null = null;

export async function getDynamoClient(): Promise<DynamoDBDocumentClient> {
  const config = loadConfig();
  const idToken = await getIdToken();

  if (cachedClient && cachedToken === idToken) {
    return cachedClient;
  }

  const loginKey = `cognito-idp.${config.awsRegion}.amazonaws.com/${config.cognitoUserPoolId}`;

  const client = new DynamoDBClient({
    region: config.awsRegion,
    credentials: fromCognitoIdentityPool({
      clientConfig: { region: config.awsRegion },
      identityPoolId: config.cognitoIdentityPoolId,
      logins: {
        [loginKey]: idToken,
      },
    }),
  });

  cachedClient = DynamoDBDocumentClient.from(client, {
    marshallOptions: { removeUndefinedValues: true },
  });
  cachedToken = idToken;
  return cachedClient;
}

export function clearDynamoClientCache(): void {
  cachedClient = null;
  cachedToken = null;
}
