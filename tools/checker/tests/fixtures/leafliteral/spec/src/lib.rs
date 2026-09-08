use patterns::because;

pub const ONE_UNIT: u32 = 1;
because!(ONE_UNIT, "the unit every other tariff figure is stated in");
pub const PENCE: u32 = 15 * ONE_UNIT;
