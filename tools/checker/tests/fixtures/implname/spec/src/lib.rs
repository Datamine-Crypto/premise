#![forbid(unknown_lints)]
pub struct Plan;

pub trait Slots {
    const MAX: u32;
}

impl Slots for Plan {
    const MAX: u32 = 500;
}

pub const Plan_MAX: u32 = 7;
