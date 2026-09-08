use patterns_macros::because;

pub const LIMIT: u32 = 3;
because!(LIMIT, "three openings is where the wear test showed hinge fatigue");
