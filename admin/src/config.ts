export type AdminConfig = {
  apiBaseUrl: string;
  cognitoDomain: string;
  cognitoClientId: string;
  cognitoUserPoolId: string;
  redirectUri: string;
  logoutUri: string;
  identityProvider: string;
};

export function loadConfig(): AdminConfig {
  return {
    apiBaseUrl: import.meta.env.VITE_API_BASE_URL ?? "",
    cognitoDomain: (import.meta.env.VITE_COGNITO_DOMAIN ?? "").replace(/\/$/, ""),
    cognitoClientId: import.meta.env.VITE_COGNITO_CLIENT_ID ?? "",
    cognitoUserPoolId: import.meta.env.VITE_COGNITO_USER_POOL_ID ?? "",
    redirectUri:
      import.meta.env.VITE_REDIRECT_URI ?? `${window.location.origin}/auth-callback`,
    logoutUri: import.meta.env.VITE_LOGOUT_URI ?? `${window.location.origin}/`,
    identityProvider: import.meta.env.VITE_IDENTITY_PROVIDER ?? "IAMIdentityCenter",
  };
}
