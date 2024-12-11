# sendDdexToSQS Lambda Function

The `sendDdexToSQS` Lambda function processes batches of files uploaded to an S3 bucket and prepares them for further processing by:
1. Packaging all files (except `BatchComplete*.xml`) in the folder into a single zip file.
2. Sending the zip file to an SQS FIFO queue (`OwenInputQueue.fifo`) for downstream processing.

## Workflow

1. Triggered by an S3 event when a `BatchComplete*.xml` file is uploaded.
2. Identifies the folder containing the `BatchComplete*.xml` file.
3. Lists all files in the folder.
4. Creates a zip archive of the folder contents (excluding the `BatchComplete*.xml` file).
5. Sends the zip file to the SQS FIFO queue with metadata for ordered and deduplicated processing.

## Prerequisites

- **S3 Bucket**: Ensure the bucket exists and is configured to trigger this Lambda on `BatchComplete*.xml` uploads.
- **SQS FIFO Queue**: Create the queue and note its URL.
- **IAM Role**: The Lambda's execution role requires:
  - `AmazonS3ReadOnlyAccess`
  - `AmazonSQSFullAccess`

## Variables

- `QUEUE_URL`: The URL of the SQS FIFO queue (`OwenInputQueue.fifo`).

## Deployment

1. **Create the Lambda**:
   - Set the runtime to Python 3.9+.
   - Paste the code into the function editor or deploy via CLI.

2. **Configure the S3 Trigger**:
   - Set the bucket to trigger this Lambda on `BatchComplete*.xml` uploads.

3. **Test**:
   - Upload files to the S3 bucket, including a `BatchComplete*.xml`.
   - Verify that a message with the zip file is sent to the SQS FIFO queue.

## Output

- Sends an SQS message with:
  - The S3 bucket name.
  - The zip file (encoded as a string).
  - Metadata (`MessageGroupId`, `MessageDeduplicationId`) for ordered processing.
