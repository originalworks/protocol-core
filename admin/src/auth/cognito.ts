const TOKEN_KEY = "owen_admin_tokens";
const VERIFIER_KEY = "owen_admin_pkce_verifier";
const STATE_KEY = "owen_admin_oauth_state";

export type TokenSet = {
  accessToken: string;
  idToken: string;
  refreshToken?: string;
  expiresAt: number;
};

function base64UrlEncode(buffer: ArrayBuffer): string {
  const bytes = new Uint8Array(buffer);
  let binary = "";
  bytes.forEach((b) => {
    binary += String.fromCharCode(b);
  });
  return btoa(binary).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
}

async function sha256(input: string): Promise<ArrayBuffer> {
  const data = new TextEncoder().encode(input);
  return crypto.subtle.digest("SHA-256", data);
}

function randomString(length = 64): string {
  const chars =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";
  const values = crypto.getRandomValues(new Uint8Array(length));
  return Array.from(values, (v) => chars[v % chars.length]).join("");
}

export function getStoredTokens(): TokenSet | null {
  const raw = localStorage.getItem(TOKEN_KEY);
  if (!raw) return null;
  try {
    return JSON.parse(raw) as TokenSet;
  } catch {
    return null;
  }
}

export function storeTokens(tokens: TokenSet): void {
  localStorage.setItem(TOKEN_KEY, JSON.stringify(tokens));
}

export function clearTokens(): void {
  localStorage.removeItem(TOKEN_KEY);
}

export function isTokenValid(tokens: TokenSet | null): boolean {
  if (!tokens?.accessToken) return false;
  return tokens.expiresAt > Date.now() + 30_000;
}

export async function beginLogin(config: {
  cognitoDomain: string;
  cognitoClientId: string;
  redirectUri: string;
  identityProvider: string;
}): Promise<void> {
  const verifier = randomString(64);
  const state = randomString(32);
  const challenge = base64UrlEncode(await sha256(verifier));

  localStorage.setItem(VERIFIER_KEY, verifier);
  localStorage.setItem(STATE_KEY, state);

  const params = new URLSearchParams({
    response_type: "code",
    client_id: config.cognitoClientId,
    redirect_uri: config.redirectUri,
    scope: "openid email profile",
    state,
    code_challenge_method: "S256",
    code_challenge: challenge,
    identity_provider: config.identityProvider,
  });

  window.location.assign(
    `${config.cognitoDomain}/oauth2/authorize?${params.toString()}`,
  );
}

export async function completeLogin(
  config: {
    cognitoDomain: string;
    cognitoClientId: string;
    redirectUri: string;
  },
  code: string,
  state: string,
): Promise<TokenSet> {
  const expectedState = localStorage.getItem(STATE_KEY);
  const verifier = localStorage.getItem(VERIFIER_KEY);
  if (!expectedState || !verifier || expectedState !== state) {
    throw new Error("Invalid OAuth state");
  }

  const body = new URLSearchParams({
    grant_type: "authorization_code",
    client_id: config.cognitoClientId,
    code,
    redirect_uri: config.redirectUri,
    code_verifier: verifier,
  });

  const response = await fetch(`${config.cognitoDomain}/oauth2/token`, {
    method: "POST",
    headers: { "content-type": "application/x-www-form-urlencoded" },
    body,
  });

  if (!response.ok) {
    throw new Error(`Token exchange failed: ${response.status}`);
  }

  const json = (await response.json()) as {
    access_token: string;
    id_token: string;
    refresh_token?: string;
    expires_in: number;
  };

  localStorage.removeItem(VERIFIER_KEY);
  localStorage.removeItem(STATE_KEY);

  const tokens: TokenSet = {
    accessToken: json.access_token,
    idToken: json.id_token,
    refreshToken: json.refresh_token,
    expiresAt: Date.now() + json.expires_in * 1000,
  };
  storeTokens(tokens);
  return tokens;
}

export function logoutRedirect(config: {
  cognitoDomain: string;
  cognitoClientId: string;
  logoutUri: string;
}): void {
  clearTokens();
  const params = new URLSearchParams({
    client_id: config.cognitoClientId,
    logout_uri: config.logoutUri,
  });
  window.location.assign(
    `${config.cognitoDomain}/logout?${params.toString()}`,
  );
}

export async function getIdToken(): Promise<string> {
  const tokens = getStoredTokens();
  if (!isTokenValid(tokens) || !tokens?.idToken) {
    throw new Error("Not authenticated");
  }
  return tokens.idToken;
}
