#[rustfmt::skip]
pub const CURRENT_DDEX_GUEST_ID: [u32; 8] = [2350930292, 4060131709, 2722476513, 648990656, 3781418365, 1381390647, 1464157086, 3812315690];
#[rustfmt::skip]
pub const PREVIOUS_DDEX_GUEST_ID: [u32; 8] = [2736210167, 126406685, 3952657666, 1937735572, 3274166662, 3273172413, 154522746, 3282716169];
pub const CURRENT_DDEX_GUEST_ELF: &[u8] = include_bytes!("current_ddex_guest");
pub const PREVIOUS_DDEX_GUEST_ELF: &[u8] = include_bytes!("previous_ddex_guest");
pub use prover_interface::*;