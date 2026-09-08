#![forbid(unknown_lints)]
use patterns::because;

pub const LIMIT: u32 = 42;
because!(LIMIT, "measured ceiling from the load test");

pub use toml::Value;
pub use toml::ser::Error;
