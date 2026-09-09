use patterns_macros::because;

pub const LIMIT: u32 = 3;
because!(LIMIT, "the opening count at which the wear test showed hinge fatigue");
