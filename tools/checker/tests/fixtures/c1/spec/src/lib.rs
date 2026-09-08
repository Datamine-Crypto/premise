#![forbid(unknown_lints)]
use patterns::because;

pub const LIMIT: u32 = 42;
because!(LIMIT, "measured ceiling from the load test");

#[cfg(test)]
pub mod ghost;

#[cfg(test)]
pub mod inline {
    use patterns::because;

    pub const SURCHARGE_CENTS: u32 = 995;
    because!(SURCHARGE_CENTS, "the fee the 2024 processor agreement sets per transaction");
}
