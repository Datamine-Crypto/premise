use patterns::{because, rejected};

pub struct Hire;
because!(Hire, "the docked hire scheme the 2026 concession covers");

pub enum Command { Take, Return }
pub enum Event { Taken, Returned }
pub enum Fault { NoneFree, NotOnHire }

pub enum Status { Idle, Active, Faulted }
because!(Status, "a hire is faulted when the payment fails, which the dock never observes");

pub const PENCE_PER_MINUTE: u32 = 8;
because!(PENCE_PER_MINUTE, "the March 2026 tariff, set to cover the battery swap contract");
rejected!(PENCE_PER_MINUTE, "the 2024 rate", "it predated the battery swap contract");

pub const LEGACY_PENCE_PER_MINUTE: u32 = 5;
because!(LEGACY_PENCE_PER_MINUTE, "the 2024 tariff, still owed on hires taken before March 2026");

pub const FREE_MINUTES: u32 = 30;
because!(FREE_MINUTES, "the 2026 concession requires a free half hour on every hire");

pub const CAP_MINUTES: u32 = 120;
because!(CAP_MINUTES, "the 2025 usage study found 98 percent of hires end inside two hours");

pub const PAID_MINUTES: u32 = CAP_MINUTES - FREE_MINUTES;
pub const FARE_CAP_PENCE: u32 = PAID_MINUTES * PENCE_PER_MINUTE;
