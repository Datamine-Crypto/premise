use patterns::because;

pub const LIMIT: u32 = 42;
because!(LIMIT, "measured ceiling from the load test");
