use patterns::because;

pub const LIMIT: u32 = 42;
because!(LIMIT, "the first reason");
because!(LIMIT, "a second reason for the same item");
