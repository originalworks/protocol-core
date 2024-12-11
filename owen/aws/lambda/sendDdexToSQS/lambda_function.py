import boto3
import zipfile
import io
import json

s3 = boto3.client('s3')
sqs = boto3.client('sqs')

def lambda_handler(event, context):
    for record in event['Records']:
        bucket_name = record['s3']['bucket']['name']
        object_key = record['s3']['object']['key']
        
        print(f"Triggered by file: {object_key} in bucket: {bucket_name}")
        
        if "/" in object_key:
            batch_folder = "/".join(object_key.split("/")[:-1]) + "/"
        else:
            batch_folder = ""

        print(f"Batch folder: {batch_folder}")

        response = s3.list_objects_v2(Bucket=bucket_name, Prefix=batch_folder)
        files = response.get('Contents', [])
        
        if not files:
            print(f"No files found in folder: {batch_folder}")
            return {
                "statusCode": 404,
                "body": "No files found in folder"
            }

        zip_buffer = io.BytesIO()
        with zipfile.ZipFile(zip_buffer, 'w') as zip_file:
            for file in files:
                file_key = file['Key']
                if file_key == object_key:
                    continue
                file_obj = s3.get_object(Bucket=bucket_name, Key=file_key)
                file_content = file_obj['Body'].read()
                zip_file.writestr(file_key.split('/')[-1], file_content)
        
        zip_buffer.seek(0)
        zip_binary = zip_buffer.getvalue()

        queue_url = "YOUR_SQS_QUEUE_URL"  # Replace with the OwenInputQueue.fifo URL
        sqs.send_message(
            QueueUrl=queue_url,
            MessageBody=json.dumps({
                "bucket": bucket_name,
                "zip_file": zip_binary.hex()
            }),
            MessageGroupId="BatchProcessing",
            MessageDeduplicationId=batch_folder
        )
        print(f"Zip file sent to SQS successfully for folder: {batch_folder}")

    return {
        "statusCode": 200,
        "body": "Zip file created and sent to SQS"
    }