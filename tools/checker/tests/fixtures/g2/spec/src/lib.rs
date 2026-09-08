#![forbid(unknown_lints)]
use patterns::because;

pub const LIMIT: u32 = 42;
because!(LIMIT, "measured ceiling from the load test");

pub enum Tier {
    Free = 1,
    Team,
    Business,
}
because!(Tier, "the three price bands the 2024 pricing page sells");
