pub const DDEX_GUEST_ID: [u32; 8] = [1286094366, 3472262110, 773275823, 840627141, 1146959206, 2183744720, 939476648, 3653933889];
pub const DDEX_GUEST_ELF: &[u8] = include_bytes!("ddex_guest");
pub use prover_interface::*;