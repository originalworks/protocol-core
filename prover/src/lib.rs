#[rustfmt::skip]
pub const CURRENT_DDEX_GUEST_ID: [u32; 8] = [1804324578, 1710829334, 2782437713, 1488700276, 2609963593, 1932804332, 3018504481, 1724492902];
#[rustfmt::skip]
pub const PREVIOUS_DDEX_GUEST_ID: [u32; 8] = [0; 8];
pub const CURRENT_DDEX_GUEST_ELF: &[u8] = include_bytes!("current_ddex_guest");
pub const PREVIOUS_DDEX_GUEST_ELF: &[u8] = include_bytes!("previous_ddex_guest");
pub use prover_interface::*;
