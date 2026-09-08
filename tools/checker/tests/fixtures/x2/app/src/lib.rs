#![forbid(unknown_lints)]
use patterns::pick_upper;
use spec::{LIMIT, SURCHARGE_CENTS};

pub fn cap(v: u32) -> u32 {
    pick_upper(v, SURCHARGE_CENTS)
}
