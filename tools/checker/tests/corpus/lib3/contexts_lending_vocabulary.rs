use crate::contexts::shared::vocabulary::POST_DAYS;

use patterns::{because};
use patterns::supersedes;

pub struct Lending;
because!(Lending, "the borrowing desk the 2026 service review covers");

pub enum Command { Borrow, Renew }
pub enum Event { Borrowed, Renewed }
pub enum Fault { OnLoan, RenewalSpent }

pub const LOAN_DAYS: u32 = 21;
because!(LOAN_DAYS, "the 2026 service review set three weeks after the reading survey");

pub const LEGACY_LOAN_DAYS: u32 = 14;
because!(LEGACY_LOAN_DAYS, "the pre 2026 term, still governing loans taken before the review");
supersedes!(LOAN_DAYS, LEGACY_LOAN_DAYS, "from the 2026 service review");

pub const GRACE_DAYS: u32 = 3;
because!(GRACE_DAYS, "the post office quotes three days for a returned item to reach the desk");



pub static DESK_LABEL: &str = "front desk";
because!(DESK_LABEL, "the 2026 service review named the ground floor counter this on all signage");



pub const RETURN_WINDOW_DAYS: u32 = POST_DAYS + GRACE_DAYS;
