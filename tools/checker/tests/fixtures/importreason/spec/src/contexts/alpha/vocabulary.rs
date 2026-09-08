use patterns::because;

pub const LOAN_DAYS: u32 = 21;
because!(LOAN_DAYS, "the term the branch lends a book for");
