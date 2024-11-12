use c_kzg::BYTES_PER_BLOB;
use errors::OwCodecError;
use std::error::Error;
use std::fs;
use std::path::Path;
pub mod decoder;
pub mod encoder;
pub mod errors;
pub mod validator;

fn append_to_blob<'a>(
    kzg_blob: &'a mut [u8; BYTES_PER_BLOB],
    path: &Path,
    cursor: &'a mut usize,
) -> Result<&'a mut [u8; BYTES_PER_BLOB], Box<dyn Error>> {
    let compressed_xml = encoder::file_to_vec(path)?;
    if compressed_xml.len() + *cursor > BYTES_PER_BLOB {
        return Err(Box::new(OwCodecError::BlobOverflowError(
            path.to_string_lossy().to_string(),
        )));
    }
    kzg_blob[*cursor..*cursor + compressed_xml.len()].copy_from_slice(&compressed_xml);
    *cursor += compressed_xml.len();
    Ok(kzg_blob)
}

fn extract_file_extension(path: &Path) -> Result<&str, Box<dyn Error>> {
    let extension = path
        .extension()
        .ok_or_else(|| {
            Box::new(OwCodecError::FileExtensionExtraction(
                path.to_string_lossy().to_string(),
            ))
        })?
        .to_str()
        .ok_or_else(|| {
            Box::new(OwCodecError::FileExtensionExtraction(
                path.to_string_lossy().to_string(),
            ))
        })?;
    Ok(extension)
}

pub fn blob_from_dir(path: &str) -> Result<[u8; BYTES_PER_BLOB], Box<dyn Error>> {
    let dir = Path::new(path);
    let mut kzg_blob: [u8; BYTES_PER_BLOB] = [0; BYTES_PER_BLOB];
    let mut blob_cursor = 0;
    if dir.is_dir() {
        let files = fs::read_dir(dir)?;
        let mut empty_folder = true;
        for file_entry in files {
            let path = file_entry?.path();
            let file_extension = extract_file_extension(&path)?;
            if path.is_file() && file_extension == "xml" {
                validator::validate_xml(&path)?;
                append_to_blob(&mut kzg_blob, &path, &mut blob_cursor)?;
                empty_folder = false;
            }
        }
        if empty_folder {
            return Err(Box::new(OwCodecError::NoXmlInFolder(
                dir.to_string_lossy().to_string(),
            )));
        }
    } else {
        return Err(Box::new(OwCodecError::NotADirectory(
            dir.to_string_lossy().to_string(),
        )));
    }
    Ok(kzg_blob)
}

pub fn blob_from_file(path: &str) -> Result<[u8; BYTES_PER_BLOB], Box<dyn Error>> {
    let path = Path::new(path);
    let mut kzg_blob: [u8; BYTES_PER_BLOB] = [0; BYTES_PER_BLOB];
    let mut blob_cursor = 0;
    let file_extension = extract_file_extension(&path)?;

    if path.is_file() && file_extension == "xml" {
        validator::validate_xml(&path)?;
        append_to_blob(&mut kzg_blob, path, &mut blob_cursor)?;
    } else {
        return Err(Box::new(OwCodecError::NotAnXmlFile(
            path.to_string_lossy().to_string(),
        )));
    }
    Ok(kzg_blob)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn panic_for_empty_dir() {
        blob_from_dir("./tests/assets").unwrap();
    }

    #[test]
    #[should_panic]
    fn panic_for_non_dir() {
        blob_from_dir("./tests/assets/pierogi").unwrap();
    }

    #[test]
    #[should_panic]
    fn panic_for_non_xml_file() {
        blob_from_file("./tests/assets/test.txt").unwrap();
    }

    #[test]
    #[should_panic]
    fn panic_for_wrong_path() {
        blob_from_file("./tests/kapusta").unwrap();
    }

    #[test]
    fn encode_file_into_blob_from_valid_xml() {
        let blob = blob_from_file("./tests/assets/valid_xml/ERN_example_1.xml").unwrap();

        assert_ne!(blob, [0; BYTES_PER_BLOB]);
    }

    #[test]
    fn encode_dir_files_into_blob() {
        let blob = blob_from_dir("./tests/assets/valid_xml").unwrap();

        assert_ne!(blob, [0; BYTES_PER_BLOB]);
    }
}
