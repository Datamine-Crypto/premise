use patterns::because;

pub const PENCE: u32 = 15;
because!(PENCE, "what an overdue day costs, set against the cost of chasing it");
pub const ONE_UNIT: u32 = 1;
because!(ONE_UNIT, "the unit every other tariff figure is stated in");
pub const PENCE_AGAIN: u32 = PENCE;
pub const TWO: u32 = PENCE / PENCE_AGAIN + ONE_UNIT;
