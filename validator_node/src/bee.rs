use std::process::{Command, Child};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use std::fs;
use log::{info, error};

const BEE_BINARY: &str = "bee";
const BEE_INSTALL_PATH: &str = "/usr/local/bin/bee";
const BEE_RELEASE_URL: &str = "https://github.com/ethersphere/bee/releases/latest/download/bee-linux-amd64";
const BEE_CONFIG_FILE: &str = "./bee-config.yaml";
const BEE_CONFIG_TEMPLATE: &str = "./bee-config.yaml.template";

/// Checks if Bee is installed by checking if the binary is in the PATH.
pub fn is_bee_installed() -> bool {
    Command::new("which")
        .arg(BEE_BINARY)
        .output()
        .map_or(false, |output| output.status.success())
}

/// Downloads and installs Bee if it's missing.
pub fn install_bee() -> Result<(), String> {
    if Path::new(BEE_INSTALL_PATH).exists() {
        info!("Bee is already installed.");
        return Ok(());
    }

    info!("Bee not found. Installing Bee...");

    let status = Command::new("curl")
        .args(&["-L", "-o", BEE_INSTALL_PATH, BEE_RELEASE_URL])
        .status()
        .map_err(|e| format!("Failed to download Bee: {}", e))?;

    if !status.success() {
        return Err("Failed to download Bee binary.".into());
    }

    let chmod_status = Command::new("chmod")
        .args(&["+x", BEE_INSTALL_PATH])
        .status()
        .map_err(|e| format!("Failed to set executable permission: {}", e))?;

    if !chmod_status.success() {
        return Err("Failed to set execute permission on Bee binary.".into());
    }

    info!("Bee successfully installed to {}", BEE_INSTALL_PATH);
    Ok(())
}

/// Copies the template to create the Bee configuration file if it doesn't exist.
pub fn ensure_config_file() -> Result<(), String> {
    let config_path = Path::new(BEE_CONFIG_FILE);
    let template_path = Path::new(BEE_CONFIG_TEMPLATE);

    if config_path.exists() {
        info!("Bee configuration file already exists: {}", BEE_CONFIG_FILE);
        return Ok(());
    }

    // Check if template exists
    if !template_path.exists() {
        return Err(format!(
            "Configuration template file '{}' not found.",
            BEE_CONFIG_TEMPLATE
        ));
    }

    // Copy the template to create the config file
    fs::copy(BEE_CONFIG_TEMPLATE, BEE_CONFIG_FILE)
        .map_err(|e| format!("Failed to copy config template: {}", e))?;

    info!("Copied '{}' to '{}'", BEE_CONFIG_TEMPLATE, BEE_CONFIG_FILE);
    Ok(())
}

/// Starts the Bee node as a background process with the configuration file.
pub fn start_bee_node() -> Result<Child, String> {
    // Ensure config file exists
    ensure_config_file()?;

    info!("Starting Bee node with config: {}", BEE_CONFIG_FILE);

    let bee_process = Command::new(BEE_BINARY)
        .args(&["start", "--config", BEE_CONFIG_FILE])
        .spawn()
        .map_err(|e| format!("Failed to start Bee node: {}", e))?;

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
