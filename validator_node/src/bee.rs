use std::process::{Child, Command};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use std::fs;

use log::{info, error};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

const BEE_BINARY: &str = "bee";
const BEE_BINARY_NAME: &str = "bee";
const BEE_RELEASE_URL: &str = "https://github.com/ethersphere/bee/releases/download/v2.4.0/bee-linux-amd64";

const BEE_CONFIG_FILE: &str = "./bee-config.yaml";
const BEE_CONFIG_TEMPLATE: &str = "./bee-config.yaml.template";

/// Resolves the Bee install path dynamically
pub fn get_bee_install_path() -> String {
    dirs::home_dir()
        .map(|home| home.join(".local/bin").join(BEE_BINARY_NAME).to_string_lossy().to_string())
        .unwrap_or_else(|| "/usr/local/bin/bee".to_string())
}

/// Checks if Bee is installed by checking if the binary exists.
pub fn is_bee_installed() -> bool {
    let install_path = get_bee_install_path();
    Path::new(&install_path).exists()
}

/// Downloads and installs Bee if it's missing.
pub fn install_bee() -> Result<(), String> {
    let install_path = get_bee_install_path();

    if Path::new(&install_path).exists() {
        info!("Bee is already installed at: {}", install_path);
        return Ok(());
    }

    info!("Bee not found. Installing Bee...");

    // Ensure .local/bin directory exists
    let bee_dir = Path::new(&install_path).parent().unwrap();
    if !bee_dir.exists() {
        fs::create_dir_all(bee_dir).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // Download the Bee binary
    info!("Downloading Bee from: {}", BEE_RELEASE_URL);
    let status = Command::new("curl")
        .args(&["-L", "--fail", "-o", &install_path, BEE_RELEASE_URL])
        .status()
        .map_err(|e| format!("Failed to download Bee: {}", e))?;

    if !status.success() {
        return Err(format!(
            "Failed to download Bee binary with exit code: {:?}. Check network or URL.",
            status.code()
        ));
    }

    // Verify the binary file after download
    if !Path::new(&install_path).exists() {
        return Err("Bee binary was not downloaded despite successful curl execution.".into());
    }

    // Make the binary executable
    let chmod_status = Command::new("chmod")
        .args(&["+x", &install_path])
        .status()
        .map_err(|e| format!("Failed to set executable permission: {}", e))?;

    if !chmod_status.success() {
        return Err("Failed to set execute permission on Bee binary.".into());
    }

    info!("Bee successfully installed to {}", install_path);
    Ok(())
}

/// Generates a secure random key for the token-encryption-key field using rand's OsRng.
pub fn generate_token_encryption_key() -> String {
    use rand::RngCore;
    use rand::rngs::OsRng;

    let mut key = [0u8; 32];
    // Fill `key` with OS-based randomness
    OsRng.fill_bytes(&mut key);

    STANDARD.encode(key)
}

/// Copies the template to create the Bee configuration file if it doesn't exist and modifies it if needed.
pub fn ensure_config_file() -> Result<(), String> {
    let config_path = Path::new(BEE_CONFIG_FILE);
    let template_path = Path::new(BEE_CONFIG_TEMPLATE);

    if config_path.exists() {
        info!("Bee configuration file already exists: {}", BEE_CONFIG_FILE);
        return Ok(());
    }

    if !template_path.exists() {
        return Err(format!(
            "Configuration template file '{}' not found.",
            BEE_CONFIG_TEMPLATE
        ));
    }

    fs::copy(BEE_CONFIG_TEMPLATE, BEE_CONFIG_FILE)
        .map_err(|e| format!("Failed to copy config template: {}", e))?;

    info!("Copied '{}' to '{}'", BEE_CONFIG_TEMPLATE, BEE_CONFIG_FILE);

    // Modify the config file to add a token-encryption-key
    let config_content = fs::read_to_string(BEE_CONFIG_FILE)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    let token_key = generate_token_encryption_key();
    //token-encryption-key field no longer exists in the config file
    /*
    let updated_content = config_content.replace(
        "token-encryption-key: \"\"",
        &format!("token-encryption-key: \"{}\"", token_key)
    );
    */

    fs::write(BEE_CONFIG_FILE, updated_content)
        .map_err(|e| format!("Failed to update config file with token encryption key: {}", e))?;

    info!("Generated and inserted a token-encryption-key into the config file.");

    Ok(())
}

/// Starts the Bee node as a background process with the configuration file.
pub fn start_bee_node() -> Result<Child, String> {
    ensure_config_file()?;

    info!("Starting Bee node with config: {}", BEE_CONFIG_FILE);

    let bee_binary = get_bee_install_path();

    if !Path::new(&bee_binary).exists() {
        return Err(format!("Bee binary not found at: {}", bee_binary));
    }

    let bee_process = Command::new(&bee_binary)
        .args(&["start", "--config", BEE_CONFIG_FILE, "--password 1234"]) // --password 1234 is a temporary workaround
        .spawn()
        .map_err(|e| format!("Failed to start Bee node: {}", e))?;

    // Give Bee a few seconds to launch
    sleep(Duration::from_secs(5));
    info!("Bee node started successfully.");

    Ok(bee_process)
}

/// Stops the Bee node process.
pub fn stop_bee_node(bee_process: &mut Child) {
    info!("Stopping Bee node...");
    if let Err(e) = bee_process.kill() {
        error!("Failed to kill Bee node: {}", e);
    } else {
        info!("Bee node stopped.");
    }
}
