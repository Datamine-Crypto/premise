#![forbid(unknown_lints)]
pub struct Plan;

pub trait Slots {
    const MAX: u32;
}

pub trait Limits {
    const CEILING: u32 = 100;
}

pub const A_CONST: u32 = 1;

pub static A_STATIC: u32 = 2;

impl Slots for Plan {
    const MAX: u32 = 3;
}
