use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    Reader,
};

use crate::{
    constants::IMAGE_FILE_CID_TAG, ddex_sequencer::DdexSequencer::DdexMessageData,
    errors::OwValidatorNodeError,
};
use std::{error::Error, io::Cursor};

pub struct ExtractedMessageData {
    pub emittable_data: DdexMessageData,
    pub image_ipfs_cid: String,
}

pub fn extract_message_data(
    decoded_blob: &Vec<Vec<u8>>,
) -> Result<Vec<ExtractedMessageData>, Box<dyn Error>> {
    let mut result = Vec::<ExtractedMessageData>::new();

    let mut reader: Reader<Cursor<&Vec<u8>>>;
    let mut buffer = Vec::new();

    let mut inside_isrc_tag = false;
    let mut inside_release_id_tag = false;
    let mut inside_image_file_cid_tag = false;

    let mut isrc_tag_value = String::new();
    let mut release_id_tag_value = String::new();
    let mut image_file_cid_value = String::new();

    for ddex_message in decoded_blob {
        reader = Reader::from_reader(Cursor::new(&ddex_message));

        loop {
            match reader.read_event_into(&mut buffer) {
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) if e == BytesStart::new("ISRC") => {
                    inside_isrc_tag = true;
                }
                Ok(Event::Start(e)) if e == BytesStart::new("GRid") => {
                    inside_release_id_tag = true;
                }
                Ok(Event::Start(e)) if e == BytesStart::new(IMAGE_FILE_CID_TAG) => {
                    inside_image_file_cid_tag = true;
                }
                Ok(Event::Text(e)) if inside_isrc_tag => {
                    isrc_tag_value = String::from_utf8(e.to_vec())?;
                }
                Ok(Event::Text(e)) if inside_release_id_tag => {
                    release_id_tag_value = String::from_utf8(e.to_vec())?;
                }
                Ok(Event::Text(e)) if inside_image_file_cid_tag => {
                    image_file_cid_value = String::from_utf8(e.to_vec())?;
                }
                Ok(Event::End(e)) if e == BytesEnd::new("ISRC") => {
                    inside_isrc_tag = false;
                }
                Ok(Event::End(e)) if e == BytesEnd::new("GRid") => {
                    inside_release_id_tag = false;
                }
                Ok(Event::End(e)) if e == BytesEnd::new(IMAGE_FILE_CID_TAG) => {
                    inside_image_file_cid_tag = false;
                }
                Ok(_) => (),
                Err(_) => {
                    return Err(Box::new(OwValidatorNodeError::InvalidHexStringValue(
                        "duer".to_string(),
                    )))
                }
            }
            buffer.clear();
        }

        result.push(ExtractedMessageData {
            emittable_data: DdexMessageData {
                isrc: isrc_tag_value.clone(),
                releaseId: release_id_tag_value.clone(),
            },
            image_ipfs_cid: image_file_cid_value.clone(),
        });

        isrc_tag_value = String::new();
        release_id_tag_value = String::new();
    }

    Ok(result)
}
