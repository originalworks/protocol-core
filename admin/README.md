# OWEN Admin (React Admin)

Read-only UI for DDEX message processing status. Talks to DynamoDB **directly** from the browser via Cognito Identity Pool credentials (no admin API Lambda).

## Setup

1. Deploy [`aws/admin-infra`](../aws/admin-infra) and complete Identity Center SAML wiring.
2. Copy `.env.example` → `.env` and fill values from stack outputs (or use `make publish-spa-*`).

```bash
npm ci
npm run dev
```

## Auth + data

1. Login: Cognito Hosted UI + PKCE with `identity_provider=IAMIdentityCenter`
2. Data: Cognito ID token → Identity Pool → temporary AWS credentials → `@aws-sdk/lib-dynamodb` `Query` / `GetItem`
