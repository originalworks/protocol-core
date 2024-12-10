pub const DDEX_GUEST_ID: [u32; 8] = [4204661656, 662750207, 398597736, 2719835357, 3291798939, 2345184322, 2532602085, 1786775211];
pub const DDEX_GUEST_ELF: &[u8] = include_bytes!("ddex_guest");
pub use prover_interface::*;