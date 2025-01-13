pub const DDEX_GUEST_ID: [u32; 8] = [3369104439, 469700432, 1381535978, 1229322847, 2287385638, 3286027250, 2137566209, 2963702429];
pub const DDEX_GUEST_ELF: &[u8] = include_bytes!("ddex_guest");
pub use prover_interface::*;