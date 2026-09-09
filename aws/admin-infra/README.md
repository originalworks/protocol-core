# OWEN Admin Panel infrastructure

Read-only admin UI over `DdexMessageStatus*` (DynamoDB), authenticated with **IAM Identity Center → Cognito (SAML) → API Gateway JWT → Lambda**.

## Components

| Resource | Purpose |
|----------|---------|
| Cognito User Pool + Hosted UI domain | App IdP bridge (federation only after SAML is wired) |
| HTTP API + `admin_api` Lambda | `GET /messages?status=…`, `GET /messages/{messageFolder+}` |
| S3 + CloudFront | React Admin SPA hosting |

Sibling stack to `owen-infra` — does **not** create the DynamoDB table; pass the existing table name via parameters.

## Prerequisites

- Owen infra already deployed (status table exists)
- `cargo-lambda`, SAM CLI, Node.js 22+, AWS credentials
- Org admin access to **IAM Identity Center** (usually management account)

## Deploy (two-step auth bootstrap)

Identity Center needs Cognito ACS / Entity ID from stack outputs, and Cognito needs the Identity Center SAML metadata URL. Use this order:

### 1. Build the API Lambda

From repo root (Amazon Linux / CI preferred for `provided.al2023`):

```bash
cd aws/admin_api
cargo lambda build --release
```

### 2. First SAM deploy (no SAML yet)

Leave `IdentityCenterSamlMetadataUrl` empty in `template-config-{env}.json`, then:

```bash
cd aws/admin-infra
make deploy-infra-dev   # or stage / prod
```

Ensure `CognitoDomainPrefix` is globally unique.

### 3. Create Identity Center custom SAML application

In the **Identity Center** console (org/management account):

1. **Applications → Add application → Custom SAML 2.0 application**
2. Set:
   - **Application ACS URL** → stack output `CognitoSamlAcsUrl`  
     (`https://{CognitoDomainPrefix}.auth.us-east-1.amazoncognito.com/saml2/idpresponse`)
   - **Application SAML audience** → stack output `CognitoSamlEntityId`  
     (`urn:amazon:cognito:sp:{UserPoolId}`)
3. Attribute mappings (minimum):
   - Subject / NameID → `${user:email}` (or persistent ID if required by policy)
   - `email` → `${user:email}`
4. Assign users/groups who may use OWEN Admin.
5. Copy the application **IAM Identity Center SAML metadata URL**.

### 4. Second SAM deploy (enable federation)

Set `IdentityCenterSamlMetadataUrl` in `template-config-{env}.json` to that metadata URL, then redeploy:

```bash
make deploy-infra-dev
```

This replaces the bootstrap Cognito app client with a federation-only client (`IAMIdentityCenter`). **Client ID changes** — republish the SPA after this step.

### 5. Publish the React Admin SPA

```bash
make publish-spa-dev
```

Open stack output `CloudFrontUrl`. Login redirects through Cognito → Identity Center.

## Local SPA development

```bash
cd admin
cp .env.example .env   # fill from stack outputs
npm ci
npm run dev
```

Use `http://localhost:5173` (already allow-listed on the Cognito app client).

## Stack outputs (reference)

| Output | Use |
|--------|-----|
| `CloudFrontUrl` | Admin UI |
| `ApiEndpoint` | SPA `VITE_API_BASE_URL` |
| `UserPoolId` / `UserPoolClientId` / `CognitoDomain` | SPA Cognito env |
| `CognitoSamlAcsUrl` / `CognitoSamlEntityId` | Identity Center SAML app |
| `SpaBucketName` / `CloudFrontDistributionId` | SPA publish / invalidation |

## Security notes

- Lambda IAM is read-only: `dynamodb:Query`, `dynamodb:GetItem`
- After SAML is enabled, Cognito client only allows `IAMIdentityCenter` (no local password login)
- Access control = Identity Center application assignments
- SPA sends Cognito **ID token** to API Gateway (JWT `aud` = app client id)
