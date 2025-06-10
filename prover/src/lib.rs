#[rustfmt::skip]
pub const CURRENT_DDEX_GUEST_ID: [u32; 8] = [3102207236, 3681267226, 3369154079, 1408402703, 1462175110, 2618567049, 3566194429, 2560944845];
#[rustfmt::skip]
pub const PREVIOUS_DDEX_GUEST_ID: [u32; 8] = [451203591, 3059929556, 1754286712, 6971930, 3759116214, 2978278281, 795481652, 9049088];
pub const CURRENT_DDEX_GUEST_ELF: &[u8] = include_bytes!("current_ddex_guest");
pub const PREVIOUS_DDEX_GUEST_ELF: &[u8] = include_bytes!("previous_ddex_guest");
pub use prover_interface::*;