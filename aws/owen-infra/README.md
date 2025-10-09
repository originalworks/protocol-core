# HOW TO DEPLOY INFRASTRUCTURE ON AWS

## Prerequisites

- AWS account and credentials with proper rights to create resources in Cloud Formation
- AWS CLI installed [INSTRUCTIONS](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html)
- AWS SAM CLI installed [INSTRUCTIONS](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/install-sam-cli.html)
- rust and cargo installed [INSTRUCTIONS](https://www.rust-lang.org/tools/install)
- `cargo-lambda` installed [INSTRUCTIONS](https://www.cargo-lambda.info/guide/getting-started.html)

## Steps

1. In your [AWS Console](https://console.aws.amazon.com/) manually create a bucket named `owen-artifacts` to store your deployment artifacts
2. On your local machine login to aws with `aws configure`
3. Inside `owen/aws/deploy-infra` run `make deploy-infra-stage`.
4. Go again to your [AWS Console](https://console.aws.amazon.com/) => AWS Secret Manager and set your secrets (private key, rpc url etc.) there.
