#[rustfmt::skip]
pub const CURRENT_DDEX_GUEST_ID: [u32; 8] = [451203591, 3059929556, 1754286712, 6971930, 3759116214, 2978278281, 795481652, 9049088];
#[rustfmt::skip]
pub const PREVIOUS_DDEX_GUEST_ID: [u32; 8] = [1250100468, 287071712, 2529389667, 2550806557, 4167999951, 3040949899, 1315570359, 2517868328];
pub const CURRENT_DDEX_GUEST_ELF: &[u8] = include_bytes!("current_ddex_guest");
pub const PREVIOUS_DDEX_GUEST_ELF: &[u8] = include_bytes!("previous_ddex_guest");
pub use prover_interface::*;