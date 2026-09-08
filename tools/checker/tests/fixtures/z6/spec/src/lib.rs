#![forbid(unknown_lints)]
use patterns::because;

pub const LIMIT: u32 = 42;
because!(LIMIT, "measured ceiling from the load test");

#[cfg(test)]
mod tests {
    pub const SAMPLE_ROWS: u32 = 7;
}
