pub const DDEX_GUEST_ID: [u32; 8] = [3654185808, 2496449233, 2206363168, 3279462808, 2658726011, 672983252, 3534444589, 3105428094];
pub const DDEX_GUEST_ELF: &[u8] = include_bytes!("ddex_guest");
pub use prover_interface::*;