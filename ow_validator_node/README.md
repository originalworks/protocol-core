# WIP!!!

This is still very much pre-alfa early stage sketch-draft prototype!!!

This package currently do:

1. Read the BLOB associated with queue head of the `DdexSequencer`
2. Decode the BLOB and use `circuit_mock` to extract `isrc` and `GRid`
3. Send `isrc` and `GRid` as events to `DdexSequencer`

### TODO:

- ~~Observe DDEX MESSAGE SEQUENCER contract and listen to the events~~
- check if there is an access on IPFS to the added CID of graphic files inside DDEX message
- BLOB processing:
  - create ZK proof for either successful BLOB processing or for incompatible BLOB
  - extract key data from DDEX messages packed into the BLOB
- pin BLOB to IPFS
- pin each DDEX message to IPFS
- ~~prepare and send transaction to DDEX MESSAGE SEQUENCER~~
- tests!
