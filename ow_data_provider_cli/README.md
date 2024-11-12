# WIP!!!

This is still very much pre-alfa early stage sketch-draft prototype!!!

To send `.xml` files as a BLOB using EIP4844 transaction:

1. Create `.env` file in the project root with `RPC_URL` and `PRIVATE_KEY` values
2. Run command:
   ```
   cargo run ./dir-with-xml-files
   ```

### TODO:

- ~~pack multiple messages into BLOB~~
- ~~prepare and send transaction to DDEX MESSAGE SEQUENCER~~
- upload media files to IPFS and include CID in DDEX message before encoding it into BLOB
- either upload audio files to private ipfs node or only generate CID of audio and then include the CID in the DDEX message
- generate ISCC of audio file and include it
- manage stake
- write tests!
