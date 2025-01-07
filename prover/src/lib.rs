pub const DDEX_GUEST_ID: [u32; 8] = [3629867018, 3775469235, 4246080319, 1281573842, 3465072370, 1722924655, 288412137, 3577806805];
pub const DDEX_GUEST_ELF: &[u8] = include_bytes!("ddex_guest");
pub use prover_interface::*;