mod bee;

use anyhow::Context;
use log_macros::log_error; // your existing macro
use regex::Regex;          // for replacing lines in .env
use sentry::{ClientInitGuard, User};
use serde_json::json;
use std::fs;
use std::path::Path;
use std::process::Command;
use validator_node::Config;
use ctrlc;

// Additional crates for key generation
use rand::rngs::OsRng;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use tiny_keccak::{Hasher, Keccak};

fn init_sentry(config: &Config) -> ClientInitGuard {
    sentry::init((
        "https://8a5e3e61ac0beb391ad84b32086674df@o4508766269014016.ingest.us.sentry.io/4508805625217024",
        sentry::ClientOptions {
            environment: Some(config.environment.to_owned().into()),
            release: sentry::release_name!(),
            attach_stacktrace: true,
            auto_session_tracking: true,
            traces_sample_rate: 1.0, // lower it in production
            ..Default::default()
        },
    ))
}

fn init_logging() -> anyhow::Result<()> {
    let mut log_builder = pretty_env_logger::formatted_timed_builder();
    log_builder.parse_filters("info");

    let logger = sentry_log::SentryLogger::with_dest(log_builder.build());
    log::set_boxed_logger(Box::new(logger))
        .with_context(|| "Failed to set boxed logger")?;
    log::set_max_level(log::LevelFilter::Info);

    Ok(())
}

/// Queries nvidia-smi for both `name` and `memory.total`.
/// Logs each GPU found and returns the largest memory (in MiB) among them.
/// Returns `None` if `nvidia-smi` is unavailable or no GPUs are found.
fn detect_nvidia_gpu_memory_mib() -> Option<u64> {
    let output = Command::new("nvidia-smi")
        .args(["--query-gpu=name,memory.total", "--format=csv,noheader,nounits"])
        .output()
        .ok()?; // If the command fails, return None

    if !output.status.success() {
        // nvidia-smi ran but returned non-zero => no GPU or some error
        return None;
    }

    let stdout_str = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout_str.lines().collect();
    if lines.is_empty() {
        // No GPUs found
        return None;
    }

    let mut max_mem = 0;
    for line in lines {
        // Expect "Name,Memory"
        let parts: Vec<_> = line.split(',').map(|s| s.trim()).collect();
        if parts.len() == 2 {
            let gpu_name = parts[0];
            if let Ok(mem) = parts[1].parse::<u64>() {
                log::info!("Found NVIDIA GPU: Model=\"{}\", Memory={} MiB", gpu_name, mem);
                if mem > max_mem {
                    max_mem = mem;
                }
            }
        }
    }

    if max_mem > 0 {
        Some(max_mem)
    } else {
        None
    }
}

/// Decide which SEGMENT_LIMIT_PO2 to use based on GPU memory.
fn decide_segment_limit_po2(mem_mib: u64) -> u64 {
    // If <4 GB => 0
    // If >=4 GB => 18
    // If >=8 GB => 19
    // If >=16 GB => 20
    if mem_mib >= 16_384 {
        20
    } else if mem_mib >= 8_192 {
        19
    } else if mem_mib >= 4_096 {
        18
    } else {
        0
    }
}

/// Simple keccak256 function
fn keccak256(bytes: &[u8]) -> [u8; 32] {
    let mut keccak = Keccak::v256();
    keccak.update(bytes);
    let mut hash = [0u8; 32];
    keccak.finalize(&mut hash);
    hash
}

/// Given a secp256k1 SecretKey, derive the Ethereum address (0x + 40 hex).
fn eth_address_from_private_key(sk: &SecretKey) -> String {
    let secp = Secp256k1::new();
    let pk = PublicKey::from_secret_key(&secp, sk);
    // Uncompressed => 65 bytes: 0x04 + 64-byte pubkey
    let uncompressed = pk.serialize_uncompressed();
    // Skip the first byte (0x04). Hash the remaining 64 with keccak256
    let hash = keccak256(&uncompressed[1..]);
    // Eth address = last 20 bytes, hex-encoded with "0x" prefix
    let address = &hash[12..];
    format!("0x{}", hex::encode(address))
}

