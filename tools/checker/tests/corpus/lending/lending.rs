use patterns::because;

pub const MAX_LOANS: usize = 5;
because!(MAX_LOANS, "the shelf a member can fill before the desk needs a conversation");

pub const LOAN_DAYS: u32 = 21;
because!(LOAN_DAYS, "three weeks, the period the 2026 lending policy sets for a member");

pub const LOANS_LABEL: &str = "loans";
because!(LOANS_LABEL, "the key the desk terminal reads when it redraws the member panel");

pub const OVERDUE_LABEL: &str = "overdue";
because!(OVERDUE_LABEL, "the key the recall job reads to decide whether to send a notice");

pub const SEPARATOR: &str = " ";
because!(SEPARATOR, "a single space is what the fixed width desk row uses");
