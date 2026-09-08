use patterns::because;

#[deprecated(note = "the rate doubled after the spring review and nobody moved it")]
pub const OLD_PENCE: u32 = 8;
because!(OLD_PENCE, "what an overdue day cost before the review repriced it");
