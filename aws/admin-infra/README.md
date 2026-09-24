# OWEN Admin Panel infrastructure

Read-only React Admin UI over `DdexMessageStatus*`, with **no API/Lambda backend**.

Auth and data path:

```text
IAM Identity Center → Cognito User Pool (SAML)
                   → Cognito Identity Pool (temp AWS creds)
                   → browser AWS SDK → DynamoDB (Query / GetItem)
```

SPA is hosted on **S3 + CloudFront**.

## Components

| Resource | Purpose |
|----------|---------|
| Cognito User Pool + Hosted UI | Workforce login via Identity Center SAML |
| Cognito Identity Pool | Exchange Cognito ID token for temporary AWS credentials |
| IAM authenticated role | `dynamodb:Query` + `GetItem` on the status table only |
| S3 + CloudFront | Static React Admin SPA |

Sibling stack to `owen-infra` — does **not** create the DynamoDB table; pass the existing table name via parameters.

## Prerequisites

- Owen infra already deployed (status table exists)
- SAM CLI, Node.js 22+, AWS credentials
- Org admin access to **IAM Identity Center**

## Deploy (two-step auth bootstrap)

### 1. First SAM deploy (no SAML yet)

Leave `IdentityCenterSamlMetadataUrl` empty in `template-config-{env}.json`:

```bash
cd aws/admin-infra
make deploy-infra-dev
```

Ensure `CognitoDomainPrefix` is globally unique.

### 2. Create Identity Center custom SAML application

In the Identity Center console:

1. **Applications → Add application → Custom SAML 2.0 application**
2. Set:
   - **Application ACS URL** → stack output `CognitoSamlAcsUrl`
   - **Application SAML audience** → stack output `CognitoSamlEntityId`
3. Map `email` → `${user:email}` (and NameID as required)
4. Assign users/groups
5. Copy the application **SAML metadata URL**

### 3. Second SAM deploy (enable federation)

Set `IdentityCenterSamlMetadataUrl` in the env config JSON, then:

```bash
make deploy-infra-dev
```

App client ID may change — republish the SPA after this step.

### 4. Publish the SPA

```bash
make publish-spa-dev
```

Open stack output `CloudFrontUrl`.

## Local development

```bash
cd admin
cp .env.example .env   # fill from stack outputs
npm ci
npm run dev
```

## Security notes

- Unauthenticated Identity Pool access is disabled
- Authenticated role is read-only (`Query`, `GetItem`) on the message status table + indexes
- Access control = Identity Center application assignments + Cognito login
- Temporary AWS credentials live in the browser after login (acceptable for this internal tool; tight IAM still required)
