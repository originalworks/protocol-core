export type AdminConfig = {
  awsRegion: string;
  cognitoDomain: string;
  cognitoClientId: string;
  cognitoUserPoolId: string;
  cognitoIdentityPoolId: string;
  messageStatusTableName: string;
  processingStatusIndexName: string;
  redirectUri: string;
  logoutUri: string;
  identityProvider: string;
};

export function loadConfig(): AdminConfig {
  return {
    awsRegion: import.meta.env.VITE_AWS_REGION ?? "us-east-1",
    cognitoDomain: (import.meta.env.VITE_COGNITO_DOMAIN ?? "").replace(/\/$/, ""),
    cognitoClientId: import.meta.env.VITE_COGNITO_CLIENT_ID ?? "",
    cognitoUserPoolId: import.meta.env.VITE_COGNITO_USER_POOL_ID ?? "",
    cognitoIdentityPoolId: import.meta.env.VITE_COGNITO_IDENTITY_POOL_ID ?? "",
    messageStatusTableName: import.meta.env.VITE_MESSAGE_STATUS_TABLE_NAME ?? "",
    processingStatusIndexName:
      import.meta.env.VITE_PROCESSING_STATUS_INDEX_NAME ?? "ProcessingStatusIndex",
    redirectUri:
      import.meta.env.VITE_REDIRECT_URI ?? `${window.location.origin}/auth-callback`,
    logoutUri: import.meta.env.VITE_LOGOUT_URI ?? `${window.location.origin}/`,
    identityProvider: import.meta.env.VITE_IDENTITY_PROVIDER ?? "IAMIdentityCenter",
  };
}
