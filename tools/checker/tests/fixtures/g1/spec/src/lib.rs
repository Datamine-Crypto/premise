#![forbid(unknown_lints)]
use patterns::because;

pub mod aaa;
pub mod zzz;

pub const LIMIT: u32 = 42;
because!(LIMIT, "measured ceiling from the load test");

pub enum Tier {
    Free = 1,
    Team,
    Business,
    Enterprise,
}
because!(Tier, "the four price bands the 2024 pricing page sells");
