#![forbid(unknown_lints)]
use patterns::because;

#[allow(clippy::the_rate_doubled_after_the_spring_review_and_nobody_moved_it)]
pub const OLD_PENCE: u32 = 8;
because!(OLD_PENCE, "what an overdue day cost before the desk repriced it");
