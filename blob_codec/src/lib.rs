pub mod constants;
mod decoder;
mod encoder;
pub mod errors;
use constants::BYTES_PER_BLOB;
use decoder::blob_to_vecs;
use errors::OwCodecError;
use log_macros::loc;
use sha2::{Digest as _, Sha256};
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct BlobCodec {
    blob_bytes: [u8; BYTES_PER_BLOB],
    initialized: bool,
}

#[derive(Debug)]
pub struct CalldataLimitConfig {
    max_calldata_size: u64,
    ratio: f64,
}

impl CalldataLimitConfig {
    pub fn default() -> Self {
        Self {
            max_calldata_size: 130472, // 131072 (max limit for most rpc providers) - 600 (constant tx cost + constant args)
            ratio: 0.5,
        }
    }

    pub fn new(max_calldata_size: u64, ratio: f64) -> Self {
        Self {
            max_calldata_size,
            ratio,
        }
    }

    pub fn check_calldata_size(
        &self,
        calldata_size: u64,
        file_path: &Path,
    ) -> Result<u64, OwCodecError> {
        let metadata = std::fs::metadata(file_path).map_err(|_e| OwCodecError::NotAFile {
            path: file_path.to_string_lossy().to_string(),
            loc: loc!(),
        })?;

        let new_calldata_size = calldata_size + (metadata.len() as f64 * self.ratio).ceil() as u64;

        if new_calldata_size > self.max_calldata_size {
            return Err(OwCodecError::CalldataOverflow {
                limit: self.max_calldata_size,
                loc: loc!(),
            });
        }

        return Ok(new_calldata_size);
    }
}

impl BlobCodec {
    pub fn from_file(path: &str) -> Result<Self, OwCodecError> {
        let path = Path::new(path);
        let mut kzg_blob: [u8; BYTES_PER_BLOB] = [0; BYTES_PER_BLOB];
        let mut blob_cursor = 0;

        if path.is_file() {
            append_to_blob(&mut kzg_blob, path, &mut blob_cursor)?;
        } else {
            return Err(OwCodecError::NotAFile {
                path: path.to_string_lossy().to_string(),
                loc: loc!(),
            });
        }
        Ok(Self {
            blob_bytes: kzg_blob,
            initialized: true,
        })
    }

    pub fn from_dir(path: &str, config: Option<CalldataLimitConfig>) -> Result<Self, OwCodecError> {
        let dir = Path::new(path);
        let mut kzg_blob: [u8; BYTES_PER_BLOB] = [0; BYTES_PER_BLOB];
        let mut blob_cursor = 0;
        let mut calldata_size = 0;

        if dir.is_dir() {
            let files = fs::read_dir(dir).map_err(|e| OwCodecError::Io {
                source: e,
                path: dir.to_string_lossy().to_string(),
                loc: loc!(),
            })?;
            let mut empty_folder = true;
            for file_entry in files {
                let path = file_entry
                    .map_err(|e| OwCodecError::Io {
                        source: e,
                        path: dir.to_string_lossy().to_string(),
                        loc: loc!(),
                    })?
                    .path();

                if path.is_file() && path.extension().unwrap() == "json" {
                    if let Some(calldata_check) = &config {
                        calldata_size = calldata_check.check_calldata_size(calldata_size, &path)?;
                    }
                    append_to_blob(&mut kzg_blob, &path, &mut blob_cursor)?;
                    empty_folder = false;
                }
            }
            if empty_folder {
                return Err(OwCodecError::EmptyDirectory {
                    path: dir.to_string_lossy().to_string(),
                    loc: loc!(),
                });
            }
        } else {
            return Err(OwCodecError::NotADirectory {
                path: dir.to_string_lossy().to_string(),
                loc: loc!(),
            });
        }
        Ok(Self {
            blob_bytes: kzg_blob,
            initialized: true,
        })
    }

