use anyhow::Context;
use log_macros::log_error;         // your existing macro
use regex::Regex;                  // <--- new
use sentry::{ClientInitGuard, User};
use serde_json::json;
use std::fs;
use std::path::Path;
use std::process::Command;
use validator_node::Config;

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
    // Now we query both the GPU `name` and `memory.total`
    let output = Command::new("nvidia-smi")
        .args(["--query-gpu=name,memory.total", "--format=csv,noheader,nounits"])
        .output()
        .ok()?; // If the command fails, return None

    if !output.status.success() {
        // nvidia-smi ran but returned non-zero exit code => no GPU or some error
        return None;
    }

    let stdout_str = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout_str.lines().collect();
    if lines.is_empty() {
        // No GPUs found
        return None;
    }

    let mut max_mem = 0;
    // Parse each line (one line per GPU)
    for line in lines {
        // Expect "Name,Memory"
        let parts: Vec<_> = line.split(',').map(|s| s.trim()).collect();
        if parts.len() == 2 {
            let gpu_name = parts[0];
            if let Ok(mem) = parts[1].parse::<u64>() {
                // Log the GPU name and memory
                log::info!("Found NVIDIA GPU: Model=\"{}\", Memory={} MiB", gpu_name, mem);
                // Track the largest memory
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
/// then set SEGMENT_LIMIT_PO2 based on GPU memory if not already set.
fn update_env_if_created(mut env_contents: String) -> anyhow::Result<String> {
    // 1) Generate a new Ethereum key pair
    let (private_key_hex, address) = generate_new_eth_key();

    // 2) Replace or insert PRIVATE_KEY=...
    let private_key_line = format!("PRIVATE_KEY={}", private_key_hex);
    if env_contents.contains("PRIVATE_KEY=") {
        // Replace existing line using a small regex
        let regex = Regex::new(r"(?m)^PRIVATE_KEY=.*$")?;
        env_contents = regex
            .replace_all(&env_contents, private_key_line.clone())
            .to_string();
    } else {
        // If there's no existing line at all, append
        env_contents.push_str(&format!("\n{}\n", private_key_line));
    }

    // Only log the public address
    log::info!("Generated new Ethereum key. Address={}", address);
    sentry::configure_scope(|scope| {
        scope.set_extra("generated_eth_address", address.clone().into());
    });

    // 3) Detect GPU memory
    match detect_nvidia_gpu_memory_mib() {
        Some(mib) => {
            log::info!("Detected 1 or more NVIDIA GPU(s). Maximum memory of them is: {} MiB", mib);
            let limit = decide_segment_limit_po2(mib);
            if !env_contents.contains("SEGMENT_LIMIT_PO2=") {
                env_contents.push_str(&format!("\nSEGMENT_LIMIT_PO2={}\n", limit));
            }
            log::info!(
                "Set SEGMENT_LIMIT_PO2={} for newly created .env (GPU has {} MiB)",
                limit,
                mib
            );
        }
        None => {
            log::info!("No NVIDIA GPU detected or `nvidia-smi` not available.");
            if !env_contents.contains("SEGMENT_LIMIT_PO2=") {
                env_contents.push_str("\nSEGMENT_LIMIT_PO2=0\n");
            }
            log::info!("Set SEGMENT_LIMIT_PO2=0 for newly created .env");
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
        // Do not log the real private key:
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

    // C1) Derive & log the public address from PRIVATE_KEY, if present
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

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(init(config))
}
