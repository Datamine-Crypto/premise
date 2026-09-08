use crate::contexts::shared::vocabulary::VehicleClass;
use patterns::{because, rejected};

pub struct Billing;
because!(Billing, "the pay on foot machine, the only collection point the deck has");

pub enum Command { Charge(VehicleClass), Settle }
pub enum Event { Charged(VehicleClass), Settled }
pub enum Fault { CapReached, NothingOwed }

pub const HOURLY_PENCE: u32 = 350;
because!(HOURLY_PENCE, "the council tariff for a surface bay, set at the March 2024 meeting");
rejected!(HOURLY_PENCE, "the 2019 tariff", "it did not cover the resurfacing loan");

pub const CAP_HOURS: u32 = 8;
because!(CAP_HOURS, "a working day at the office park the deck was built to serve");

pub const DAY_CAP_PENCE: u32 = HOURLY_PENCE * CAP_HOURS;

const _: () = assert!(DAY_CAP_PENCE % HOURLY_PENCE == 0);
