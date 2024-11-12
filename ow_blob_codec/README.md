# WIP! Do not use in production code!

This code compress xml file/files under a given path into the single BLOB that conforms with the requirements for the KZG BLOB described [HERE](https://notes.ethereum.org/@vbuterin/proto_danksharding_faq#What-format-is-blob-data-in-and-how-is-it-committed-to).

It uses `DEFLATE` compression algorithm. After compressing the file it cut received `Vec<u8>` into 31 byte long chunks and append 0 or 1 at the beginning of each. It appends 1 if it's the last chunk in the processed file and 0 for all the other cases.
