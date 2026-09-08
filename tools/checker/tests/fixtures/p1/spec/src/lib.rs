#![forbid(unknown_lints)]
use patterns::{because, fact};

pub const LIMIT: u32 = 42;
because!(LIMIT, "measured ceiling from the load test");

fact!(REFUND_WINDOW_DAYS, 30);
