# OWEN Admin (React Admin)

Read-only UI for DDEX message processing status.

## Setup

1. Deploy [`aws/admin-infra`](../aws/admin-infra) and complete Identity Center SAML wiring (see that README).
2. Copy `.env.example` → `.env` and fill values from stack outputs (or use `make publish-spa-*` which injects them).

```bash
npm ci
npm run dev
```

## Auth

Login uses Cognito Hosted UI with PKCE and `identity_provider=IAMIdentityCenter`, so users authenticate with the existing workforce Identity Center directory.
