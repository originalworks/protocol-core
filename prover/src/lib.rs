pub const DDEX_GUEST_ID: [u32; 8] = [2701975946, 934715648, 2412713119, 3704675147, 1552379809, 3232328494, 139165195, 446402629];
pub const DDEX_GUEST_ELF: &[u8] = include_bytes!("ddex_guest");
pub use prover_interface::*;