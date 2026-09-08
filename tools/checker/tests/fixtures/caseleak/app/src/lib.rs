#![forbid(unknown_lints)]
use patterns::clamp_upper;
use spec::{BLOCK_ONE_LITRES, BLOCK_TWO_LITRES};

pub fn block_one(litres: u32) -> u32 {
    clamp_upper(litres, BLOCK_ONE_LITRES)
}

pub fn block_two(litres: u32) -> u32 {
    clamp_upper(litres, BLOCK_TWO_LITRES)
}
