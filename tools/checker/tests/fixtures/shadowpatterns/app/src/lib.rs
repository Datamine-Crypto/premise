use ::patterns::is_below;
use std::cmp as patterns;

pub fn bigger(a: u32, b: u32) -> u32 {
    patterns::max(a, b)
}

pub fn under(a: u32, b: u32) -> bool {
    is_below(a, b)
}
