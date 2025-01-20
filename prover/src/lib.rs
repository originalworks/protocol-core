pub const DDEX_GUEST_ID: [u32; 8] = [2372024186, 937387774, 3977086473, 3863762085, 1395434531, 4217839000, 2920127036, 1966306933];
pub const DDEX_GUEST_ELF: &[u8] = include_bytes!("ddex_guest");
pub use prover_interface::*;