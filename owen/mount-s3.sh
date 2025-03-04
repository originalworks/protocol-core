#!/usr/bin/env bash
set -e

# Load environment variables from .env (ignoring lines that start with #)
if [ -f .env ]; then
  export $(grep -v '^#' .env | xargs)
fi

# Create the s3fs password file
echo "${AWS_ACCESS_KEY_ID}:${AWS_SECRET_ACCESS_KEY}" > /etc/passwd-s3fs
chmod 600 /etc/passwd-s3fs

# Mount the S3 bucket
s3fs "${S3_BUCKET_NAME}" /protocol-core/owen/input \
  -o passwd_file=/etc/passwd-s3fs \
  -o url="https://s3.${AWS_DEFAULT_REGION}.amazonaws.com" \
  -o use_path_request_style

# Execute the container's main process (whatever CMD was specified)
exec "$@"
