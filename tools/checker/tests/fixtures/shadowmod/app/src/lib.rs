mod patterns {
    pub use std::cmp::max;
}

pub fn shadow_mod(a: u32, b: u32) -> u32 {
    patterns::max(a, b)
}
