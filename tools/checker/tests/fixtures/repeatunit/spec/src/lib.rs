use patterns::because;

pub const ONE_UNIT: u32 = 1;
because!(ONE_UNIT, "the unit every other tariff figure is stated in");
pub const THREE: u32 = ONE_UNIT + ONE_UNIT + ONE_UNIT;
because!(THREE, "the count of units a short stay is charged for");
