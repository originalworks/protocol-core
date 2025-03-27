#[rustfmt::skip]
pub const CURRENT_DDEX_GUEST_ID: [u32; 8] = [1250100468, 287071712, 2529389667, 2550806557, 4167999951, 3040949899, 1315570359, 2517868328];
#[rustfmt::skip]
pub const PREVIOUS_DDEX_GUEST_ID: [u32; 8] = [1804324578, 1710829334, 2782437713, 1488700276, 2609963593, 1932804332, 3018504481, 1724492902];
pub const CURRENT_DDEX_GUEST_ELF: &[u8] = include_bytes!("current_ddex_guest");
pub const PREVIOUS_DDEX_GUEST_ELF: &[u8] = include_bytes!("previous_ddex_guest");
pub use prover_interface::*;