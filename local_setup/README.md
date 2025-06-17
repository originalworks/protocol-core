# Environments Overview

## DEV MODE SETUP

Run with:

```bash
local_setup/scripts/run_dev_mode.sh
```

- Deploys contracts with a **fake verifier**
- Configures Validators to run in RISC0_DEV_MODE

## FULL PROVING SETUP (CPU)

Run with:

```bash
local_setup/scripts/run_full_proving_setup.sh
```

- Deploys contracts with a **real verifier**
- Configures Validators to run in CPU mode and produce real ZK proofs

**Warning:** it's not recommended to run multiple validators at once with this setup

## EXPIRED BLOB SETUP

Run with:

```bash
local_setup/scripts/run_expired_blobs_setup.sh
```

- Deploys contracts with a **fake verifier** and modified version of `DdexSequencer` that will accept tx with broken BLOBs
- Configures Validators to run in RISC0_DEV_MODE

---

# How to use a setup

## Sending a Blob to the DdexSequencer

### Send single BLOB

Run the `send_blob.sh` script located at `local_setup/scripts/owen`.

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

### Send all BLOBs

To send all blobs from `/protocol-core/local_setup/blobs` folder in separate transactions:

```bash
./send_all.sh
```

### Send broken BLOB (expired blob testing)

> **Important!**
> This will only work with setup run with `run_expired_blobs_setup.sh`.

To send a broken blob that can not be found on the beacon chain:

```bash
./send_broken_blob.sh
```

---

## Start a Validator

Run the `turn_on.sh` script located at `local_setup/scripts/validator`.

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

Run the `turn_off.sh` script located at `local_setup/scripts/validator`.

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

# Cleanup

Run the `kill.sh` script located at `local_setup/scripts/setup`.

```bash
./kill.sh
```
