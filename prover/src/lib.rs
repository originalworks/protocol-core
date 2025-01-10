pub const DDEX_GUEST_ID: [u32; 8] = [795128476, 1183041679, 3472933527, 325156746, 3556561243, 372050051, 421501584, 2973040595];
pub const DDEX_GUEST_ELF: &[u8] = include_bytes!("ddex_guest");
pub use prover_interface::*;