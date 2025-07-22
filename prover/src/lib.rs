#[rustfmt::skip]
pub const CURRENT_DDEX_GUEST_ID: [u32; 8] = [2736210167, 126406685, 3952657666, 1937735572, 3274166662, 3273172413, 154522746, 3282716169];
#[rustfmt::skip]
pub const PREVIOUS_DDEX_GUEST_ID: [u32; 8] = [3102207236, 3681267226, 3369154079, 1408402703, 1462175110, 2618567049, 3566194429, 2560944845];
pub const CURRENT_DDEX_GUEST_ELF: &[u8] = include_bytes!("current_ddex_guest");
pub const PREVIOUS_DDEX_GUEST_ELF: &[u8] = include_bytes!("previous_ddex_guest");
pub use prover_interface::*;