use patterns::{provisional, source};

pub struct LendingPolicy;
source!(LendingPolicy, "the branch lending policy that fixes the visit cycle");

pub const COLLECT_DAYS: u32 = 7;
provisional!(COLLECT_DAYS, LendingPolicy, "chosen as one visit cycle, but the policy has not confirmed it; the desk supervisor would settle it");
