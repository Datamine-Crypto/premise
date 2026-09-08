#![forbid(unknown_lints)]
use patterns::because;

pub const LIMIT: u32 = 42;
because!(LIMIT, "measured ceiling from the load test");

pub struct Plan;

impl Plan {
    pub const SURCHARGE_CENTS: u32 = 995;
}
