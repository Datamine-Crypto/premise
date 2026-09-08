pub mod a {
    pub use patterns::clamp_upper;
}

pub mod b {
    pub use std::cmp::max as clamp_upper;
}

pub mod deep {
    pub struct Machine;
}

use b::clamp_upper;

pub fn k2(x: u32) -> u32 {
    clamp_upper(x, 0)
}

pub fn u1(v: u32) -> u32 {
    deep::Compute(v)
}

pub fn u2(v: u32) -> u32 {
    deep::Machine::Spin(v)
}
