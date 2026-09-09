import type { AuthProvider } from "react-admin";
import { loadConfig } from "../config";
import {
  beginLogin,
  clearTokens,
  completeLogin,
  getStoredTokens,
  isTokenValid,
  logoutRedirect,
} from "./cognito";

const config = loadConfig();

export const authProvider: AuthProvider = {
  login: async () => {
    await beginLogin(config);
    return Promise.reject();
  },
  logout: async () => {
    logoutRedirect(config);
  },
  checkAuth: async () => {
    if (window.location.pathname === "/auth-callback") {
      return Promise.resolve();
    }
    if (!isTokenValid(getStoredTokens())) {
      return Promise.reject();
    }
    return Promise.resolve();
  },
  checkError: async (error: { status?: number }) => {
    if (error?.status === 401 || error?.status === 403) {
      clearTokens();
      return Promise.reject();
    }
    return Promise.resolve();
  },
  getPermissions: async () => Promise.resolve(),
  getIdentity: async () => {
    const tokens = getStoredTokens();
    if (!tokens?.idToken) {
      throw new Error("Not authenticated");
    }
    const payload = JSON.parse(atob(tokens.idToken.split(".")[1] ?? "{}")) as {
      email?: string;
      sub?: string;
    };
    return {
      id: payload.sub ?? "user",
      fullName: payload.email ?? payload.sub ?? "User",
    };
  },
  handleCallback: async () => {
    const url = new URL(window.location.href);
    const code = url.searchParams.get("code");
    const state = url.searchParams.get("state");
    if (!code || !state) {
      throw new Error("Missing OAuth callback parameters");
    }
    await completeLogin(config, code, state);
  },
};
