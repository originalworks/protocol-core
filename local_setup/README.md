# Development Environment Guide

## Prepare the Environment

Run the `run.sh` script located at `local_setup/dev_mode_network/scripts/setup`.

### What it does:

1. Launches Kurtosis
2. Prepares all environment variables
3. Deploys contracts with a **fake verifier**
4. Builds OWEN and the validator
5. Starts OWEN and validators dedicated containers and IPFS

```bash
./run.sh
```

---

## Send a Blob

Run the `send_blob.sh` script located at `local_setup/dev_mode_network/scripts/owen`.

You can set the `BLOB_FOLDER` variable to specify the path to the blob folder.  
If not set, it defaults to `/protocol-core/local_setup/blobs/blob_one`.

> **Important:**  
> The script runs OWEN inside the `owen` container, where the absolute path to the project root is `/protocol-core`.

To send a blob with messages from `/protocol-core/local_setup/blobs/blob_two`:

```bash
BLOB_FOLDER=/protocol-core/local_setup/blobs/blob_two ./send_blob.sh
```

To send a blob with messages from the default folder `/protocol-core/local_setup/blobs/blob_one`:

```bash
./send_blob.sh
```

---

## Start a Validator in RISC0_DEV_MODE

Run the `turn_on.sh` script located at `local_setup/dev_mode_network/scripts/validator`.

You can set the `VALIDATOR` variable to specify which validator to start.  
Available values: `1`, `2`, `3`  
If not set, it defaults to `1`.

To start `validator_node_1`:

```bash
./turn_on.sh
```

To start `validator_node_3`:

```bash
VALIDATOR=3 ./turn_on.sh
```

---

## Turn Off a Validator

Run the `turn_off.sh` script located at `local_setup/dev_mode_network/scripts/validator`.

You can set the `VALIDATOR` variable to specify which validator to stop.  
Available values: `1`, `2`, `3`  
If not set, it defaults to `1`.

To turn off `validator_node_1`:

```bash
./turn_off.sh
```

To turn off `validator_node_3`:

```bash
VALIDATOR=3 ./turn_off.sh
```

---

## Cleanup

Run the `kill.sh` script located at `local_setup/dev_mode_network/scripts/setup`.

```bash
./kill.sh
```
