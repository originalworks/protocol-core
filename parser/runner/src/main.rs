use ddex_schema::ddex_parse_file;
use simple_logger::SimpleLogger;
fn main() {
    // SimpleLogger::new().init().unwrap();
    let res = ddex_parse_file("resources/example_messages/1Audio.xml").unwrap();
    // let res = ddex_parse_file("resources/example_messages/2Video.xml").unwrap();
    // let res = ddex_parse_file("resources/example_messages/3MixedMedia.xml").unwrap();
    // let res =
    //     ddex_parse_file("resources/example_messages/4SimpleAudioSingle.xml").unwrap();
    // let res =
    //     ddex_parse_file("resources/example_messages/5SimpleVideoSingle.xml").unwrap();
    // let res = ddex_parse_file("resources/example_messages/6Ringtone.xml").unwrap();
    // let res = ddex_parse_file("resources/example_messages/7LongformMusicalWorkVideo.xml")
    //     .unwrap();
    // IT's INCORRECT!!
    // let res = ddex_parse_file("resources/example_messages/8DjMix.xml").unwrap();
    // let res = ddex_parse_file("resources/example_messages/VariantClassical.xml").unwrap();
    dbg!(res);
}
