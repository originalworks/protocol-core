pub const DDEX_GUEST_ID: [u32; 8] = [3360582775, 2562138718, 2921358103, 4061262186, 2417372690, 2257034942, 1057970198, 2149426504];
pub const DDEX_GUEST_ELF: &[u8] = include_bytes!("ddex_guest");
pub use prover_interface::*;