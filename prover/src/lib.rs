pub const DDEX_GUEST_ID: [u32; 8] = [1804324578, 1710829334, 2782437713, 1488700276, 2609963593, 1932804332, 3018504481, 1724492902];
pub const DDEX_GUEST_ELF: &[u8] = include_bytes!("ddex_guest");
pub use prover_interface::*;