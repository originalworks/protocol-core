#[rustfmt::skip]
pub const CURRENT_DDEX_GUEST_ID: [u32; 8] = [1579710201, 41761759, 220143184, 4250135564, 3769561102, 1473455056, 187928062, 2493613017];
#[rustfmt::skip]
pub const PREVIOUS_DDEX_GUEST_ID: [u32; 8] = [1579710201, 41761759, 220143184, 4250135564, 3769561102, 1473455056, 187928062, 2493613017];
pub const CURRENT_DDEX_GUEST_ELF: &[u8] = include_bytes!("current_ddex_guest");
pub const PREVIOUS_DDEX_GUEST_ELF: &[u8] = include_bytes!("previous_ddex_guest");
pub use prover_interface::*;