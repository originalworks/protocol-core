pub const DDEX_GUEST_ID: [u32; 8] = [1443569663, 1452980516, 939870247, 3349208735, 3454160243, 4241909383, 2137134907, 3772865768];
pub const DDEX_GUEST_ELF: &[u8] = include_bytes!("ddex_guest");
pub use prover_interface::*;