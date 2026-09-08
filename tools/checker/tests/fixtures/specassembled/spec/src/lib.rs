use patterns::because;

pub const NINE: u32 = 2 * 2 * 2 + 1;
because!(NINE, "a value spelled out as arithmetic on literals rather than derived");
