/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_AWS_REGION: string;
  readonly VITE_COGNITO_DOMAIN: string;
  readonly VITE_COGNITO_CLIENT_ID: string;
  readonly VITE_COGNITO_USER_POOL_ID: string;
  readonly VITE_COGNITO_IDENTITY_POOL_ID: string;
  readonly VITE_MESSAGE_STATUS_TABLE_NAME: string;
  readonly VITE_PROCESSING_STATUS_INDEX_NAME: string;
  readonly VITE_REDIRECT_URI: string;
  readonly VITE_LOGOUT_URI: string;
  readonly VITE_IDENTITY_PROVIDER: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
