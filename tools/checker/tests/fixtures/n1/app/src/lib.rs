#![forbid(unknown_lints)]
use patterns::pick_upper;
use spec::REFUND_WINDOW_DAYS;

pub fn cap(v: u32) -> u32 {
    pick_upper(v, REFUND_WINDOW_DAYS)
}
