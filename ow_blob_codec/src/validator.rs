use crate::errors::OwCodecError;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::error::Error;
use std::path::Path;
use std::{fs::File, io::Read};

pub fn validate_xml(path: &Path) -> Result<&Path, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut file_buffer = Vec::new();
    file.read_to_end(&mut file_buffer)?;

    if file_buffer.len() == 0 {
        return Err(Box::new(OwCodecError::BlobOverflowError(
            path.to_string_lossy().to_string(),
        )));
    }

    let mut reader = Reader::from_file(&path)?;
    let mut buffer = Vec::new();

    loop {
        match reader.read_event_into(&mut buffer) {
            Ok(Event::Eof) => break,
            Ok(_) => (),
            Err(_) => {
                return Err(Box::new(OwCodecError::CorruptedXmlFile(
                    path.to_string_lossy().to_string(),
                )))
            }
        }
        buffer.clear();
    }

    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn panic_for_empty_file() {
        let path = Path::new("./tests/assets/corrupted_xml/empty.xml");
        validate_xml(path).unwrap();
    }

    #[test]
    #[should_panic]
    fn panic_for_corrupted_xml() {
        let path = Path::new("./tests/assets/corrupted_xml/corrupted.xml");
        validate_xml(path).unwrap();
    }

    #[test]
    fn pass_for_valid_xml() {
        validate_xml(Path::new("./tests/assets/valid_xml/ERN_example_1.xml")).unwrap();
        validate_xml(Path::new("./tests/assets/valid_xml/ERN_example_2.xml")).unwrap();
        validate_xml(Path::new("./tests/assets/valid_xml/ERN_example_3.xml")).unwrap();
    }
}
