use c_kzg::BYTES_PER_FIELD_ELEMENT;
use miniz_oxide::deflate::compress_to_vec;
use std::error::Error;
use std::path::Path;
use std::{fs::File, io::Read};

const RAW_CHUNK_SIZE: usize = BYTES_PER_FIELD_ELEMENT - 1;

pub fn file_to_vec(path: &Path) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut file = File::open(path)?;

    let mut file_buffer = Vec::new();
    file.read_to_end(&mut file_buffer)?;

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
        let path = Path::new("./tests/assets/valid_xml/ERN_example_1.xml");
        let mut file = File::open(path).unwrap();
        let mut file_buffer = Vec::new();
        file.read_to_end(&mut file_buffer).unwrap();

        let encoded = file_to_vec(path).unwrap();

        assert!(encoded.len() < file_buffer.len());
    }
}