    pub fn from_vec(vec: Vec<u8>) -> Result<Self, OwCodecError> {
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

    pub fn from_bytes(bytes: [u8; BYTES_PER_BLOB]) -> Self {
        Self {
            blob_bytes: bytes,
            initialized: true,
        }
    }

    pub fn digest(&self) -> [u8; 32] {
        if !self.initialized {
            panic!("Not initialized");
        }
        Sha256::digest(&self.blob_bytes.as_slice()).into()
    }

    pub fn decode(self) -> Result<Vec<Vec<u8>>, OwCodecError> {
        if !self.initialized {
            panic!("Not initialized");
        }

        blob_to_vecs(self.blob_bytes).map_err(|e| OwCodecError::Decompress {
            msg: e.to_string(),
            loc: loc!(),
        })
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
) -> Result<&'a mut [u8; BYTES_PER_BLOB], OwCodecError> {
    let compressed_file = encoder::file_to_vec(path)?;
    let compressed_file_len = compressed_file.len();
    if compressed_file_len + *cursor > BYTES_PER_BLOB {
        return Err(OwCodecError::BlobOverflow {
            path: path.to_string_lossy().to_string(),
            loc: loc!(),
        });
    }
    if compressed_file_len == 0 {
        return Err(OwCodecError::EmptyFile {
            path: path.to_string_lossy().to_string(),
            loc: loc!(),
        });
    }
    kzg_blob[*cursor..*cursor + compressed_file.len()].copy_from_slice(&compressed_file);
    *cursor += compressed_file.len();
    Ok(kzg_blob)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::io::Write;

    fn generate_test_json(sizes_in_kb: Vec<usize>) -> impl FnOnce() -> std::io::Result<()> {
        let mut paths = Vec::new();

        for (i, size) in sizes_in_kb.iter().enumerate() {
            let target_size = size * 1024 / 2;
            let mut data = vec![];
            let mut current_size = 0;

            while current_size < target_size {
                let item = json!({
                    "id": "1",
                    "name": "Dua Lipa"
                });

                let item_size = serde_json::to_string(&item).unwrap().len();
                current_size += item_size;

                data.push(item);
            }

            let json_string = serde_json::to_string_pretty(&json!({ "items": data })).unwrap();
            let filename = format!("test_{}.json", i);
            let path = std::path::PathBuf::from(format!("./tests/assets/{}", filename));
            let mut file = std::fs::File::create(&path).unwrap();
            file.write_all(json_string.as_bytes()).unwrap();
            paths.push(path);
        }

        let cleanup = move || {
            for path in paths {
                if path.exists() {
                    std::fs::remove_file(&path)?;
                }
            }
            Ok(())
        };

        cleanup
    }

    #[test]
    #[should_panic]
    fn panic_for_empty_dir() {
        let empty_dir_path = "./tests/empty_dir";
        fs::remove_dir_all(&empty_dir_path).unwrap();
        fs::create_dir_all(&empty_dir_path).unwrap();
        BlobCodec::from_dir(&empty_dir_path, None).unwrap();
        fs::remove_dir_all(&empty_dir_path).unwrap();
    }

    #[test]
    #[should_panic]
    fn panic_for_non_dir() {
        BlobCodec::from_dir("./tests/assets/pierogi", None).unwrap();
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
    fn encode_dir_files_into_blob_no_config() {
        let blob = BlobCodec::from_dir("./tests/assets", None).unwrap();

        assert_ne!(blob.to_bytes(), [0; BYTES_PER_BLOB]);
    }

    #[test]
    fn encode_dir_files_into_blob_default_config() {
        assert_eq!(
            BlobCodec::from_dir("./tests/assets", Some(CalldataLimitConfig::default())).is_err(),
            false
        );
        let cleanup = generate_test_json(vec![125, 100, 50, 25]);

        let error_msg =
            "Adding a file would cause to exceed limit of 130472 bytes in validator calldata."
                .to_string();

        assert_eq!(
            BlobCodec::from_dir("./tests/assets", Some(CalldataLimitConfig::default()))
                .unwrap_err()
                .to_string(),
            error_msg
        );

        cleanup().unwrap();
    }

    #[test]
    fn encode_dir_files_into_blob_custom_config() {
        assert_eq!(
            BlobCodec::from_dir("./tests/assets", Some(CalldataLimitConfig::new(20000, 1.0)))
                .is_err(),
            false
        );
        let cleanup = generate_test_json(vec![10]);

        let error_msg =
            "Adding a file would cause to exceed limit of 20000 bytes in validator calldata."
                .to_string();

        assert_eq!(
            BlobCodec::from_dir("./tests/assets", Some(CalldataLimitConfig::new(20000, 1.0)))
                .unwrap_err()
                .to_string(),
            error_msg
        );

        cleanup().unwrap();
    }
}
