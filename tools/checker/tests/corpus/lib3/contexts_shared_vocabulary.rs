use patterns::because;

pub const POST_DAYS: u32 = 3;
because!(POST_DAYS, "the post office quotes three days for an item to reach either site");
