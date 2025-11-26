# 0. UNDERSTANDING OWEN AND ITS PART IN THE PROTOCOL

OWEN is software developed and maintained by **_The Protocol_**. It processes and sends DDEX messages to the Protocol's `DdexSequencer` smart contract. From there, DDEX messages can be downloaded and verified by other actors who run other Protocol components such as the Validator (`/validator_node`).

<br>

# 1. DEPLOY INFRASTRUCTURE ON AWS

## Prerequisites

- AWS account and credentials with proper rights to create resources in Cloud Formation
- AWS CLI installed [INSTRUCTIONS](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html)
- AWS SAM CLI installed [INSTRUCTIONS](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/install-sam-cli.html)
- rust and cargo installed [INSTRUCTIONS](https://www.rust-lang.org/tools/install)
- `cargo-lambda` installed [INSTRUCTIONS](https://www.cargo-lambda.info/guide/getting-started.html)

## Steps

1. In your [AWS Console](https://console.aws.amazon.com/) manually create a bucket named `owen-artifacts` to store your deployment artifacts
2. On your local machine login to AWS (e.g. `aws configure` or `aws sso login (...)`)
3. Inside `owen/aws/deploy-infra` run `make deploy-infra-prod`.

<br>

# 2. CONFIGURE AWS INFRASTRUCTURE BEFORE USE

## Scaling

The OWEN infrastructure stack was designed to scale easily. In file `aws/owen-infra/template.yml` you can find three resources that looks almost the same: `OwenOne`, `OwenTwo` and `OwenThree`. These are Lambda instances running the OWEN binary in parallel. You can add or remove them as needed. You can also change the value of `LambdaScheduleRateMin` to configure how often the Lambda is triggered — the lowest recommended value is around 8 minutes.

## Available settings

### Account Abstraction

By default, the OWEN infrastructure is configured so that all DDEX messages are batched and sent from one address. Set this address in `aws/owen-infra/template-config-prod.json` => `SeoaAddress`. This should be the address of your EOA delegated to our smart account implementation. Instructions on how to perform this delegation can be found here: TOADD

### KMS integration

Several private keys are required for this infrastructure to operate:

- one for each OWEN instance (`OwenOne`, `OwenTwo`, etc.) — used for signing requests to the Protocol’s IPFS bridge
- one for sending transactions to the blockchain with the batched payload from each instance

By default, all of these keys are created during deployment as dedicated KMS key pairs. This maintains a clear separation of responsibilities but requires additional configuration before use.

**_Use without KMS_** <br>
If you prefer not to use separate KMS keys for each instance and for the batch sender, change `USE_KMS` to `false` in:

- `aws/owen-infra/resources/owen.yml`
- `aws/owen-infra/resources/owen-blobs-queue.yml`

Then store a single private key in Secrets Manager (see `Setting Secrets`).

However:

- you must still register the associated public key in the Protocol’s whitelist
- you must still grant it the `BLOB_SENDER_ROLE`. See here: TOADD

**_Use with KMS_** <br>
If you choose to use KMS, complete the following additional steps:
Because OWEN instances sign requests to the Protocol's IPFS bridge using unique KMS private keys, each corresponding public address must be registered on the Protocol’s whitelist.

1. In your [AWS Console](https://console.aws.amazon.com/) go to: `KMS → Customer managed keys`. Locate the keys associated with your OWEN Lambdas (visible in resource descriptions) and with `BlobsBatchSender`. Alternatively, inspect the Lambdas’ `SIGNER_KMS_ID` environment variable.
2. Resolve each key locally to obtain its public key: [INSTRUCTIONS](https://luhenning.medium.com/the-dark-side-of-the-elliptic-curve-signing-ethereum-transactions-with-aws-kms-in-javascript-83610d9a6f81)
3. Send all public addresses of OWEN to The Protocol so we can register them
4. If not done already during sEOA delegation, grant `BLOB_SENDER_ROLE` to the public address of `BlobsBatchSender`.

## Setting Secrets

OWEN needs RPC access to operate, and RPC URLs can be abused, so they must be stored in AWS Secrets Manager.
Open the Console → Secrets Manager → find `OwenLambdaSecretsProd`.
Set:

- `RPC_URL`
- (optional when you use default `USE_KMS=false`) `PRIVATE_KEY`

## Trigger file

To enqueue a message for processing, upload it to the S3 bucket: `ddex-messages-prod`.
Each message should be placed in a separate folder; optional assets such as images can go in subfolders.

To trigger processing, include a special file in the message folder whose filename starts with `BatchComplete_`.
This file may be empty (0 bytes) and must be uploaded last to avoid early triggering.

You can customize this naming pattern in:
`aws/owen-infra/template-config-prod.json → TriggerFilePattern`

## Funding your account

To send transaction with DDEX messsages to the blockchain network you need some funds to cover gas.
Depending on which option you choose, it will be either:

- the KMS public account associated with `BlobsBatchSender`, or
- the public account of your `PRIVATE_KEY` set in secret manager.

<br>

# 3. HOW TO USE IT TO SEND DDEX MESSAGES

After deploying and configuring your stack, and after assigning the correct roles to your sEOA, you can begin sending DDEX messages. Upload each message (in its own folder) to the `ddex-messages-prod` bucket.
Make sure each folder includes a trigger file as described earlier in `Trigger file` section.

You can monitor processing progress in the DynamoDB table (default name: `DdexMessageStatusProd`).
