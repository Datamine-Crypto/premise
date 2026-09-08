use patterns::because;

pub type Ratio = u32;

pub const SHARE: Ratio = 10 / 3;
because!(SHARE, "the split the 2024 capacity review settled on");