/// Generates a new secp256k1 private key, returning (private_key_hex, eth_address).
fn generate_new_eth_key() -> (String, String) {
    let secp = Secp256k1::new();
    let (secret_key, _) = secp.generate_keypair(&mut OsRng);
    let private_key_hex = hex::encode(secret_key.secret_bytes());
    let address = eth_address_from_private_key(&secret_key);
    (private_key_hex, address)
}

/// If we just created .env, fill PRIVATE_KEY with a newly generated one,
/// then set SEGMENT_LIMIT_PO2 based on GPU memory (replacing or inserting).
fn update_env_if_created(mut env_contents: String) -> anyhow::Result<String> {
    // 1) Generate a new Ethereum key pair
    let (private_key_hex, address) = generate_new_eth_key();

    // 2) Replace or insert PRIVATE_KEY=...
    {
        let private_key_line = format!("PRIVATE_KEY={}", private_key_hex);
        let regex = Regex::new(r"(?m)^PRIVATE_KEY=.*$")?;
        if regex.is_match(&env_contents) {
            // Replace existing line
            env_contents = regex
                .replace_all(&env_contents, private_key_line.clone())
                .to_string();
        } else {
            // If there's no existing line, append
            env_contents.push_str(&format!("\n{}\n", private_key_line));
        }
    }

    // 3) Log the newly generated address (don't log the private key)
    log::info!("Generated new Ethereum key. Address={}", address);
    sentry::configure_scope(|scope| {
        scope.set_extra("generated_eth_address", address.clone().into());
    });

    // 4) Detect GPU memory & set SEGMENT_LIMIT_PO2
    let limit = match detect_nvidia_gpu_memory_mib() {
        Some(mib) => {
            log::info!("Detected 1 or more NVIDIA GPU(s). Maximum memory of them is: {} MiB", mib);
            let limit = decide_segment_limit_po2(mib);
            log::info!(
                "Set SEGMENT_LIMIT_PO2={} for newly created .env (GPU has {} MiB)",
                limit,
                mib
            );
            limit
        }
        None => {
            log::info!("No NVIDIA GPU detected or `nvidia-smi` not available.");
            let limit = 0;
            log::info!("Set SEGMENT_LIMIT_PO2=0 for newly created .env");
            limit
        }
    };

    // 5) Replace or insert SEGMENT_LIMIT_PO2=...
    {
        let seg_line = format!("SEGMENT_LIMIT_PO2={}", limit);
        let regex = Regex::new(r"(?m)^SEGMENT_LIMIT_PO2=.*$")?;
        if regex.is_match(&env_contents) {
            // Replace existing line
            env_contents = regex.replace_all(&env_contents, seg_line).to_string();
        } else {
            // Append if missing
            env_contents.push_str(&format!("\n{}\n", seg_line));
        }
    }

    Ok(env_contents)
}

