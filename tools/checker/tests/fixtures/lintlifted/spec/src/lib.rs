#![forbid(unknown_lints)]
use patterns::because;

#[allow(unknown_lints, the_rate, doubled_after, the_spring, review_and, nobody_moved_it)]
pub const OLD_PENCE: u32 = 8;
because!(OLD_PENCE, "what an overdue day cost before the desk repriced it");
