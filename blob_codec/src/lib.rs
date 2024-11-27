pub mod constants;
mod decoder;
mod encoder;
pub mod errors;
use constants::BYTES_PER_BLOB;
use decoder::blob_to_vecs;
use errors::OwCodecError;
use sha2::{Digest as _, Sha256};
use std::error::Error;
use std::fs;
use std::path::Path;

pub struct BlobCodec {
    blob_bytes: [u8; BYTES_PER_BLOB],
    initialized: bool,
}

impl BlobCodec {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let path = Path::new(path);
        let mut kzg_blob: [u8; BYTES_PER_BLOB] = [0; BYTES_PER_BLOB];
        let mut blob_cursor = 0;

        if path.is_file() {
            append_to_blob(&mut kzg_blob, path, &mut blob_cursor)?;
        } else {
            return Err(Box::new(OwCodecError::NotAFile(
                path.to_string_lossy().to_string(),
            )));
        }
        Ok(Self {
            blob_bytes: kzg_blob,
            initialized: true,
        })
    }

    pub fn from_dir(path: &str) -> Result<Self, Box<dyn Error>> {
        let dir = Path::new(path);
        let mut kzg_blob: [u8; BYTES_PER_BLOB] = [0; BYTES_PER_BLOB];
        let mut blob_cursor = 0;
        if dir.is_dir() {
            let files = fs::read_dir(dir)?;
            let mut empty_folder = true;
            for file_entry in files {
                let path = file_entry?.path();
                if path.is_file() {
                    append_to_blob(&mut kzg_blob, &path, &mut blob_cursor)?;
                    empty_folder = false;
                }
            }
            if empty_folder {
                return Err(Box::new(OwCodecError::EmptyDirectory(
                    dir.to_string_lossy().to_string(),
                )));
            }
        } else {
            return Err(Box::new(OwCodecError::NotADirectory(
                dir.to_string_lossy().to_string(),
            )));
        }
        Ok(Self {
            blob_bytes: kzg_blob,
            initialized: true,
        })
    }

    pub fn from_vec(vec: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            blob_bytes: vec.try_into().unwrap_or_else(|v: Vec<u8>| {
                panic!(
                    "Expected a Vec of length {} but it was {}",
                    BYTES_PER_BLOB,
                    v.len()
                )
            }),
            initialized: true,
        })
    }

    pub fn digest(self) -> [u8; 32] {
        if !self.initialized {
            panic!("Not initialized");
        }
        Sha256::digest(self.blob_bytes.as_slice()).into()
    }

    pub fn decode(self) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
        if !self.initialized {
            panic!("Not initialized");
        }

        match blob_to_vecs(self.blob_bytes) {
            Ok(res) => return Ok(res),
            Err(err) => return Err(err.to_string().into()),
        }
    }

    pub fn to_bytes(self) -> [u8; BYTES_PER_BLOB] {
        if !self.initialized {
            panic!("Not initialized");
        }

        self.blob_bytes
    }
}

fn append_to_blob<'a>(
    kzg_blob: &'a mut [u8; BYTES_PER_BLOB],
    path: &Path,
    cursor: &'a mut usize,
) -> Result<&'a mut [u8; BYTES_PER_BLOB], Box<dyn Error>> {
    let compressed_file = encoder::file_to_vec(path)?;
    let compressed_file_len = compressed_file.len();
    if compressed_file_len + *cursor > BYTES_PER_BLOB {
        return Err(Box::new(OwCodecError::BlobOverflowError(
            path.to_string_lossy().to_string(),
        )));
    }
    if compressed_file_len == 0 {
        return Err(Box::new(OwCodecError::EmptyFile(
            path.to_string_lossy().to_string(),
        )));
    }
    kzg_blob[*cursor..*cursor + compressed_file.len()].copy_from_slice(&compressed_file);
    *cursor += compressed_file.len();
    Ok(kzg_blob)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn panic_for_empty_dir() {
        let empty_dir_path = "./tests/empty_dir";
        fs::remove_dir_all(&empty_dir_path).unwrap();
        fs::create_dir_all(&empty_dir_path).unwrap();
        BlobCodec::from_dir(&empty_dir_path).unwrap();
        fs::remove_dir_all(&empty_dir_path).unwrap();
    }

    #[test]
    #[should_panic]
    fn panic_for_non_dir() {
        BlobCodec::from_dir("./tests/assets/pierogi").unwrap();
    }

    #[test]
    #[should_panic]
    fn panic_for_non_file() {
        BlobCodec::from_file("./tests/assets/test.txt").unwrap();
    }

    #[test]
    #[should_panic]
    fn panic_for_incorrect_vec() {
        let faulty_vec: Vec<u8> = vec![1, 2, 3];
        BlobCodec::from_vec(faulty_vec).unwrap();
    }

    #[test]
    fn encode_file_into_blob() {
        let blob = BlobCodec::from_file("./tests/assets/msg_one.json").unwrap();

        assert_ne!(blob.to_bytes(), [0; BYTES_PER_BLOB]);
    }

    #[test]
    fn encode_dir_files_into_blob() {
        let blob = BlobCodec::from_dir("./tests/assets").unwrap();

        assert_ne!(blob.to_bytes(), [0; BYTES_PER_BLOB]);
    }
}
