use patterns::{because, rejected};

pub const NINE: u32 = 9;
because!(NINE, "the count of lives the playtest settled on");
rejected!(NINE, "was 8 until 2024", "changed at the 2024 rebalance");
