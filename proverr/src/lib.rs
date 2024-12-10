pub const DDEX_GUEST_ID: [u32; 8] = [3661642473, 2114894314, 1842113330, 3614622090, 4121478182, 2941001259, 2098619595, 213449828];
pub const DDEX_GUEST_ELF: &[u8] = include_bytes!("ddex_guest");
pub use prover_interface::*;