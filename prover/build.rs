use dotenvy::dotenv;
use risc0_build::{self, embed_methods_with_options, DockerOptionsBuilder, GuestOptionsBuilder};
use risc0_build_ethereum::generate_solidity_files;
use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

const SOLIDITY_IMAGE_ID_PATH: &str = "../contracts/contracts/ImageID.sol";
const SOLIDITY_ELF_PATH: &str = "../contracts/contracts/Elf.sol";
const LIB_PATH: &str = "src/lib.rs";
const CURRENT_BIN_NAME: &str = "current_ddex_guest";
const PREVIOUS_BIN_NAME: &str = "previous_ddex_guest";

fn get_current_image_id() -> String {
    let content = fs::read_to_string(LIB_PATH).unwrap();

    let pattern = format!("pub const {}_ID", CURRENT_BIN_NAME.to_ascii_uppercase());
    let line = content
        .lines()
        .find(|line| line.starts_with(pattern.as_str()))
        .expect("CURRENT_DDEX_GUEST_ID not found");

    let value_part = line.split_once('=').expect("Invalid constant format").1;
    let start = value_part.find("[").expect("Missing [ in array");
    let end = value_part.find("]").expect("Missing ] in array");

    let current_image_id = value_part[start..end + 1].to_string();

    return current_image_id;
}

fn main() {
    dotenv().unwrap();

    let run_build = matches!(
        std::env::var("RUN_BUILD").unwrap().as_str(),
        "true" | "t" | "1"
    );

    if !run_build {
        return ();
    }

    // let use_docker = env::var("RISC0_USE_DOCKER").ok().map(|_| DockerOptions {
    //     root_dir: Some("../".into()),
    // });

    let mut builder = GuestOptionsBuilder::default();

    if env::var("RISC0_USE_DOCKER").is_ok() {
        let root_dir = Path::new("../").to_path_buf();
        let docker_options = DockerOptionsBuilder::default()
            .root_dir(root_dir)
            .build()
            .unwrap();
        builder.use_docker(docker_options);
    }

    let guest_options = builder.build().unwrap();
    let list = embed_methods_with_options(HashMap::from([("ddex_guest", guest_options)]));
    let entry = list.first().unwrap();

    // Archive previous version
    let current_image_id_backup = get_current_image_id();

    // Move around elf files
    std::fs::copy(
        format!("src/{}", CURRENT_BIN_NAME),
        format!("src/{}", PREVIOUS_BIN_NAME),
    )
    .expect("Failed to copy bin file");

    std::fs::copy(&entry.path.to_string(), format!("src/{}", CURRENT_BIN_NAME))
        .expect("Failed to copy bin file");

    // Create lib.rs
    let mut lib_file = File::create(LIB_PATH).unwrap();
    let mut lib_content = Vec::new();

    lib_content.push(format!(
        "#[rustfmt::skip]\npub const {}_ID: [u32; 8] = {:?};",
        CURRENT_BIN_NAME.to_uppercase(),
        &entry.image_id.as_words()
    ));

    lib_content.push(format!(
        "#[rustfmt::skip]\npub const {}_ID: [u32; 8] = {};",
        PREVIOUS_BIN_NAME.to_uppercase(),
        if format!("{:?}", &entry.image_id).to_string() == current_image_id_backup {
            "[0; 8]".to_string()
        } else {
            current_image_id_backup
        }
    ));

    lib_content.push(format!(
        r#"pub const {}_ELF: &[u8] = include_bytes!("{}");"#,
        CURRENT_BIN_NAME.to_uppercase(),
        CURRENT_BIN_NAME
    ));

    lib_content.push(format!(
        r#"pub const {}_ELF: &[u8] = include_bytes!("{}");"#,
        PREVIOUS_BIN_NAME.to_uppercase(),
        PREVIOUS_BIN_NAME
    ));

    lib_content.push("pub use prover_interface::*;".to_string());

    lib_file.write(lib_content.join("\n").as_bytes()).unwrap();

    let solidity_opts = risc0_build_ethereum::Options::default()
        .with_image_id_sol_path(SOLIDITY_IMAGE_ID_PATH)
        .with_elf_sol_path(SOLIDITY_ELF_PATH);

    generate_solidity_files(list.as_slice(), &solidity_opts).unwrap();
}
