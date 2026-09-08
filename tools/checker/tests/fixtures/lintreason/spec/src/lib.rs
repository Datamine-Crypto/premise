use patterns::because;

#[allow(dead_code, reason = "kept because the branch will reinstate the old rate next spring, per the desk")]
pub const OLD_PENCE: u32 = 8;
because!(OLD_PENCE, "what an overdue day cost before the desk repriced it");
