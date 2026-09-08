use patterns_macros::because;

pub const FINE_CAP_PENCE: u32 = 500;
because!(FINE_CAP_PENCE, "the most an adult borrower can owe before the charges stop accruing");
