# How to use scripts

## auth.sh

Prerequisite to interact with our AWS. Without running it all other scripts will fail
1. `export AWS_PROFILE=your-profile-to-rev-aws`
2. `. ./auth.sh` and enter key from authenticator

## py_deploy.sh
- builds owen zip file (using ../bundle.sh)
- updates owen_cli layer with it
- updates aws_cli_test lambda to use new layer version

## trigger.sh
`S3.json` contains list of ddex messages stored in revelator s3 bucket that can be processed by owen.
`to_process.json` is an input file with ddex messages that will be processed. Select as many messages as you wish from `S3.json` and paste them to `to_process.json` and clear it after they are processed.
`template.json` is an template event that will be sent to lambda. `trigger.sh` iterates over `to_process.json`, fills the `template.json` and triggers lambda.

