#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOCKER_FOLDER=$SCRIPT_DIR/../../docker



BLOB_FOLDER="/protocol-core/local_setup/blobs/blob_one" $SCRIPT_DIR/send_blob.sh
BLOB_FOLDER="/protocol-core/local_setup/blobs/blob_two" $SCRIPT_DIR/send_blob.sh
BLOB_FOLDER="/protocol-core/local_setup/blobs/blob_three" $SCRIPT_DIR/send_blob.sh
BLOB_FOLDER="/protocol-core/local_setup/blobs/blob_four" $SCRIPT_DIR/send_blob.sh
BLOB_FOLDER="/protocol-core/local_setup/blobs/blob_five" $SCRIPT_DIR/send_blob.sh
BLOB_FOLDER="/protocol-core/local_setup/blobs/blob_six" $SCRIPT_DIR/send_blob.sh
BLOB_FOLDER="/protocol-core/local_setup/blobs/blob_seven" $SCRIPT_DIR/send_blob.sh
BLOB_FOLDER="/protocol-core/local_setup/blobs/blob_eight" $SCRIPT_DIR/send_blob.sh
BLOB_FOLDER="/protocol-core/local_setup/blobs/blob_nine" $SCRIPT_DIR/send_blob.sh
BLOB_FOLDER="/protocol-core/local_setup/blobs/blob_ten" $SCRIPT_DIR/send_blob.sh
BLOB_FOLDER="/protocol-core/local_setup/blobs/blob_eleven" $SCRIPT_DIR/send_blob.sh
BLOB_FOLDER="/protocol-core/local_setup/blobs/blob_twelve" $SCRIPT_DIR/send_blob.sh
BLOB_FOLDER="/protocol-core/local_setup/blobs/blob_thirteen" $SCRIPT_DIR/send_blob.sh