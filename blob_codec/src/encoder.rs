use crate::constants::RAW_CHUNK_SIZE;
use crate::errors::OwCodecError;
use log_macros::loc;
use miniz_oxide::deflate::compress_to_vec;
use std::path::Path;
use std::{fs::File, io::Read};

pub fn file_to_vec(path: &Path) -> Result<Vec<u8>, OwCodecError> {
    let mut file = File::open(path).map_err(|e| OwCodecError::Io {
        source: e,
        path: path.to_string_lossy().to_string(),
        loc: loc!(),
    })?;

    let mut file_buffer = Vec::new();
    file.read_to_end(&mut file_buffer)
        .map_err(|e| OwCodecError::Io {
            source: e,
            path: path.to_string_lossy().to_string(),
            loc: loc!(),
        })?;

    file_buffer = compress_to_vec(&file_buffer, 10);

    let mut encoded = Vec::new();
    let mut cursor = 0;

    for chunk in file_buffer.chunks(RAW_CHUNK_SIZE) {
        if cursor + RAW_CHUNK_SIZE >= file_buffer.len() {
            encoded.push(1u8);
            encoded.extend_from_slice(chunk);
            let cell_padding = RAW_CHUNK_SIZE - chunk.len();
            if cell_padding > 0 {
                encoded.extend(std::iter::repeat(0).take(cell_padding));
            }
        } else {
            encoded.push(0u8);
            encoded.extend_from_slice(chunk);
        }
        cursor += RAW_CHUNK_SIZE;
    }

    Ok(encoded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encoded_file_is_smaller() {
        let path = Path::new("./tests/assets/msg_one.json");
        let mut file = File::open(path).unwrap();
        let mut file_buffer = Vec::new();
        file.read_to_end(&mut file_buffer).unwrap();

        let encoded = file_to_vec(path).unwrap();

        assert!(encoded.len() < file_buffer.len());
    }
}
