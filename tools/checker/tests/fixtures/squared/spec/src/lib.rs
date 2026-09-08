use patterns::because;

pub const CAP: u32 = 12;
because!(CAP, "the width of the forecourt in bays as counted on site");
pub const AREA: u32 = CAP * CAP;
