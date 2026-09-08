#![forbid(unknown_lints)]
use patterns::because;

pub const REFUND_WINDOW_DAYS: u32 = 30;
because!(REFUND_WINDOW_DAYS, "the window the payment processor allows for a chargeback");