async fn init(config: Config) -> anyhow::Result<()> {
    sentry::configure_scope(|scope| {
        scope.set_user(Some(User {
            username: Some(config.username.to_owned()),
            ..Default::default()
        }));

        let mut cloned_config = config.clone();
        // Do not log the real private key
        cloned_config.private_key = "***".to_string();
        scope.set_extra("config", json!(cloned_config));
    });

    validator_node::run(&config).await.map_err(|e| {
        sentry::configure_scope(|scope| {
            scope.set_extra("error_object", json!(format!("{e:#?}")));
        });
        log_error!("{e}")
    })?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    // A) Init logging
    init_logging()?;

    // -- (1) Ensure Bee is installed if not present:
    if !bee::is_bee_installed() {                        // <-- Added
        match bee::install_bee() {                       // <-- Added
            Ok(_) => log::info!("Bee installed successfully."),    
            Err(e) => {
                log::error!("Failed to install Bee: {}", e);
                std::process::exit(1);
            }
        }
    }

    // -- (2) Start Bee node
    let mut bee_process = match bee::start_bee_node() {
        Ok(proc) => {
            log::info!("Bee node started successfully.");
            proc
        }
        Err(e) => {
            log::error!("Failed to start Bee node: {}", e);
            std::process::exit(1);
        }
    };

    // B) Check if .env exists; if not, create it from .env.template
    let env_path = Path::new(".env");
    if !env_path.exists() {
        fs::copy(".env.template", ".env")
            .with_context(|| "Failed to copy .env.template to .env")?;
        log::info!("No .env found. Created .env from .env.template.");

        let env_contents = fs::read_to_string(".env")?;
        let updated_env = update_env_if_created(env_contents)?;
        fs::write(".env", updated_env)?;
    } else {
        // .env already exists, so just detect/log GPU presence or not
        match detect_nvidia_gpu_memory_mib() {
            Some(mib) => {
                log::info!("Detected NVIDIA GPU(s). Maximum memory: {} MiB", mib);
            }
            None => {
                log::info!("No NVIDIA GPU detected or `nvidia-smi` not available.");
            }
        }
    }

    // C) Load .env
    dotenvy::dotenv().ok();

    // C1) Validate important env vars
    // If the user left them as "placeholder" or empty, shut down gracefully with a clear error.
    for (key, desc) in &[
        ("RPC_URL", "HTTP(S) RPC endpoint"),
        ("WS_URL", "WebSocket RPC endpoint"),
        ("BEACON_RPC_URL", "Beacon chain RPC endpoint"),
    ] {
        let val = std::env::var(key).unwrap_or_default();
        if val.is_empty() || val == "placeholder" {
            log::error!(
                "{} is unset or placeholder. Please set a valid {} in .env and restart.",
                key,
                desc
            );
            std::process::exit(1);
        }
    }

    // C2) Derive & log the public address from PRIVATE_KEY, if present
    if let Ok(priv_key_hex) = std::env::var("PRIVATE_KEY") {
        match hex::decode(&priv_key_hex) {
            Ok(sk_bytes) if sk_bytes.len() == 32 => {
                if let Ok(secret_key) = SecretKey::from_slice(&sk_bytes) {
                    let address = eth_address_from_private_key(&secret_key);
                    log::info!("Public key used: {}", address);
                    sentry::configure_scope(|scope| {
                        scope.set_extra("public_key_from_env", address.into());
                    });
                } else {
                    log::warn!("PRIVATE_KEY in .env is not a valid secp256k1 key");
                }
            }
            _ => {
                log::warn!("PRIVATE_KEY in .env is missing or not valid hex for a 32-byte key");
            }
        }
    }

    // D) Log out START_BLOCK, SEGMENT_LIMIT_PO2, ENVIRONMENT
    let start_block = std::env::var("START_BLOCK").unwrap_or_default();
    let segment_limit = std::env::var("SEGMENT_LIMIT_PO2").unwrap_or_default();
    let environment = std::env::var("ENVIRONMENT").unwrap_or_default();

    log::info!("START_BLOCK={}", start_block);
    log::info!("SEGMENT_LIMIT_PO2={}", segment_limit);
    log::info!("ENVIRONMENT={}", environment);

    sentry::configure_scope(|scope| {
        scope.set_extra("START_BLOCK", start_block.clone().into());
        scope.set_extra("SEGMENT_LIMIT_PO2", segment_limit.clone().into());
        scope.set_extra("ENVIRONMENT", environment.clone().into());
    });

    // E) Build & run your validator_node logic
    let config = Config::build();
    let _guard = init_sentry(&config);

    let result = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(init(config));

    // Stop the Bee node before exiting the application
    bee::stop_bee_node(&mut bee_process);

    result
}
