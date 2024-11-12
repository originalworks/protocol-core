use c_kzg::{BYTES_PER_BLOB, BYTES_PER_FIELD_ELEMENT};
use miniz_oxide::inflate::{decompress_to_vec, DecompressError};

pub fn blob_to_vecs(blob: [u8; BYTES_PER_BLOB]) -> Result<Vec<Vec<u8>>, DecompressError> {
    let mut decoded_chunks: Vec<Vec<u8>> = Vec::new();
    let mut encoded_chunk = Vec::new();

    for chunk in blob.chunks(BYTES_PER_FIELD_ELEMENT) {
        encoded_chunk.extend_from_slice(&chunk[1..]);
        if chunk[0] == 1u8 {
            let decoded_chunk = decompress_to_vec(encoded_chunk.as_slice())?;
            decoded_chunks.push(decoded_chunk);
            encoded_chunk.clear();
        }
    }

    Ok(decoded_chunks)
}
