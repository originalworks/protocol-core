pub const DDEX_GUEST_ID: [u32; 8] = [1795724190, 2884950009, 2188846748, 381346155, 92827857, 416124791, 83262638, 351618579];
pub const DDEX_GUEST_ELF: &[u8] = include_bytes!("ddex_guest");
pub use prover_interface::*;