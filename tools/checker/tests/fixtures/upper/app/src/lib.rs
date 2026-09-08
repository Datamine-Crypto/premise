use std::cmp::max as Max;
use std::cmp::min as Min;

pub fn c1(a: u32, b: u32) -> u32 {
    Max(Min(a, b), b)
}
