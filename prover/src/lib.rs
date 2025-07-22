#[rustfmt::skip]
pub const CURRENT_DDEX_GUEST_ID: [u32; 8] = [1257162620, 2930857830, 1316735100, 210601467, 4194734166, 1858070196, 35071561, 54947997];
#[rustfmt::skip]
pub const PREVIOUS_DDEX_GUEST_ID: [u32; 8] = [3102207236, 3681267226, 3369154079, 1408402703, 1462175110, 2618567049, 3566194429, 2560944845];
pub const CURRENT_DDEX_GUEST_ELF: &[u8] = include_bytes!("current_ddex_guest");
pub const PREVIOUS_DDEX_GUEST_ELF: &[u8] = include_bytes!("previous_ddex_guest");
pub use prover_interface::*;