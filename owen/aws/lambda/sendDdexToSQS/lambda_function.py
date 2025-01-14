import boto3
import zipfile
import io
import json
import uuid

# Initialize S3 and SQS clients
s3 = boto3.client('s3')
sqs = boto3.client('sqs')

def lambda_handler(event, context):
    try:
        for record in event['Records']:
            # Extract bucket and object details from the event
            bucket_name = record['s3']['bucket']['name']
            object_key = record['s3']['object']['key']

            print(f"Triggered by file: {object_key} in bucket: {bucket_name}")

            # Determine the batch folder
            if "/" in object_key:
                batch_folder = "/".join(object_key.split("/")[:-1]) + "/"
            else:
                batch_folder = ""

            print(f"Batch folder determined: {batch_folder}")

            # List all files in the batch folder
            response = s3.list_objects_v2(Bucket=bucket_name, Prefix=batch_folder)
            files = response.get('Contents', [])

            print(f"Found {len(files)} files in folder: {batch_folder}")

            # If no files are found in the folder, log and return 404
            if not files:
                print(f"No files found in folder: {batch_folder}")
                return {
                    "statusCode": 404,
                    "body": "No files found in folder"
                }

            # Prepare an in-memory zip file
            zip_buffer = io.BytesIO()

            try:
                with zipfile.ZipFile(zip_buffer, 'w') as zip_file:
                    for file in files:
                        file_key = file['Key']

                        # Skip the file that triggered the event (BatchComplete XML file)
                        if file_key == object_key:
                            print(f"Skipping trigger file: {file_key}")
                            continue

                        print(f"Adding file to ZIP: {file_key}")

                        # Get the object from S3 and read its content
                        file_obj = s3.get_object(Bucket=bucket_name, Key=file_key)
                        file_content = file_obj['Body'].read()

                        # Preserve the folder structure in the ZIP
                        relative_path = file_key[len(batch_folder):]  # Relative path inside the batch folder
                        zip_file.writestr(relative_path, file_content)

                # Seek back to the start of the buffer
                zip_buffer.seek(0)

                # Generate a unique key for the ZIP file
                zip_file_key = f"{batch_folder}Batch_{uuid.uuid4().hex}.zip"

                # Upload the ZIP file to S3
                s3.put_object(
                    Bucket=bucket_name,
                    Key=zip_file_key,
                    Body=zip_buffer,
                )
                print(f"Uploaded ZIP file to S3: {bucket_name}/{zip_file_key}")

                # Send SQS message with a reference to the ZIP file
                queue_url = "https://sqs.us-east-1.amazonaws.com/595785979655/OwenInputQueue.fifo"
                sqs_message_body = {
                    "bucket": bucket_name,
                    "zip_file_key": zip_file_key,
                    "original_trigger_file": object_key
                }

                sqs_response = sqs.send_message(
                    QueueUrl=queue_url,
                    MessageBody=json.dumps(sqs_message_body),
                    MessageGroupId="BatchProcessing",
                    MessageDeduplicationId=batch_folder
                )
                print(f"SQS message sent successfully: {sqs_response.get('MessageId')}")

            except Exception as e:
                print(f"Error while processing files or uploading zip: {str(e)}")
                raise e

    except Exception as e:
        print(f"Critical error in Lambda function: {str(e)}")
        raise e

    print("Lambda execution completed successfully.")
    return {
        "statusCode": 200,
        "body": "Zip file created, uploaded to S3, and reference sent to SQS"
    }
