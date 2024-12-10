use std::{
    fs::{self, File},
    io::Write,
};

use dotenvy::dotenv;
use risc0_build;

// // Uncomment when playing with docker build
// use risc0_build::{DockerOptions, GuestOptions};
// use std::collections::HashMap;

fn main() {
    dotenv().unwrap();

    let run_build = matches!(
        std::env::var("RUN_BUILD").unwrap().as_str(),
        "true" | "t" | "1"
    );

    if !run_build {
        return ();
    }

    let list = risc0_build::embed_methods();

    // // Or use docker to make deterministic build
    // // TODO: Docker build throws error - local dependencies outside the prover package are not visible in the docker
    // let list = risc0_build::embed_methods_with_options(HashMap::from([(
    //     "ddex_parser_guest",
    //     GuestOptions {
    //         features: Vec::new(),
    //         use_docker: Some(DockerOptions {
    //             root_dir: Some("../../".into()),
    //         }),
    //     },
    // )]));
    let entry = list.first().unwrap();

    let mut lib_file = File::create("src/lib.rs").unwrap();
    let mut lib_content = Vec::new();

    let image_id = format!(
        "pub const {}_ID: [u32; 8] = {:?};",
        &entry.name.to_uppercase(),
        &entry.image_id
    );

    lib_content.push(image_id);

    fs::copy(
        &entry.path.to_string(),
        format!("src/{}", &entry.name.to_string()),
    )
    .unwrap();

    let elf = format!(
        r#"pub const {}_ELF: &[u8] = include_bytes!("{}");"#,
        &entry.name.to_uppercase(),
        &entry.name.to_string()
    );

    lib_content.push(elf);

    let interface = "pub use prover_interface::*;".to_string();

    lib_content.push(interface);

    lib_file.write(lib_content.join("\n").as_bytes()).unwrap();
}
