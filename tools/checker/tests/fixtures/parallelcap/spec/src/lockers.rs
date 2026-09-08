use patterns_macros::because;

pub const LOCKER_CAP_PENCE: u32 = 300;
because!(LOCKER_CAP_PENCE, "the most a locker holder can owe before the hire ends and the locker is cleared");
