use patterns::because;

pub const ONE_UNIT: u32 = 1;
because!(ONE_UNIT, "the unit every other tariff figure is stated in");
pub const OTHER_UNIT: u32 = 1;
because!(OTHER_UNIT, "the same unit under the name the older tariff used");
pub const THREE: u32 = ONE_UNIT + OTHER_UNIT + ONE_UNIT;
